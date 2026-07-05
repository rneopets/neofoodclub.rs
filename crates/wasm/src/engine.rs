use std::collections::HashMap;

use neofoodclub::bets::Bets;
use neofoodclub::modifier::Modifier;
use neofoodclub::nfc::{NeoFoodClub, ProbabilityModel};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct BetsOut {
    indices: Vec<[u8; 5]>,
    amounts: Option<Vec<Option<u32>>>,
    #[serde(rename = "betsHash")]
    bets_hash: String,
    #[serde(rename = "amountsHash")]
    amounts_hash: Option<String>,
}

impl From<Bets> for BetsOut {
    fn from(bets: Bets) -> Self {
        BetsOut {
            indices: bets.get_indices(),
            amounts: bets.bet_amounts.clone(),
            bets_hash: bets.bets_hash(),
            amounts_hash: bets.amounts_hash(),
        }
    }
}

fn to_js(bets: Bets) -> Result<JsValue, JsError> {
    serde_wasm_bindgen::to_value(&BetsOut::from(bets)).map_err(|e| JsError::new(&e.to_string()))
}

/// A stateful wrapper around `NeoFoodClub`, meant to be constructed once per
/// round-data update and reused across multiple bet-generation calls, so
/// Rust's own caching (the 3124-combo table, max-TER rankings, etc.) isn't
/// redone on every call.
#[wasm_bindgen]
pub struct NfcEngine {
    inner: NeoFoodClub,
}

#[wasm_bindgen]
impl NfcEngine {
    /// Constructs a new engine from raw round JSON (the same camelCase shape
    /// already used by `computeStdProbabilities`). `bet_amount` should
    /// already be clamped by the caller (e.g. TS's `getMaxBet()`) before
    /// being passed in.
    #[wasm_bindgen(constructor)]
    pub fn new(
        round_json: &str,
        bet_amount: Option<u32>,
        use_logit: bool,
    ) -> Result<NfcEngine, JsError> {
        let model = if use_logit {
            ProbabilityModel::MultinomialLogitModel
        } else {
            ProbabilityModel::OriginalModel
        };
        let inner = NeoFoodClub::from_json(round_json, bet_amount, Some(model), None)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(NfcEngine { inner })
    }

    /// Re-seats the bet amount (max bet) without rebuilding the whole engine;
    /// correctly invalidates just the ranking caches that depend on it.
    #[wasm_bindgen(js_name = setBetAmount)]
    pub fn set_bet_amount(&mut self, amount: Option<u32>) {
        self.inner.set_bet_amount(amount);
    }

    /// Overrides odds for specific pirates (by global pirate ID, derived from
    /// `odds_grid` + this engine's own round data). `odds_grid` is a
    /// flattened 25-element array (5 arenas x [unused, o1..o4]), matching the
    /// app's `OddsData` shape - already resolved to whatever the caller wants
    /// to treat as "current" odds.
    #[wasm_bindgen(js_name = setCustomOdds)]
    pub fn set_custom_odds(&mut self, odds_grid: Vec<u8>) -> Result<(), JsError> {
        if odds_grid.len() != 25 {
            return Err(JsError::new(
                "odds_grid must have exactly 25 elements (5 arenas x 5 slots)",
            ));
        }
        let mut map: HashMap<u8, u8> = HashMap::new();
        for arena in 0..5 {
            for pirate_idx in 1..5 {
                let pirate_id = self.inner.round_data.pirates[arena][pirate_idx - 1];
                let odds = odds_grid[arena * 5 + pirate_idx];
                map.insert(pirate_id, odds);
            }
        }
        let modifier = Modifier::new(
            self.inner.modifier.value,
            Some(map),
            self.inner.modifier.custom_time,
        )
        .map_err(|e| JsError::new(&e.to_string()))?;
        self.inner
            .with_modifier(modifier)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    }

