use pyo3::prelude::*;

use crate::chance::Chance;

#[pyclass(name = "Math", module = "math", frozen)]
pub struct Math;

#[pymethods]
impl Math {
    #[classattr]
    const BIT_MASKS: [u32; 5] = neofoodclub::math::BIT_MASKS;

    #[classattr]
    pub const BET_AMOUNT_MIN: u32 = neofoodclub::math::BET_AMOUNT_MIN;

    #[classattr]
    pub const BET_AMOUNT_HASH_MAX: u32 = neofoodclub::math::BET_AMOUNT_HASH_MAX;

    #[staticmethod]
    fn pirate_bit(index: u8, arena: u8) -> u32 {
        neofoodclub::math::pirate_bit(index, arena)
    }

    #[staticmethod]
    fn indices_to_binary(bets_indices: [u8; 5]) -> u32 {
        neofoodclub::math::indices_to_binary(bets_indices)
    }

    #[staticmethod]
    fn binary_to_indices(binary: u32) -> (u8, u8, u8, u8, u8) {
        let arr = neofoodclub::math::binary_to_indices(binary);
        (arr[0], arr[1], arr[2], arr[3], arr[4])
    }

    #[staticmethod]
    fn bets_hash_to_bet_indices(bets_hash: &str) -> PyResult<Vec<[u8; 5]>> {
        neofoodclub::math::bets_hash_to_bet_indices(bets_hash)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn bet_amounts_to_amounts_hash(bet_amounts: Vec<Option<u32>>) -> PyResult<String> {
        neofoodclub::math::bet_amounts_to_amounts_hash(&bet_amounts)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn bet_indices_to_bets_hash(bets_indices: Vec<[u8; 5]>) -> String {
        neofoodclub::math::bet_indices_to_bets_hash(bets_indices)
    }

    #[staticmethod]
    fn amounts_hash_to_bet_amounts<'py>(
        py: Python<'py>,
        amounts_hash: &str,
    ) -> PyResult<Bound<'py, pyo3::types::PyTuple>> {
        let amounts = neofoodclub::math::amounts_hash_to_bet_amounts(amounts_hash)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
        pyo3::types::PyTuple::new(py, amounts)
    }

    #[staticmethod]
    fn bets_hash_to_bet_binaries(bets_hash: &str) -> PyResult<Vec<u32>> {
        neofoodclub::math::bets_hash_to_bet_binaries(bets_hash)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn bets_hash_to_bet_count(bets_hash: &str) -> PyResult<usize> {
        neofoodclub::math::bets_hash_to_bet_count(bets_hash)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))
    }

    #[staticmethod]
    fn bet_indices_to_bet_binaries(bets_indices: Vec<[u8; 5]>) -> Vec<u32> {
        neofoodclub::math::bet_indices_to_bet_binaries(bets_indices)
    }

    #[staticmethod]
    fn build_chances(
        bets: Vec<[u8; 5]>,
        bet_odds: Vec<u32>,
        probabilities: [[f64; 5]; 5],
    ) -> Vec<Chance> {
        neofoodclub::math::build_chances(&bets, &bet_odds, probabilities)
            .into_iter()
            .map(Chance::from)
            .collect()
    }

    #[staticmethod]
    fn build_payout_regions(
        bets: Vec<[u8; 5]>,
        bet_odds: Vec<u32>,
    ) -> neofoodclub::math::FxHashMap<u32, u32> {
        neofoodclub::math::build_payout_regions(&bets, &bet_odds)
    }
}
