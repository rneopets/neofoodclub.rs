use neofoodclub::nfc::NeoFoodClub;

// Round 8765 - same fixture as integration_test.rs
const ROUND_DATA_JSON: &str = r#"
{"foods":[[5,20,24,21,18,7,34,29,38,8],[26,24,20,36,33,40,5,13,8,25],[5,29,22,31,40,27,30,4,8,19],[35,19,36,5,12,37,6,3,29,30],[28,24,36,17,18,9,1,33,19,3]],"round":8765,"start":"2023-05-05T23:14:57+00:00","changes":[{"t":"2023-05-06T00:17:30+00:00","new":7,"old":5,"arena":1,"pirate":3},{"t":"2023-05-06T00:21:43+00:00","new":10,"old":8,"arena":3,"pirate":2},{"t":"2023-05-06T00:21:43+00:00","new":6,"old":5,"arena":3,"pirate":3},{"t":"2023-05-06T00:21:43+00:00","new":6,"old":5,"arena":3,"pirate":4},{"t":"2023-05-06T01:09:14+00:00","new":4,"old":3,"arena":4,"pirate":2},{"t":"2023-05-06T01:48:19+00:00","new":3,"old":4,"arena":0,"pirate":4},{"t":"2023-05-06T02:04:11+00:00","new":4,"old":3,"arena":0,"pirate":4},{"t":"2023-05-06T07:29:28+00:00","new":3,"old":4,"arena":0,"pirate":4},{"t":"2023-05-06T09:44:15+00:00","new":5,"old":6,"arena":3,"pirate":3},{"t":"2023-05-06T09:55:08+00:00","new":4,"old":3,"arena":0,"pirate":2},{"t":"2023-05-06T11:11:17+00:00","new":12,"old":11,"arena":0,"pirate":1},{"t":"2023-05-06T16:29:01+00:00","new":11,"old":12,"arena":0,"pirate":1},{"t":"2023-05-06T17:16:30+00:00","new":3,"old":4,"arena":0,"pirate":2},{"t":"2023-05-06T19:16:49+00:00","new":4,"old":5,"arena":2,"pirate":3},{"t":"2023-05-06T19:21:01+00:00","new":6,"old":5,"arena":3,"pirate":3}],"pirates":[[6,11,4,3],[14,15,2,9],[10,16,18,20],[1,12,13,5],[8,19,17,7]],"winners":[3,2,3,2,2],"timestamp":"2023-05-06T23:14:20+00:00","lastChange":"2023-05-06T19:21:01+00:00","currentOdds":[[1,11,3,2,3],[1,13,2,7,13],[1,13,2,4,2],[1,2,10,6,6],[1,13,4,2,4]],"openingOdds":[[1,11,3,2,4],[1,13,2,5,13],[1,13,2,5,2],[1,2,8,5,5],[1,13,3,2,4]]}
"#;

const BET_AMOUNT: u32 = 8000;

fn make_nfc(amount: u32) -> NeoFoodClub {
    NeoFoodClub::from_json(ROUND_DATA_JSON, Some(amount), None, None).expect("valid test JSON")
}

fn make_nfc_no_amount() -> NeoFoodClub {
    NeoFoodClub::from_json(ROUND_DATA_JSON, None, None, None).expect("valid test JSON")
}

#[cfg(test)]
mod payout_table_tests {
    use super::*;

    // Bustproof bets for round 8765 have known odds from the existing stats_table test:
    // [7, 13, 13, 4, 20, 12, 12] (order not guaranteed, so we sort)
    fn sorted_bustproof_odds(nfc: &NeoFoodClub) -> Vec<u32> {
        let bets = nfc.make_bustproof_bets().unwrap();
        let mut odds = bets.odds_values(nfc);
        odds.sort_unstable();
        odds
    }

    #[test]
    fn test_odds_values_bustproof() {
        let nfc = make_nfc(BET_AMOUNT);
        let odds = sorted_bustproof_odds(&nfc);
        assert_eq!(odds, [4, 7, 12, 12, 13, 13, 20]);
    }

