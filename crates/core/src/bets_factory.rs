use std::collections::HashSet;

use itertools::Itertools;
use rand::seq::IteratorRandom;

use crate::bets::Bets;
use crate::error::NfcError;
use crate::math::{random_full_pirates_binary, BET_AMOUNT_MIN, BIT_MASKS};
use crate::nfc::NeoFoodClub;
use crate::utils::argsort_slice_3124;

impl NeoFoodClub {
    // Bets indices related stuff

    /// Returns the maximum amount of bets you can place.
    /// Normally, this would be 10.
    /// If the modifier has the charity corner perk, this is 15.
    pub fn max_amount_of_bets(&self) -> usize {
        if self.modifier.is_charity_corner() {
            15
        } else {
            10
        }
    }

    /// Returns the maximum TER values we'll use.
    #[inline]
    pub fn max_ters(&self) -> &Vec<f64> {
        let general = self.modifier.is_general();
        let data = self.round_dict_data();

        if let Some(bet_amount) = self.bet_amount {
            if general {
                return &data.ers;
            }

            // if there's a bet amount, we use Net Expected instead of Expected Return
            let maxbets: &Vec<u32> = self.clamped_max_bets.get_or_init(|| {
                data.maxbets
                    .iter()
                    .map(|&x| x.max(BET_AMOUNT_MIN).min(bet_amount))
                    .collect()
            });

            let new_ers: &Vec<f64> = self.net_expected_indices.get_or_init(|| {
                maxbets
                    .iter()
                    .zip(data.ers.iter())
                    .map(|(maxbet, er)| {
                        let mb = *maxbet as f64;
                        mb * er - mb
                    })
                    .collect()
            });
            new_ers
        } else {
            &data.ers
        }
    }

    /// Returns max-TER indices.
    pub(crate) fn max_ter_indices(&self) -> &[usize] {
        self.max_ter_indices.get_or_init(|| {
            let use_ers = self.max_ters();

            let mut indices = argsort_slice_3124(use_ers, |a: &f64, b: &f64| a.total_cmp(b));

            let reverse = self.modifier.is_reverse();
            // since it's reversed to begin with, we reverse it if
            // the modifier does not have the reverse flag
            if !reverse {
                indices.reverse();
            }

            indices
        })
    }

    /// Returns sorted indices of odds, highest to lowest.
    pub(crate) fn get_sorted_odds_indices(&self) -> &[usize] {
        self.sorted_odds_indices.get_or_init(|| {
            let data = self.round_dict_data();
            let mut indices = argsort_slice_3124(&data.odds, |a: &u32, b: &u32| a.cmp(b));
            indices.reverse();
            indices
        })
    }

    /// Returns sorted indices of probabilities, highest to lowest.
    pub(crate) fn get_sorted_probs_indices(&self) -> &[usize] {
        self.sorted_probs_indices.get_or_init(|| {
            let data = self.round_dict_data();
            let mut indices = argsort_slice_3124(&data.probs, |a: &f64, b: &f64| a.total_cmp(b));
            indices.reverse();
            indices
        })
    }

    /// Return the binary representation of the highest expected return full-arena bet.
    pub(crate) fn get_highest_er_full_bet(&self) -> u32 {
        *self.highest_er_full_bet.get_or_init(|| {
            let data = self.round_dict_data();

            let index = self
                .max_ter_indices()
                .iter()
                .copied()
                .find(|&index| data.bins[index].count_ones() == 5)
                .expect("round data always contains at least one full-arena bet");

            data.bins[index]
        })
    }
}

impl NeoFoodClub {
    // bets-related stuff

    /// Creates a Bets object that consists of all bets.
    /// This is mostly for debugging purposes.
    pub fn make_all_bets(&self) -> Bets {
        Bets::new(self, (0..3124).collect_vec())
    }

    /// Creates a Bets object that consists of all max-TER bets.
    /// This is mostly for debugging purposes.
    pub fn make_all_max_ter_bets(&self) -> Bets {
        let indices = self.max_ter_indices();

        let mut bets = Bets::new(self, indices.to_vec());
        bets.fill_bet_amounts(self);
        bets
    }

    /// Creates a Bets object that consists of the highest ER bets that
    /// are greater than or equal to the given units.
    pub fn make_units_bets(&self, units: u32) -> Option<Bets> {
        let sorted_probs = self.get_sorted_probs_indices();
        let data = self.round_dict_data();

        let count = self.max_amount_of_bets();
        let mut units_indices = Vec::<usize>::with_capacity(count);

        for index in sorted_probs.iter() {
            if data.odds[*index] >= units {
                units_indices.push(*index);
                if units_indices.len() == count {
                    break;
                }
            }
        }

        if units_indices.is_empty() {
            return None;
        }

        let mut bets = Bets::new(self, units_indices);

        bets.fill_bet_amounts(self);

        Some(bets)
    }

    /// Creates a Bets object that consists of random bets.
    /// Following these bets is not recommended.
    pub fn make_random_bets(&self) -> Bets {
        let mut rng = rand::rng();

        let chosen_values: Vec<usize> = (0..3124).sample(&mut rng, self.max_amount_of_bets());

        let mut bets = Bets::new(self, chosen_values);
        bets.fill_bet_amounts(self);
        bets
    }

