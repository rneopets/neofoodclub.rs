use std::cell::OnceCell;

use crate::arena::Arenas;
use crate::bets::Bets;
use crate::error::NfcError;
use crate::math::{
    make_round_dicts, pirates_binary, RoundDictData, BET_AMOUNT_MAX, BET_AMOUNT_MIN,
};
use crate::modifier::{Modifier, ModifierFlags};
use crate::oddschange::OddsChange;
use crate::round_data::{RoundData, RoundDataRaw};
use chrono::{DateTime, Utc};
use chrono_tz::{OffsetComponents, Tz};
use serde::Serialize;

use crate::models::multinomial_logit::MultinomialLogitModel;
use crate::models::original::OriginalModel;
use crate::pirates::Pirate;

#[derive(Serialize)]
struct UrlAllDataParams<'a> {
    pirates: &'a str,
    #[serde(rename = "openingOdds")]
    opening_odds: &'a str,
    #[serde(rename = "currentOdds")]
    current_odds: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    winners: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timestamp: Option<&'a str>,
}

/// The probability model to use when calculating bets.
#[derive(Debug, Clone, Default)]
pub enum ProbabilityModel {
    #[default]
    OriginalModel,
    MultinomialLogitModel,
}

/// A struct to represent the NeoFoodClub object.
/// This object contains all the data needed to calculate bets,
/// and methods to create URLs.
#[derive(Debug, Clone)]
pub struct NeoFoodClub {
    pub round_data: RoundData,
    pub bet_amount: Option<u32>,
    pub modifier: Modifier,
    pub probability_model: ProbabilityModel,
    arenas: OnceCell<Arenas>,
    stds: OnceCell<[[f64; 5]; 5]>,
    data: OnceCell<RoundDictData>,
    pub(crate) max_ter_indices: OnceCell<Vec<usize>>,
    pub(crate) net_expected_indices: OnceCell<Vec<f64>>,
    pub(crate) clamped_max_bets: OnceCell<Vec<u32>>,
    pub(crate) sorted_odds_indices: OnceCell<Vec<usize>>,
    pub(crate) sorted_probs_indices: OnceCell<Vec<usize>>,
    pub(crate) highest_er_full_bet: OnceCell<u32>,
}

impl NeoFoodClub {
    // constructor stuff
    pub fn new(
        mut round_data: RoundData,
        bet_amount: Option<u32>,
        model: Option<ProbabilityModel>,
        modifier: Option<Modifier>,
    ) -> Result<NeoFoodClub, NfcError> {
        validate_round_data(&round_data)?;

        let use_modifier = modifier.unwrap_or_default();

        use_modifier.apply(&mut round_data)?;

        let mut nfc = NeoFoodClub {
            round_data,
            bet_amount: None,
            modifier: use_modifier,
            probability_model: model.unwrap_or_default(),
            arenas: OnceCell::new(),
            stds: OnceCell::new(),
            data: OnceCell::new(),
            max_ter_indices: OnceCell::new(),
            net_expected_indices: OnceCell::new(),
            clamped_max_bets: OnceCell::new(),
            sorted_odds_indices: OnceCell::new(),
            sorted_probs_indices: OnceCell::new(),
            highest_er_full_bet: OnceCell::new(),
        };

        nfc.set_bet_amount(bet_amount);

        Ok(nfc)
    }

    /// Sets the bet amount
    pub fn set_bet_amount(&mut self, amount: Option<u32>) {
        self.bet_amount = amount.map(|x| x.clamp(BET_AMOUNT_MIN, BET_AMOUNT_MAX));
        self.clear_ranking_caches();
    }

    /// Lazy loads the Arenas object.
    #[inline]
    pub fn get_arenas(&self) -> &Arenas {
        self.arenas.get_or_init(|| Arenas::new(&self.round_data))
    }

    /// Lazy loads the probabilities.
    #[inline]
    pub fn probabilities(&self) -> [[f64; 5]; 5] {
        *self.stds.get_or_init(|| match self.probability_model {
            ProbabilityModel::OriginalModel => OriginalModel::new(&self.round_data),
            ProbabilityModel::MultinomialLogitModel => {
                MultinomialLogitModel::new(self.get_arenas())
            }
        })
    }

    /// Lazy loads the RoundDictData object.
    #[inline]
    pub fn round_dict_data(&self) -> &RoundDictData {
        self.data
            .get_or_init(|| make_round_dicts(self.probabilities(), self.custom_odds()))
    }

    /// Clear our lazy-loaded caches.
    pub fn clear_caches(&mut self) {
        self.arenas = OnceCell::new();
        self.stds = OnceCell::new();
        self.data = OnceCell::new();
        self.clear_ranking_caches();
        self.sorted_odds_indices = OnceCell::new();
        self.sorted_probs_indices = OnceCell::new();
        self.round_data.custom_odds = None;
    }