    #[test]
    fn test_odds_values_single_arena_bets() {
        let nfc = make_nfc(BET_AMOUNT);
        // 0x02000 = Lagoon pirate at index 3 = odds 7 (from currentOdds[1][3])
        // 0x04000 = Lagoon pirate at index 2 = odds 2 (from currentOdds[1][2])
        let bets = nfc.make_bets_from_binaries(vec![0x02000, 0x04000]);
        let odds = bets.odds_values(&nfc);
        assert_eq!(odds, [7, 2]);
    }

    #[test]
    fn test_odds_values_multi_arena_bet() {
        let nfc = make_nfc(BET_AMOUNT);
        // 0x04080 = Lagoon pirate 2 (odds 2) + Hidden pirate 1 (odds 2) = combined odds 4
        let bets = nfc.make_bets_from_binaries(vec![0x04080]);
        let odds = bets.odds_values(&nfc);
        assert_eq!(odds, [4]);
    }

    #[test]
    fn test_expected_return_list_length_matches_bets() {
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let ers = bets.expected_return_list(&nfc);
        assert_eq!(ers.len(), bets.len());
    }

    #[test]
    fn test_expected_return_list_all_positive() {
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        for er in bets.expected_return_list(&nfc) {
            assert!(er > 0.0, "ER should be positive, got {er}");
        }
    }

    #[test]
    fn test_expected_return_list_approximate_values() {
        // Sorted ER values from the known stats_table output (3 decimal places shown).
        // Actual values within 0.001 of these.
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let mut ers = bets.expected_return_list(&nfc);
        ers.sort_unstable_by(f64::total_cmp);

        let expected_approx = [0.650, 0.650, 1.283, 1.477, 1.577, 1.577, 1.692];
        for (actual, expected) in ers.iter().zip(expected_approx.iter()) {
            assert!(
                (actual - expected).abs() < 0.001,
                "ER {actual:.4} not close to {expected:.3}"
            );
        }
    }

    #[test]
    fn test_net_expected_list_empty_without_bet_amounts() {
        let nfc = make_nfc_no_amount();
        let bets = nfc.make_bustproof_bets().unwrap();
        assert!(bets.net_expected_list(&nfc).is_empty());
    }

    #[test]
    fn test_net_expected_list_length_matches_bets() {
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let nes = bets.net_expected_list(&nfc);
        assert_eq!(nes.len(), bets.len());
    }

    #[test]
    fn test_net_expected_list_approximate_values() {
        // Sorted NE values from the known stats_table output at BET_AMOUNT=8000.
        // Formatted to 2dp in the table, so actual values within 0.005.
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let mut nes = bets.net_expected_list(&nfc);
        nes.sort_unstable_by(f64::total_cmp);

        let expected_approx = [
            -861.35, -861.35, 1107.41, 1295.12, 1537.39, 1537.39, 3817.04,
        ];
        for (actual, expected) in nes.iter().zip(expected_approx.iter()) {
            assert!(
                (actual - expected).abs() < 0.005,
                "NE {actual:.4} not close to {expected:.2}"
            );
        }
    }

    #[test]
    fn test_net_expected_equals_amount_times_er_minus_amount() {
        // NE = amount * ER - amount for each bet
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let nes = bets.net_expected_list(&nfc);
        let ers = bets.expected_return_list(&nfc);
        let amounts = bets.bet_amounts.as_ref().unwrap();

        for ((ne, er), amount_opt) in nes.iter().zip(ers.iter()).zip(amounts.iter()) {
            let amount = amount_opt.unwrap_or(0) as f64;
            let expected_ne = amount.mul_add(*er, -amount);
            assert!(
                (ne - expected_ne).abs() < 1e-9,
                "NE mismatch: {ne} vs {expected_ne}"
            );
        }
    }

    #[test]
    fn test_maxbets_satisfy_odds_constraint() {
        // maxbet = ceil(1_000_000 / odds): maxbet * odds >= 1_000_000 and (maxbet-1) * odds < 1_000_000
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let data = nfc.round_dict_data();

        for &idx in &bets.array_indices {
            let odds = data.odds[idx];
            let maxbet = data.maxbets[idx];
            assert!(
                maxbet * odds >= 1_000_000,
                "maxbet {maxbet} * odds {odds} < 1_000_000"
            );
            if maxbet > 1 {
                assert!(
                    (maxbet - 1) * odds < 1_000_000,
                    "maxbet {maxbet} is not minimal for odds {odds}"
                );
            }
        }
    }

