//! Tests for the functions in this crate that return `JsValue` or hit the
//! `JsError` `Err` path. Those two things call into wasm-bindgen's JS glue,
//! which panics on non-wasm targets - so unlike the plain `#[test]`s in
//! `src/hash.rs` and `src/lib.rs`, these must run under `wasm-pack test --node`.

use neofoodclub_wasm::*;
use serde::Deserialize;
use wasm_bindgen_test::*;

const ROUND_DATA_JSON: &str = r#"
{"foods":[[5,20,24,21,18,7,34,29,38,8],[26,24,20,36,33,40,5,13,8,25],[5,29,22,31,40,27,30,4,8,19],[35,19,36,5,12,37,6,3,29,30],[28,24,36,17,18,9,1,33,19,3]],"round":8765,"start":"2023-05-05T23:14:57+00:00","changes":[{"t":"2023-05-06T00:17:30+00:00","new":7,"old":5,"arena":1,"pirate":3},{"t":"2023-05-06T00:21:43+00:00","new":10,"old":8,"arena":3,"pirate":2}],"pirates":[[6,11,4,3],[14,15,2,9],[10,16,18,20],[1,12,13,5],[8,19,17,7]],"winners":[3,2,3,2,2],"timestamp":"2023-05-06T23:14:20+00:00","lastChange":"2023-05-06T19:21:01+00:00","currentOdds":[[1,11,3,2,3],[1,13,2,7,13],[1,13,2,4,2],[1,2,10,6,6],[1,13,4,2,4]],"openingOdds":[[1,11,3,2,4],[1,13,2,5,13],[1,13,2,5,2],[1,2,8,5,5],[1,13,3,2,4]]}
"#;

#[derive(Deserialize)]
struct ChanceOutTest {
    #[allow(dead_code)]
    value: u32,
    probability: f64,
    cumulative: f64,
    tail: f64,
}

#[derive(Deserialize)]
struct PayoutTablesOutTest {
    odds: Vec<ChanceOutTest>,
    winnings: Vec<ChanceOutTest>,
}

#[derive(Deserialize)]
struct BetsOutTest {
    indices: Vec<[u8; 5]>,
    #[allow(dead_code)]
    amounts: Option<Vec<Option<u32>>>,
    #[serde(rename = "betsHash")]
    bets_hash: String,
    #[allow(dead_code)]
    #[serde(rename = "amountsHash")]
    amounts_hash: Option<String>,
}

fn assert_chances_well_formed(chances: &[ChanceOutTest]) {
    assert!(!chances.is_empty());
    // Each entry's `tail` is the running "probability not yet accounted for"
    // *before* that entry's own probability is subtracted off, so the first
    // entry always starts at 1.0 and cumulative/tail move monotonically.
    assert!((chances[0].tail - 1.0).abs() < 1e-9);
    let mut prev_cumulative = 0.0;
    let mut prev_tail = 1.0;
    for chance in chances {
        assert!((chance.cumulative - (prev_cumulative + chance.probability)).abs() < 1e-9);
        assert!(chance.tail <= prev_tail + 1e-9);
        prev_cumulative = chance.cumulative;
        prev_tail = chance.tail;
    }
}

#[wasm_bindgen_test]
fn compute_payout_tables_returns_well_formed_tables() {
    let bet_indices = vec![1, 0, 0, 0, 0];
    let probabilities = vec![0.1; 25];
    let bet_odds = vec![13];
    let bet_payoffs = vec![1300];

    let result = compute_payout_tables(bet_indices, probabilities, bet_odds, bet_payoffs).unwrap();
    let parsed: PayoutTablesOutTest = serde_wasm_bindgen::from_value(result).unwrap();

    assert_chances_well_formed(&parsed.odds);
    assert_chances_well_formed(&parsed.winnings);
}

#[wasm_bindgen_test]
fn compute_payout_tables_rejects_bad_bet_indices_length() {
    let result = compute_payout_tables(vec![1, 0, 0, 0], vec![0.1; 25], vec![13], vec![1300]);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn compute_payout_tables_rejects_mismatched_odds_payoffs_length() {
    let result =
        compute_payout_tables(vec![1, 0, 0, 0, 0], vec![0.1; 25], vec![13, 20], vec![1300]);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn compute_payout_tables_rejects_wrong_probabilities_length() {
    let result = compute_payout_tables(vec![1, 0, 0, 0, 0], vec![0.1; 24], vec![13], vec![1300]);
    assert!(result.is_err());
}

#[wasm_bindgen_test]
fn compute_bets_hash_to_indices_rejects_malformed_hash() {
    // 'z' is outside the valid a-y range.
    assert!(compute_bets_hash_to_indices("zzz").is_err());
}

#[wasm_bindgen_test]
fn compute_bets_indices_to_hash_rejects_bad_length() {
    assert!(compute_bets_indices_to_hash(vec![1, 0, 0, 0]).is_err());
}

#[wasm_bindgen_test]
fn compute_arena_ratios_rejects_wrong_length() {
    assert!(compute_arena_ratios(vec![1.0; 24]).is_err());
}

#[wasm_bindgen_test]
fn compute_pirates_binary_rejects_wrong_length() {
    assert!(compute_pirates_binary(vec![1, 2, 3]).is_err());
}

#[wasm_bindgen_test]
fn compute_std_probabilities_rejects_invalid_json() {
    assert!(compute_std_probabilities("not json", false).is_err());
}

#[wasm_bindgen_test]
fn nfc_engine_full_flow() {
    let mut engine = NfcEngine::new(ROUND_DATA_JSON, Some(8000), false).unwrap();

    engine.set_bet_amount(Some(5000));

    // 25-element flattened odds grid (5 arenas x [unused, o1..o4]).
    let odds_grid: Vec<u8> = vec![
        0, 11, 3, 2, 3, 0, 13, 2, 7, 13, 0, 13, 2, 4, 2, 0, 2, 10, 6, 6, 0, 13, 4, 2, 4,
    ];
    engine.set_custom_odds(odds_grid).unwrap();
    engine.clear_custom_odds().unwrap();

    let probs_grid: Vec<f64> = vec![0.2; 25];
    engine.set_custom_probabilities(probs_grid).unwrap();
    engine.clear_custom_probabilities();

    let result = engine.make_max_ter_bets().unwrap();
    let parsed: BetsOutTest = serde_wasm_bindgen::from_value(result).unwrap();
    assert!(!parsed.indices.is_empty());
    assert!(!parsed.bets_hash.is_empty());
}

#[wasm_bindgen_test]
fn nfc_engine_make_gambit_bets_rejects_wrong_pirate_count() {
    let engine = NfcEngine::new(ROUND_DATA_JSON, Some(8000), false).unwrap();
    // Selects only one pirate total instead of exactly 5 (one per arena).
    let single_pick = 1u32;
    assert!(engine.make_gambit_bets(single_pick).is_err());
}