    /// Clear every lazy-loaded cache that depends on odds/time, but leave
    /// `stds` (the probabilities cache) intact, since the probability models
    /// only depend on opening odds (fixed at round creation) and food data,
    /// neither of which `with_modifier` ever changes.
    fn clear_odds_dependent_caches(&mut self) {
        self.arenas = OnceCell::new();
        self.data = OnceCell::new();
        self.clear_ranking_caches();
        self.sorted_odds_indices = OnceCell::new();
        self.sorted_probs_indices = OnceCell::new();
        self.round_data.custom_odds = None;
    }

    /// Clears the caches whose values depend on bet_amount and bet ordering
    /// (max-TER indices, net-expected indices, clamped max bets, and the
    /// highest-ER full-arena bet, which is derived from max-TER indices).
    fn clear_ranking_caches(&mut self) {
        self.clamped_max_bets = OnceCell::new();
        self.net_expected_indices = OnceCell::new();
        self.max_ter_indices = OnceCell::new();
        self.highest_er_full_bet = OnceCell::new();
    }

    /// changes the modifier of this NeoFoodClub object
    /// if the modifier is different enough, we clear the caches
    pub fn with_modifier(&mut self, modifier: Modifier) -> Result<&mut Self, NfcError> {
        let current_modifier = &self.modifier;

        if self.modified()
            || (current_modifier.custom_odds != modifier.custom_odds
                || current_modifier.custom_time != modifier.custom_time
                || current_modifier.is_opening_odds() != modifier.is_opening_odds())
        {
            self.clear_odds_dependent_caches();
        } else if current_modifier.is_general() != modifier.is_general()
            || current_modifier.is_reverse() != modifier.is_reverse()
        {
            // The GENERAL flag changes which ER values are used for sorting, and
            // REVERSE changes sort direction. Neither invalidates the round data
            // cache, but both invalidate the derived index caches.
            self.clear_ranking_caches();
        }

        self.round_data.custom_odds = None;

        self.modifier = modifier;
        self.modifier.apply(&mut self.round_data)?;
        Ok(self)
    }

    /// Creates a NeoFoodClub object from a JSON string.
    /// This is generally the entrypoint for creating a NeoFoodClub object.
    pub fn from_json(
        json: &str,
        bet_amount: Option<u32>,
        model: Option<ProbabilityModel>,
        modifier: Option<Modifier>,
    ) -> Result<NeoFoodClub, NfcError> {
        let round_data: RoundData = serde_json::from_str(json)?;
        NeoFoodClub::new(round_data, bet_amount, model, modifier)
    }

    /// Creates a NeoFoodClub object from a NeoFoodClub-like URL.
    pub fn from_url(
        url: &str,
        bet_amount: Option<u32>,
        model: Option<ProbabilityModel>,
        modifier: Option<Modifier>,
    ) -> Result<NeoFoodClub, NfcError> {
        let (base, query) = url.split_once('#').ok_or(NfcError::InvalidUrl)?;

        let use_modifier = modifier.unwrap_or_default();
        let cc_perk = base.ends_with("/15/") || use_modifier.is_charity_corner();
        let new_modifier = Modifier::new(
            use_modifier.value
                | if cc_perk {
                    ModifierFlags::CHARITY_CORNER.bits()
                } else {
                    0
                },
            use_modifier.custom_odds,
            use_modifier.custom_time,
        )?;

        let temp: RoundDataRaw =
            serde_qs::from_str(query).map_err(|e| NfcError::QueryString(e.to_string()))?;

        let round_data = RoundData {
            foods: temp.foods.map(|x| serde_json::from_str(&x)).transpose()?,
            round: temp.round,
            start: temp.start,
            pirates: serde_json::from_str(&temp.pirates)?,
            opening_odds: serde_json::from_str(&temp.opening_odds)?,
            current_odds: serde_json::from_str(&temp.current_odds)?,
            custom_odds: None,
            winners: temp.winners.map(|x| serde_json::from_str(&x)).transpose()?,
            timestamp: temp.timestamp,
            changes: None,
            last_change: temp.last_change,
        };

        NeoFoodClub::new(round_data, bet_amount, model, Some(new_modifier))
    }
}

impl NeoFoodClub {
    // winner-related stuff

    /// Returns the indices of the winning pirates, if any.
    /// If there are no winners, returns a [0; 5] vector.
    pub fn winners(&self) -> [u8; 5] {
        match &self.round_data.winners {
            Some(winners) => *winners,
            None => [0; 5],
        }
    }

    /// Returns the binary representation of the winning pirates.
    /// Zero means no pirates won yet.
    pub fn winners_binary(&self) -> u32 {
        pirates_binary(self.winners())
    }

