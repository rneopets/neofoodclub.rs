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
    let indices: Vec<[u8; 5]> = flat_indices
        .chunks_exact(5)
        .map(|c| [c[0], c[1], c[2], c[3], c[4]])
        .collect();
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
