// a mapping of positive and negative foods
// the first dimension is the pirate ID,
// the second dimension is the food ID

pub const POSITIVE_FOOD: [[u8; 40]; 20] = [
    [
        2, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 1, 1, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 1, 0, 0, 1, 0, 0, 1,
    ],
    [
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 1, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0,
        1, 1, 0, 0, 0, 0, 0, 1, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 0,
        1, 1, 0, 0, 0, 0, 1, 1, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 1, 0, 0, 1,
    ],
    [
        1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 1, 0, 0, 0,
    ],
    [
        1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 1, 0, 1, 1, 1, 0, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 2, 1, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 1, 2,
    ],
    [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 0, 1, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
];

pub const NEGATIVE_FOOD: [[u8; 40]; 20] = [
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 1,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 1, 0,
    ],
    [
        1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0,
        0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0,
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 0, 0,
    ],
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 0,
        1, 0, 0, 0, 0, 0, 0, 1, 1, 0,
    ],
    [
        0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 1,
        0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
    ],
    [
        0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 1, 1, 0, 0, 0, 0,
    ],
];

#[cfg(test)]
mod tests {
    use super::*;

    // Known valid pirate_id (1-20) and food (1-40) ranges, as used in
    // src/arena.rs and src/pirates.rs via `POSITIVE_FOOD[pirate_id - 1][food - 1]`
    // and `NEGATIVE_FOOD[pirate_id - 1][food - 1]`.
    const PIRATE_ID_RANGE: std::ops::RangeInclusive<u8> = 1..=20;
    const FOOD_RANGE: std::ops::RangeInclusive<u8> = 1..=40;

    #[test]
    fn test_positive_food_values_within_sane_bounds() {
        // observed data uses small integers (0, 1, or 2); make sure nothing
        // has ballooned into something absurd, which would indicate a data
        // entry error.
        for row in POSITIVE_FOOD.iter() {
            for &value in row.iter() {
                assert!(
                    value <= 5,
                    "unexpectedly large POSITIVE_FOOD value: {value}"
                );
            }
        }
    }

    #[test]
    fn test_negative_food_values_within_sane_bounds() {
        for row in NEGATIVE_FOOD.iter() {
            for &value in row.iter() {
                assert!(
                    value <= 5,
                    "unexpectedly large NEGATIVE_FOOD value: {value}"
                );
            }
        }
    }

    #[test]
    fn test_positive_food_has_nonzero_entries() {
        let total: u32 = POSITIVE_FOOD
            .iter()
            .flat_map(|row| row.iter())
            .map(|&v| v as u32)
            .sum();
        assert!(
            total > 0,
            "POSITIVE_FOOD should contain some non-zero entries"
        );
    }

    #[test]
    fn test_negative_food_has_nonzero_entries() {
        let total: u32 = NEGATIVE_FOOD
            .iter()
            .flat_map(|row| row.iter())
            .map(|&v| v as u32)
            .sum();
        assert!(
            total > 0,
            "NEGATIVE_FOOD should contain some non-zero entries"
        );
    }

    #[test]
    fn test_every_pirate_has_at_least_one_positive_food() {
        // every pirate (row) should have at least one food that helps them,
        // otherwise the adjustment tables would be pointless for that pirate.
        for (pirate_index, row) in POSITIVE_FOOD.iter().enumerate() {
            let pirate_id = pirate_index + 1;
            assert!(
                row.iter().any(|&v| v > 0),
                "pirate_id {pirate_id} has no positive food adjustments"
            );
        }
    }

    #[test]
    fn test_every_pirate_has_at_least_one_negative_food() {
        for (pirate_index, row) in NEGATIVE_FOOD.iter().enumerate() {
            let pirate_id = pirate_index + 1;
            assert!(
                row.iter().any(|&v| v > 0),
                "pirate_id {pirate_id} has no negative food adjustments"
            );
        }
    }

    #[test]
    fn test_indices_are_accessible_for_full_valid_range() {
        // exercise the exact indexing pattern used in arena.rs/pirates.rs:
        // POSITIVE_FOOD[pirate_id - 1][food - 1], for the full valid range of
        // pirate_id (1-20) and food (1-40), ensuring no panics occur.
        for pirate_id in PIRATE_ID_RANGE {
            for food in FOOD_RANGE {
                let _pfa = POSITIVE_FOOD[pirate_id as usize - 1][food as usize - 1];
                let nfa = NEGATIVE_FOOD[pirate_id as usize - 1][food as usize - 1];
                // values should be representable losslessly when negated per
                // the arena.rs usage (`.sub(NEGATIVE_FOOD[..] as i8)`)
                assert!(nfa as i8 >= 0);
            }
        }
    }

    #[test]
    fn test_no_food_is_both_strictly_positive_and_negative_everywhere_zero() {
        // sanity check: not every single food/pirate combo is zero across
        // both tables (i.e. the tables aren't accidentally all-zero).
        let any_nonzero =
            POSITIVE_FOOD
                .iter()
                .zip(NEGATIVE_FOOD.iter())
                .any(|(pos_row, neg_row)| {
                    pos_row.iter().any(|&v| v != 0) || neg_row.iter().any(|&v| v != 0)
                });
        assert!(any_nonzero);
    }

    #[test]
    fn test_known_pirate_dan_food_adjustments() {
        // Dan is pirate_id 1. Spot-check a couple of known values from the
        // literal table data above to guard against accidental data corruption.
        // POSITIVE_FOOD[0] starts with [2, 0, 0, 1, 0, 1, ...]
        assert_eq!(POSITIVE_FOOD[0][0], 2); // food 1
        assert_eq!(POSITIVE_FOOD[0][1], 0); // food 2
        assert_eq!(POSITIVE_FOOD[0][3], 1); // food 4
                                            // NEGATIVE_FOOD[0] starts with [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, ...]
        assert_eq!(NEGATIVE_FOOD[0][13], 1); // food 14
        assert_eq!(NEGATIVE_FOOD[0][0], 0); // food 1
    }
}