    /// Returns a vector of the winning pirates, if any.
    pub fn winning_pirates(&self) -> Option<Vec<Pirate>> {
        let bin = self.winners_binary();

        if bin == 0 {
            return None;
        }

        Some(self.get_arenas().get_pirates_from_binary(bin))
    }

    /// Returns whether or not the round is over.
    /// A round is over if there are winners.
    pub fn is_over(&self) -> bool {
        if self.round_data.winners.is_none() {
            return false;
        }
        self.winners()[0] != 0
    }
}

impl NeoFoodClub {
    // getters from round_data

    /// Returns the round number.
    pub fn round(&self) -> u16 {
        self.round_data.round
    }

    /// Returns the start time of the round in ISO 8601 format as a string.
    /// If the start time is not available, returns None.
    pub fn start(&self) -> &Option<String> {
        &self.round_data.start
    }

    /// Returns the start time of the round in NST.
    /// If the start time is not available, returns None.
    pub fn start_nst(&self) -> Option<DateTime<Tz>> {
        self.round_data.start_nst()
    }

    /// Returns the start time of the round in UTC.
    /// If the start time is not available, returns None.
    pub fn start_utc(&self) -> Option<DateTime<Utc>> {
        self.round_data.start_utc()
    }

    /// Returns the current odds.
    pub fn current_odds(&self) -> &[[u8; 5]; 5] {
        &self.round_data.current_odds
    }

    /// Returns the custom odds.
    /// If the custom odds are not available, returns the current odds.
    /// Custom odds is just the resolved changes of a Modifier.
    /// Effectively, this is what we use for calculations.
    pub fn custom_odds(&self) -> [[u8; 5]; 5] {
        self.round_data.custom_odds.unwrap_or(*self.current_odds())
    }

    /// Returns the opening odds.
    pub fn opening_odds(&self) -> [[u8; 5]; 5] {
        self.round_data.opening_odds
    }

    /// Returns the timestamp of the round in ISO 8601 format as a string.
    pub fn timestamp(&self) -> &Option<String> {
        &self.round_data.timestamp
    }

    /// Returns the timestamp of the round in NST.
    /// If the timestamp is not available, returns None.
    pub fn timestamp_nst(&self) -> Option<DateTime<Tz>> {
        self.round_data.timestamp_nst()
    }

    /// Returns the timestamp of the round in UTC.
    /// If the timestamp is not available, returns None.
    pub fn timestamp_utc(&self) -> Option<DateTime<Utc>> {
        self.round_data.timestamp_utc()
    }

    /// Returns the pirate IDs, as a 2D array.
    /// The first dimension is the arena index, and the second dimension is the pirate index.
    pub fn pirates(&self) -> [[u8; 4]; 5] {
        self.round_data.pirates
    }

    /// Returns the changes of the round.
    pub fn changes(&self) -> &Option<Vec<OddsChange>> {
        &self.round_data.changes
    }

    /// Returns the last change of the round in ISO 8601 format as a string.
    /// If the last change is not available, returns None.
    pub fn last_change(&self) -> &Option<String> {
        &self.round_data.last_change
    }

    /// Returns the last change of the round in NST.
    /// If the last change is not available, returns None.
    pub fn last_change_nst(&self) -> Option<DateTime<Tz>> {
        self.round_data.last_change_nst()
    }

    /// Returns the last change of the round in UTC.
    /// If the last change is not available, returns None.
    pub fn last_change_utc(&self) -> Option<DateTime<Utc>> {
        self.round_data.last_change_utc()
    }

    /// Returns the foods of the round.
    /// If the foods are not available, returns None.
    pub fn foods(&self) -> Option<[[u8; 10]; 5]> {
        self.round_data.foods
    }

    /// Returns whether or not the modifier has made changes to the round data.
    /// We use this to determine if we need to recalculate everything
    /// between
    pub fn modified(&self) -> bool {
        self.custom_odds() != *self.current_odds()
    }

    /// Returns whether or not the round is outdated.
    pub fn is_outdated_lock(&self) -> bool {
        let Some(start_date) = self.start_utc() else {
            return true;
        };

        let day_after = start_date
            .checked_add_signed(chrono::Duration::try_days(1).unwrap())
            .unwrap();

        // Calculate DST offset difference between start_date and day_after
        // by comparing their NST offsets
        let start_nst = crate::utils::convert_from_utc_to_nst(start_date);
        let day_after_nst = crate::utils::convert_from_utc_to_nst(day_after);

        let start_offset = start_nst.offset().dst_offset();
        let day_after_offset = day_after_nst.offset().dst_offset();

        let difference = day_after_offset - start_offset;

        let now = chrono::Utc::now();

        !(start_date <= now && now <= day_after + difference)
    }

    /// Serialize the round data to JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.round_data).expect("Failed to serialize to JSON.")
    }
}

impl NeoFoodClub {
    // URL-related stuff