    /// Creates a Bets object that consists of max-TER bets.
    pub fn make_max_ter_bets(&self) -> Bets {
        let indices = self
            .max_ter_indices()
            .iter()
            .take(self.max_amount_of_bets())
            .cloned()
            .collect();

        let mut bets = Bets::new(self, indices);
        bets.fill_bet_amounts(self);
        bets
    }

    /// Creates a Bets object that consists of a gambit of the given 5-bet pirates binary.
    pub fn make_gambit_bets(&self, pirates_binary: u32) -> Bets {
        assert_eq!(
            pirates_binary.count_ones(),
            5,
            "Pirates binary must have 5 pirates."
        );

        // get indices of all bets that contain the pirates in the pirates_binary
        let data = self.round_dict_data();
        let bins = &data.bins;
        let indices = self
            .get_sorted_odds_indices()
            .iter()
            .copied()
            .filter(|&index| bins[index] & pirates_binary == bins[index])
            .take(self.max_amount_of_bets())
            .collect();

        let mut bets = Bets::new(self, indices);
        bets.fill_bet_amounts(self);
        bets
    }

    /// Creates a Bets object that consists of the best gambit bets.
    /// Basically just gambit bets with the highest expected return.
    pub fn make_best_gambit_bets(&self) -> Bets {
        let max_ter_pirates = self.get_highest_er_full_bet();

        self.make_gambit_bets(max_ter_pirates)
    }

    /// Creates a Bets object that consists of winning gambit bets.
    /// Pretty much the best bets you can make for a given round.
    pub fn make_winning_gambit_bets(&self) -> Option<Bets> {
        let winners_binary = self.winners_binary();

        match winners_binary {
            0 => None,
            _ => Some(self.make_gambit_bets(winners_binary)),
        }
    }

    /// Picks a random full-arena bet and makes a gambit out of it
    pub fn make_random_gambit_bets(&self) -> Bets {
        self.make_gambit_bets(random_full_pirates_binary())
    }

    /// Creates a Bets object that consits of "crazy" bets.
    /// Crazy bets consist of randomly-selected, full-arena bets.
    /// Following these bets is not recommended.
    pub fn make_crazy_bets(&self) -> Bets {
        let count = self.max_amount_of_bets();
        let mut binaries: HashSet<u32> = HashSet::with_capacity(count);

        while binaries.len() < count {
            binaries.insert(random_full_pirates_binary());
        }

        let mut bets = Bets::from_binaries(self, binaries.into_iter().collect());
        bets.fill_bet_amounts(self);
        bets
    }

    /// Creates a Bets object that consists of bustproof bets.
    /// Returns None if there are no positive arenas.
    pub fn make_bustproof_bets(&self) -> Option<Bets> {
        let positives = self.get_arenas().positives();

        let bets = match positives.len() {
            0 => None,
            1 => {
                // If only one arena is positive, we place 1 bet on each of the pirates of that arena. Total bets = 4.
                let best_arena = &positives[0];

                let binaries: Vec<u32> = best_arena
                    .pirates
                    .iter()
                    .map(|pirate| pirate.binary())
                    .collect();

                Some(Bets::from_binaries(self, binaries))
            }
            2 => {
                // If two arenas are positive, we place 1 bet on each of the three worst pirates of the best arena and
                // 1 bet on each of the pirates of the second arena + the best pirate of the best arena. Total bets = 7
                let (best_arena, second_best_arena) = (&positives[0], &positives[1]);

                let best_in_best_arena = best_arena.best();

                let best_pirate_binary = best_in_best_arena[0].binary();

                let binaries: Vec<u32> = best_in_best_arena[1..]
                    .iter()
                    .map(|pirate| pirate.binary())
                    .chain(
                        second_best_arena
                            .pirates
                            .iter()
                            .map(|pirate| pirate.binary() | best_pirate_binary),
                    )
                    .collect();

                Some(Bets::from_binaries(self, binaries))
            }
            3..=5 => {
                //  If three arenas are positive, we place 1 bet on each of the three worst pirates of the best arena,
                //  If four or more arenas are positive, we only play the three best arenas, seen below
                //  1 bet on each of the three worst pirates of the second arena + the best pirate of the best arena,
                //  and 1 bet on each of the pirates of the third arena + the best pirate of the best arena + the best pirate
                //  of the second arena. Total bets = 10.

                let (best_arena, second_best_arena, third_best_arena) =
                    (&positives[0], &positives[1], &positives[2]);

                let best_in_best_arena = best_arena.best();
                let best_in_second_best_arena = second_best_arena.best();

                let best_pirate_binary = best_in_best_arena[0].binary();
                let second_best_pirate_binary = best_in_second_best_arena[0].binary();

                let binaries: Vec<u32> = best_in_best_arena[1..]
                    .iter()
                    .map(|pirate| pirate.binary())
                    .chain(
                        best_in_second_best_arena[1..]
                            .iter()
                            .map(|pirate| pirate.binary() | best_pirate_binary),
                    )
                    .chain(third_best_arena.pirates.iter().map(|pirate| {
                        pirate.binary() | best_pirate_binary | second_best_pirate_binary
                    }))
                    .collect();

                Some(Bets::from_binaries(self, binaries))
            }
            _ => unreachable!("This should never happen."),
        };

        // give it bet amounts
        if let Some(mut bets) = bets {
            if let Some(amount) = self.bet_amount {
                let odds = bets.odds_values(self);
                let lowest = odds.iter().min().expect("Odds vector is empty, somehow");

                let bet_amounts: Vec<Option<u32>> =
                    odds.iter().map(|odd| Some(amount * lowest / odd)).collect();

                bets.bet_amounts = Some(bet_amounts);
            }

            return Some(bets);
        }

        None
    }