    /// Clears any custom-odds override set via `setCustomOdds`.
    #[wasm_bindgen(js_name = clearCustomOdds)]
    pub fn clear_custom_odds(&mut self) -> Result<(), JsError> {
        let modifier = Modifier::new(
            self.inner.modifier.value,
            None,
            self.inner.modifier.custom_time,
        )
        .map_err(|e| JsError::new(&e.to_string()))?;
        self.inner
            .with_modifier(modifier)
            .map_err(|e| JsError::new(&e.to_string()))?;
        Ok(())
    }

    /// Overrides win probabilities with caller-supplied values, bypassing the
    /// probability model entirely. `probs_grid` is a flattened 25-element
    /// array (5 arenas x [unused, p1..p4]).
    #[wasm_bindgen(js_name = setCustomProbabilities)]
    pub fn set_custom_probabilities(&mut self, probs_grid: Vec<f64>) -> Result<(), JsError> {
        if probs_grid.len() != 25 {
            return Err(JsError::new(
                "probs_grid must have exactly 25 elements (5 arenas x 5 slots)",
            ));
        }
        let mut probs = [[0.0; 5]; 5];
        for arena in 0..5 {
            for pirate in 0..5 {
                probs[arena][pirate] = probs_grid[arena * 5 + pirate];
            }
        }
        self.inner.set_custom_probabilities(Some(probs));
        Ok(())
    }

    /// Clears any custom-probabilities override set via `setCustomProbabilities`.
    #[wasm_bindgen(js_name = clearCustomProbabilities)]
    pub fn clear_custom_probabilities(&mut self) {
        self.inner.set_custom_probabilities(None);
    }

    #[wasm_bindgen(js_name = makeMaxTerBets)]
    pub fn make_max_ter_bets(&self) -> Result<JsValue, JsError> {
        to_js(self.inner.make_max_ter_bets())
    }

    #[wasm_bindgen(js_name = makeBestGambitBets)]
    pub fn make_best_gambit_bets(&self) -> Result<JsValue, JsError> {
        to_js(self.inner.make_best_gambit_bets())
    }

    /// `pirates_binary` must select exactly 5 pirates (one per arena) - the
    /// core `make_gambit_bets` hard-asserts this, so we validate here first
    /// rather than ever reaching that assert through the wasm boundary.
    #[wasm_bindgen(js_name = makeGambitBets)]
    pub fn make_gambit_bets(&self, pirates_binary: u32) -> Result<JsValue, JsError> {
        if pirates_binary.count_ones() != 5 {
            return Err(JsError::new(
                "pirates_binary must select exactly 5 pirates (one per arena)",
            ));
        }
        to_js(self.inner.make_gambit_bets(pirates_binary))
    }

    /// Returns `undefined` (via `null` -> caller should treat as no result)
    /// when the round has no winners yet.
    #[wasm_bindgen(js_name = makeWinningGambitBets)]
    pub fn make_winning_gambit_bets(&self) -> Result<Option<JsValue>, JsError> {
        self.inner.make_winning_gambit_bets().map(to_js).transpose()
    }

    /// Returns `null` when no arena is positive (bustproof isn't possible).
    #[wasm_bindgen(js_name = makeBustproofBets)]
    pub fn make_bustproof_bets(&self) -> Result<Option<JsValue>, JsError> {
        self.inner.make_bustproof_bets().map(to_js).transpose()
    }

    #[wasm_bindgen(js_name = makeCrazyBets)]
    pub fn make_crazy_bets(&self) -> Result<JsValue, JsError> {
        to_js(self.inner.make_crazy_bets())
    }

    /// Errors if `pirates_binary` selects more than 1 pirate per arena, or
    /// 0/more-than-3 pirates total.
    #[wasm_bindgen(js_name = makeTenbetBets)]
    pub fn make_tenbet_bets(&self, pirates_binary: u32) -> Result<JsValue, JsError> {
        let bets = self
            .inner
            .make_tenbet_bets(pirates_binary)
            .map_err(|e| JsError::new(&e.to_string()))?;
        to_js(bets)
    }
}
