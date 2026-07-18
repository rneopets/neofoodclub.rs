use neofoodclub::math;
use wasm_bindgen::prelude::*;

/// Replaces the internal `parseBets` used by `parseBetUrl`. Decodes a bets
/// hash (`#b=...` fragment) into a flattened n*5 array of pirate indices (0-4
/// per arena, 0 = no pick). Note: unlike the old TS decoder, an all-zero
/// 5-tuple embedded between real bets is dropped (positions compact), and a
/// malformed hash (containing bytes outside a-y) is a hard error rather than
/// silently stripped - callers should catch and fall back to empty bets.
#[wasm_bindgen(js_name = computeBetsHashToIndices)]
pub fn compute_bets_hash_to_indices(bets_hash: &str) -> Result<Vec<u8>, JsError> {
    let indices =
        math::bets_hash_to_bet_indices(bets_hash).map_err(|e| JsError::new(&e.to_string()))?;
    Ok(indices.into_iter().flatten().collect())
}

/// Replaces the internal `makeBetsUrl`. Encodes a flattened n*5 array of
/// pirate indices into a bets hash string.
#[wasm_bindgen(js_name = computeBetsIndicesToHash)]
pub fn compute_bets_indices_to_hash(flat_indices: Vec<u8>) -> Result<String, JsError> {
    if !flat_indices.len().is_multiple_of(5) {
        return Err(JsError::new("length must be a multiple of 5"));
    }
    let indices: Vec<[u8; 5]> = flat_indices.as_chunks::<5>().0.to_vec();
    Ok(math::bets_hash_value(indices))
}

/// Replaces the internal `parseBetAmounts`. Decodes an amounts hash (`#a=...`
/// fragment) into one entry per bet. Rust models "no amount set" as `None`
/// (any decoded value below `BET_AMOUNT_MIN`); that's represented here as
/// `-1` so callers can map it to their own "unset" sentinel without needing
/// an `Option`-aware boundary type.
#[wasm_bindgen(js_name = computeAmountsHashToBetAmounts)]
pub fn compute_amounts_hash_to_bet_amounts(amounts_hash: &str) -> Result<Vec<i64>, JsError> {
    let amounts = math::amounts_hash_to_bet_amounts(amounts_hash)
        .map_err(|e| JsError::new(&e.to_string()))?;
    Ok(amounts
        .into_iter()
        .map(|a| a.map(|v| v as i64).unwrap_or(-1))
        .collect())
}

/// Replaces the internal `makeBetAmountsUrl`. Encodes one amount per bet
/// into an amounts hash string; any value `< 1` (including the `-1` "unset"
/// sentinel from `computeAmountsHashToBetAmounts`) encodes as `None`.
#[wasm_bindgen(js_name = computeBetAmountsToAmountsHash)]
pub fn compute_bet_amounts_to_amounts_hash(amounts: Vec<i64>) -> String {
    let opts: Vec<Option<u32>> = amounts
        .into_iter()
        .map(|v| if v >= 1 { Some(v as u32) } else { None })
        .collect();
    math::bet_amounts_to_amounts_hash(&opts)
}

// Note: only the Ok path of these Result<_, JsError>-returning functions is
// safe to exercise here. Constructing a JsError (the Err path) calls into
// wasm-bindgen's JS glue and panics on non-wasm targets with "cannot call
// wasm-bindgen imported functions on non-wasm targets" - so error-path
// coverage for these functions lives in crates/wasm/tests/web.rs instead,
// run via wasm-pack test --node.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bets_hash_to_indices_decodes_valid_hash() {
        let indices = compute_bets_hash_to_indices("faafaafaafaafaafaa").unwrap();
        assert_eq!(
            indices,
            vec![
                1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0,
                0, 0
            ]
        );
    }

    #[test]
    fn bets_indices_to_hash_round_trips() {
        let flat = compute_bets_hash_to_indices("faa").unwrap();
        let hash = compute_bets_indices_to_hash(flat.clone()).unwrap();
        let round_tripped = compute_bets_hash_to_indices(&hash).unwrap();
        assert_eq!(round_tripped, flat);
    }

    #[test]
    fn amounts_hash_to_bet_amounts_decodes_valid_hash() {
        let amounts = compute_amounts_hash_to_bet_amounts("AaYAbWAcUAdSAeQ").unwrap();
        assert_eq!(amounts, vec![50, 100, 150, 200, 250]);
    }

    #[test]
    fn amounts_hash_to_bet_amounts_uses_negative_one_sentinel_for_unset() {
        // The 10th amount in this hash decodes to None (below BET_AMOUNT_MIN).
        let amounts =
            compute_amounts_hash_to_bet_amounts("EmxCoKCoKCglDKUCYqEXkByWBpqzGO").unwrap();
        assert_eq!(amounts.last(), Some(&-1));
        assert!(amounts[..9].iter().all(|&v| v >= 1));
    }

    #[test]
    fn bet_amounts_to_amounts_hash_round_trips() {
        let amounts = vec![50, 100, 150, 200, 250];
        let hash = compute_bet_amounts_to_amounts_hash(amounts.clone());
        let round_tripped = compute_amounts_hash_to_bet_amounts(&hash).unwrap();
        assert_eq!(round_tripped, amounts);
    }

    #[test]
    fn bet_amounts_to_amounts_hash_encodes_values_below_one_as_unset() {
        // 0 and the -1 sentinel should both encode as "unset".
        let hash_zero = compute_bet_amounts_to_amounts_hash(vec![0]);
        let hash_sentinel = compute_bet_amounts_to_amounts_hash(vec![-1]);
        assert_eq!(hash_zero, hash_sentinel);
        assert_eq!(
            compute_amounts_hash_to_bet_amounts(&hash_zero).unwrap(),
            vec![-1]
        );
    }
}