    #[test]
    fn test_fill_bet_amounts_all_bets_capped_by_maxbet() {
        // BET_AMOUNT_MAX = 70_304. All amounts must be <= maxbet and the resulting
        // payout must not exceed 1_000_000.
        let nfc = make_nfc(BET_AMOUNT);
        let mut bets = nfc.make_bustproof_bets().unwrap();
        bets.fill_bet_amounts(&nfc);

        let data = nfc.round_dict_data();
        let amounts = bets.bet_amounts.as_ref().unwrap();

        for (amount_opt, &idx) in amounts.iter().zip(bets.array_indices.iter()) {
            let amount = amount_opt.unwrap();
            let maxbet = data.maxbets[idx];
            assert!(amount <= maxbet, "amount {amount} exceeds maxbet {maxbet}");
            assert!(
                amount * data.odds[idx] <= 1_000_000,
                "bet would exceed jackpot cap"
            );
        }
    }

    #[test]
    fn test_fill_bet_amounts_high_odds_gets_capped_to_maxbet() {
        // BET_AMOUNT_MAX = 70_304. Bustproof bets with odds=20 have maxbet=50_000,
        // which is less than 70_304, so fill_bet_amounts caps the amount to 50_000.
        // Bets with odds <= 13 have maxbet > 70_304, so they fill at the full 70_304.
        use neofoodclub::math::BET_AMOUNT_MAX;

        let nfc = make_nfc(BET_AMOUNT_MAX);
        let mut bets = nfc.make_bustproof_bets().unwrap();
        bets.fill_bet_amounts(&nfc);

        let data = nfc.round_dict_data();
        let amounts = bets.bet_amounts.as_ref().unwrap();

        let mut found_uncapped = false;
        let mut found_capped = false;

        for (amount_opt, &idx) in amounts.iter().zip(bets.array_indices.iter()) {
            let amount = amount_opt.unwrap();
            let maxbet = data.maxbets[idx];
            if maxbet >= BET_AMOUNT_MAX {
                assert_eq!(
                    amount, BET_AMOUNT_MAX,
                    "bet with large maxbet should use full BET_AMOUNT_MAX"
                );
                found_uncapped = true;
            } else {
                assert_eq!(
                    amount, maxbet,
                    "bet with small maxbet should be capped to maxbet"
                );
                found_capped = true;
            }
        }

        assert!(
            found_uncapped,
            "expected at least one bet at full BET_AMOUNT_MAX"
        );
        assert!(
            found_capped,
            "expected at least one bet capped below BET_AMOUNT_MAX"
        );
    }

    #[test]
    fn test_fill_bet_amounts_no_op_without_bet_amount() {
        let nfc = make_nfc_no_amount();
        let mut bets = nfc.make_bustproof_bets().unwrap();
        bets.fill_bet_amounts(&nfc);
        // nfc has no bet_amount, so fill_bet_amounts is a no-op
        assert!(bets.bet_amounts.is_none());
    }

    #[test]
    fn test_stats_table_no_ne_column_without_bet_amounts() {
        let nfc = make_nfc_no_amount();
        let bets = nfc.make_bustproof_bets().unwrap();
        let table = bets.stats_table(&nfc);
        assert!(
            !table.contains("NE"),
            "stats_table without bet amounts should not have NE column"
        );
    }

    #[test]
    fn test_expected_return_sum_matches_scalar() {
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let er_sum: f64 = bets.expected_return_list(&nfc).iter().sum();
        assert!(
            (er_sum - bets.expected_return(&nfc)).abs() < 1e-9,
            "sum of ER list should equal expected_return()"
        );
    }

    #[test]
    fn test_net_expected_sum_matches_scalar() {
        let nfc = make_nfc(BET_AMOUNT);
        let bets = nfc.make_bustproof_bets().unwrap();
        let ne_sum: f64 = bets.net_expected_list(&nfc).iter().sum();
        assert!(
            (ne_sum - bets.net_expected(&nfc)).abs() < 1e-9,
            "sum of NE list should equal net_expected()"
        );
    }
}