    /// Creates a URL for the given bets.
    pub fn make_url(&self, bets: Option<&Bets>, include_domain: bool, all_data: bool) -> String {
        let mut url = String::new();

        if include_domain {
            url.push_str("https://neofood.club");
        }

        let use_15 = self.modifier.is_charity_corner() || bets.is_some_and(|b| b.len() > 10);
        if use_15 {
            url.push_str("/15");
        }

        url.push_str(&format!("/#round={}", self.round()));

        if let Some(bets) = bets {
            url.push_str(&format!("&b={}", bets.bets_hash()));

            if let Some(amounts_hash) = bets.amounts_hash() {
                url.push_str(&format!("&a={amounts_hash}"));
            }
        }

        if all_data {
            let pirates = serde_json::to_string(&self.round_data.pirates)
                .expect("Failed to serialize pirates.");
            let opening_odds = serde_json::to_string(&self.round_data.opening_odds)
                .expect("Failed to serialize openingOdds.");
            let current_odds = serde_json::to_string(&self.round_data.current_odds)
                .expect("Failed to serialize currentOdds.");
            let winners = if self.is_over() {
                Some(serde_json::to_string(&self.winners()).expect("Failed to serialize winners."))
            } else {
                None
            };

            let params = UrlAllDataParams {
                pirates: &pirates,
                opening_odds: &opening_odds,
                current_odds: &current_odds,
                winners: winners.as_deref(),
                timestamp: self.timestamp().as_ref().map(|s| s.as_str()),
            };

            let qs = serde_qs::to_string(&params).expect("Failed to serialize URL query string.");
            url.push('&');
            url.push_str(&qs);
        }

        url
    }

    /// Creates a deep copy of the NeoFoodClub object.
    /// If `model` is None, the model is going to use the default.
    /// If `modifier` is None, the modifier is going to be empty.
    pub fn copy(&self, model: Option<ProbabilityModel>, modifier: Option<Modifier>) -> NeoFoodClub {
        let mut round_data = self.round_data.clone();
        round_data.custom_odds = None;
        NeoFoodClub::new(round_data, self.bet_amount, model, modifier)
            .expect("copy of already-validated NeoFoodClub produced invalid data")
    }
}

fn validate_round_data(round_data: &RoundData) -> Result<(), NfcError> {
    if round_data.round == 0 {
        return Err(NfcError::RoundData(
            "Round number must be greater than 0.".to_string(),
        ));
    }

    let mut pirate_ids = Vec::<u8>::with_capacity(20);

    for arena in round_data.pirates.iter() {
        for pirate in arena.iter() {
            if pirate_ids.contains(pirate) {
                return Err(NfcError::RoundData("Pirates must be unique.".to_string()));
            }
            if !(&1..=&20).contains(&pirate) {
                return Err(NfcError::RoundData(
                    "Pirate IDs must be between 1 and 20.".to_string(),
                ));
            }
            pirate_ids.push(*pirate);
        }
    }

    validate_odds_arena(&round_data.current_odds)?;
    validate_odds_arena(&round_data.opening_odds)?;

    if let Some(start) = &round_data.start {
        crate::round_data::validate_timestamp(start)?;
    }
    if let Some(timestamp) = &round_data.timestamp {
        crate::round_data::validate_timestamp(timestamp)?;
    }
    if let Some(last_change) = &round_data.last_change {
        crate::round_data::validate_timestamp(last_change)?;
    }
    if let Some(changes) = &round_data.changes {
        for change in changes.iter() {
            crate::round_data::validate_timestamp(&change.t)?;
        }
    }

    if let Some(foods) = &round_data.foods {
        for arena in foods.iter() {
            for food in arena.iter() {
                if *food < 1 || *food > 40 {
                    return Err(NfcError::RoundData(
                        "Food integers must be between 1 and 40.".to_string(),
                    ));
                }
            }
        }
    }

    if let Some(winners) = &round_data.winners {
        let all_zero = winners.iter().all(|&x| x == 0);
        let all_one_to_four = winners.iter().all(|&x| (1..=4).contains(&x));

        if !(all_zero ^ all_one_to_four) {
            return Err(NfcError::RoundData(
                "Winners must either be all 0, or all 1-4.".to_string(),
            ));
        }
    }

    Ok(())
}

fn validate_odds_arena(odds: &[[u8; 5]; 5]) -> Result<(), NfcError> {
    for arena in odds.iter() {
        for (index, odd) in arena.iter().enumerate() {
            if index == 0 {
                if *odd != 1 {
                    return Err(NfcError::RoundData(
                        "First integer in each arena in odds must be 1.".to_string(),
                    ));
                }
            } else if *odd < 2 || *odd > 13 {
                return Err(NfcError::RoundData(
                    "Odds must be between 2 and 13.".to_string(),
                ));
            }
        }
    }

    Ok(())
}