    /// Creates a Bets object that consists of 10-bets on the selected pirates.
    /// Returns an error if the pirates binary is invalid.
    /// Returns an error if the amount of pirates is invalid.
    /// Returns an error if the amount of pirates is greater than 3.
    /// Returns an error if the amount of pirates is less than 1.
    pub fn make_tenbet_bets(&self, pirates_binary: u32) -> Result<Bets, NfcError> {
        for mask in BIT_MASKS.iter() {
            if (pirates_binary & mask).count_ones() > 1 {
                return Err(NfcError::InvalidBet(
                    "You can only pick 1 pirate per arena.".to_string(),
                ));
            }
        }

        let amount_of_pirates: u32 = BIT_MASKS
            .iter()
            .map(|mask| (pirates_binary & mask).count_ones())
            .sum();

        match amount_of_pirates {
            0 => {
                return Err(NfcError::InvalidBet(
                    "You must pick at least 1 pirate, and at most 3.".to_string(),
                ))
            }
            1..=3 => (),
            _ => {
                return Err(NfcError::InvalidBet(
                    "You must pick 3 pirates at most.".to_string(),
                ))
            }
        }

        let data = self.round_dict_data();
        let bins = self
            .max_ter_indices()
            .iter()
            .map(|&index| data.bins[index])
            .filter(|&bin| bin & pirates_binary == pirates_binary)
            .take(self.max_amount_of_bets())
            .collect();

        let mut bets = Bets::from_binaries(self, bins);

        bets.fill_bet_amounts(self);

        Ok(bets)
    }

    /// Creates a Bets object translated from a bets hash.
    pub fn make_bets_from_hash(&self, hash: &str) -> Result<Bets, NfcError> {
        let mut bets = Bets::from_hash(self, hash)?;

        bets.fill_bet_amounts(self);

        Ok(bets)
    }

    /// Creates a Bets object translated from a bets binary vector.
    pub fn make_bets_from_binaries(&self, binaries: Vec<u32>) -> Bets {
        let mut bets = Bets::from_binaries(self, binaries);

        bets.fill_bet_amounts(self);

        bets
    }

    /// Creates a Bets object translated from a bets indices vector.
    pub fn make_bets_from_indices(&self, indices: Vec<[u8; 5]>) -> Bets {
        let mut bets = Bets::from_indices(self, indices);

        bets.fill_bet_amounts(self);

        bets
    }

    /// Creates a Bets object from a vector of indices.
    /// Unlike the other usages of indices, this one uses the index of our RoundData struct.
    /// For when we do the sorting in Python.
    pub fn make_bets_from_array_indices(&self, array_indices: Vec<usize>) -> Bets {
        let data = self.round_dict_data();
        let binaries = array_indices.iter().map(|&i| data.bins[i]).collect();

        let mut bets = Bets::from_binaries(self, binaries);

        bets.fill_bet_amounts(self);

        bets
    }
}

impl NeoFoodClub {
    // win-related stuff

    /// Returns the amount of units you'd win if you placed the given bets.
    /// Returns 0 if there are no winners yet.
    pub fn get_win_units(&self, bets: &Bets) -> u32 {
        let winners_binary = self.winners_binary();

        if winners_binary == 0 {
            return 0;
        }
        let data = self.round_dict_data();

        bets.array_indices
            .iter()
            .map(|i| {
                let bet_bin = data.bins[*i];

                if bet_bin & winners_binary == bet_bin {
                    data.odds[*i]
                } else {
                    0
                }
            })
            .sum()
    }

    /// Returns the amount of neopoints you'd win if you placed the given bets.
    /// Returns 0 if there are no winners yet.
    /// Returns 0 if there are no bet amounts.
    pub fn get_win_np(&self, bets: &Bets) -> u32 {
        let Some(bet_amounts) = bets.bet_amounts.as_ref() else {
            return 0;
        };

        let winners_binary = self.winners_binary();

        if winners_binary == 0 {
            return 0;
        }
        let data = self.round_dict_data();

        bets.array_indices
            .iter()
            .enumerate()
            .fold(0, |acc, (bet_index, array_index)| {
                let bet_bin = data.bins[*array_index];
                if bet_bin & winners_binary == bet_bin {
                    acc + (data.odds[*array_index] * bet_amounts[bet_index].unwrap_or(0))
                        .clamp(0, 1_000_000)
                } else {
                    acc
                }
            })
    }
}
