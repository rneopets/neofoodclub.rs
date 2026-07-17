#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let _ = neofoodclub::math::bets_hash_to_bet_indices(data);
    let _ = neofoodclub::math::bets_hash_to_bet_binaries(data);
});