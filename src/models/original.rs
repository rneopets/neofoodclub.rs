use crate::round_data::RoundData;

#[derive(Debug, Clone)]
pub struct OriginalModel;

impl OriginalModel {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(round_data: &RoundData) -> [[f64; 5]; 5] {
        make_probabilities(round_data.opening_odds)
    }
}

pub fn make_probabilities(odds: [[u8; 5]; 5]) -> [[f64; 5]; 5] {
    let mut std = [[1.0, 0.0, 0.0, 0.0, 0.0]; 5];
    let mut min = [[1.0, 0.0, 0.0, 0.0, 0.0]; 5];
    let mut max = [[1.0, 0.0, 0.0, 0.0, 0.0]; 5];
    // let mut used = [[1.0, 0.0, 0.0, 0.0, 0.0]; 5];

    // turns out we only use _std values in the python implementation of NFC
    // keeping the _used math to avoid confusion between NFC impls
    // however, if we use this Rust code on the frontend of neofood.club
    // that's the best time to expose this.

    for arena in 0..5 {
        let mut min_prob: f64 = 0.0;
        let mut max_prob: f64 = 0.0;

        for pirate in 1..5 {
            let (min_val, max_val) = match odds[arena][pirate] {
                13 => (0.0, 1.0 / 13.0),
                2 => (1.0 / 3.0, 1.0),
                pirate_odd => {
                    let p_o = pirate_odd as f64;
                    (1.0 / (1.0 + p_o), 1.0 / p_o)
                }
            };

            min[arena][pirate] = min_val;
            max[arena][pirate] = max_val;

            min_prob += min_val;
            max_prob += max_val;
        }

        for pirate in 1..5 {
            let min_original: f64 = min[arena][pirate];
            let max_original: f64 = max[arena][pirate];

            min[arena][pirate] = f64::max(min_original, 1.0 + max_original - max_prob);
            max[arena][pirate] = f64::min(max_original, 1.0 + min_original - min_prob);
            std[arena][pirate] = match odds[arena][pirate] {
                13 => 0.05,
                _ => (min[arena][pirate] + max[arena][pirate]) / 2.0,
            };
        }

        for rectify_level in 2..13 {
            let mut rectify_count: f64 = 0.0;
            let mut std_total: f64 = 0.0;
            let mut rectify_value: f64 = 0.0;
            let mut max_rectify_value: f64 = 1.0;

            for pirate in 1..5 {
                std_total += std[arena][pirate];
                if odds[arena][pirate] <= rectify_level {
                    rectify_count += 1.0;
                    rectify_value += std[arena][pirate] - min[arena][pirate];
                    max_rectify_value =
                        f64::min(max_rectify_value, max[arena][pirate] - min[arena][pirate]);
                }
            }

            if std_total == 1.0 {
                break;
            }

            if !(std_total - rectify_value > 1.0
                || rectify_count == 0.0
                || max_rectify_value * rectify_count < rectify_value + 1.0 - std_total)
            {
                rectify_value = (rectify_value + 1.0 - std_total) / rectify_count;
                for pirate in 1..5 {
                    if odds[arena][pirate] <= rectify_level {
                        std[arena][pirate] = min[arena][pirate] + rectify_value;
                    }
                }
                break;
            }
        }

        // let mut return_sum = 0.0;
        // for pirate in 1..5 {
        //     used[arena][pirate] = std[arena][pirate];
        //     return_sum += used[arena][pirate];
        // }

        // for pirate in 1..5 {
        //     used[arena][pirate] /= return_sum;
        // }
    }

    std
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    // Real-world fixture odds (round 8765), reused from tests/integration_test.rs.
    const OPENING_ODDS: [[u8; 5]; 5] = [
        [1, 11, 3, 2, 4],
        [1, 13, 2, 5, 13],
        [1, 13, 2, 5, 2],
        [1, 2, 8, 5, 5],
        [1, 13, 3, 2, 4],
    ];

    const CURRENT_ODDS: [[u8; 5]; 5] = [
        [1, 11, 3, 2, 3],
        [1, 13, 2, 7, 13],
        [1, 13, 2, 4, 2],
        [1, 2, 10, 6, 6],
        [1, 13, 4, 2, 4],
    ];

    #[test]
    fn test_make_probabilities_bounds_opening_odds() {
        let probs = make_probabilities(OPENING_ODDS);
        for arena in probs.iter() {
            for &p in arena[1..5].iter() {
                assert!((0.0..=1.0).contains(&p), "probability out of bounds: {p}");
            }
        }
    }

    #[test]
    fn test_make_probabilities_bounds_current_odds() {
        let probs = make_probabilities(CURRENT_ODDS);
        for arena in probs.iter() {
            for &p in arena[1..5].iter() {
                assert!((0.0..=1.0).contains(&p), "probability out of bounds: {p}");
            }
        }
    }

    #[test]
    fn test_make_probabilities_sums_to_one_opening_odds() {
        let probs = make_probabilities(OPENING_ODDS);
        for arena in probs.iter() {
            let sum: f64 = arena[1..5].iter().sum();
            assert!(
                (sum - 1.0).abs() < EPSILON,
                "arena probabilities did not sum to 1.0: {sum}"
            );
        }
    }

    #[test]
    fn test_make_probabilities_sums_to_one_current_odds() {
        let probs = make_probabilities(CURRENT_ODDS);
        for arena in probs.iter() {
            let sum: f64 = arena[1..5].iter().sum();
            assert!(
                (sum - 1.0).abs() < EPSILON,
                "arena probabilities did not sum to 1.0: {sum}"
            );
        }
    }

    #[test]
    fn test_make_probabilities_first_column_is_always_one() {
        // index 0 in each arena's row is a fixed placeholder (unused pirate slot)
        let probs = make_probabilities(OPENING_ODDS);
        for arena in probs.iter() {
            assert_eq!(arena[0], 1.0);
        }
    }

    #[test]
    fn test_original_model_new_matches_make_probabilities() {
        use crate::round_data::RoundData;

        let round_data: RoundData = serde_json::from_str(
            r#"{"foods":null,"round":8765,"start":null,"pirates":[[6,11,4,3],[14,15,2,9],[10,16,18,20],[1,12,13,5],[8,19,17,7]],"currentOdds":[[1,11,3,2,3],[1,13,2,7,13],[1,13,2,4,2],[1,2,10,6,6],[1,13,4,2,4]],"customOdds":null,"openingOdds":[[1,11,3,2,4],[1,13,2,5,13],[1,13,2,5,2],[1,2,8,5,5],[1,13,3,2,4]],"winners":null,"timestamp":null,"changes":null,"lastChange":null}"#,
        )
        .unwrap();

        let from_model = OriginalModel::new(&round_data);
        let expected = make_probabilities(round_data.opening_odds);

        assert_eq!(from_model, expected);
    }
}
