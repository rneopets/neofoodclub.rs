use itertools::Itertools;
use rand::RngExt;
pub use rustc_hash::FxHashMap;
use rustc_hash::FxHashMap as HashMap;

use crate::chance::Chance;
use crate::error::NfcError;

pub const BET_AMOUNT_MIN: u32 = 1;

/// The largest bet amount that an amounts hash can represent.
///
/// Bets of any amount are valid (a single bet can go up to `1_000_000 / odds`, which is
/// as much as 500,000), and every calculation in this crate uses the real amount. Only the
/// amounts hash is limited: `bet_amounts_to_amounts_hash` encodes each amount in three
/// base-52 characters, and a code only exists for "no amount" and for amounts
/// `1..=BET_AMOUNT_HASH_MAX`. Larger amounts have no code, so hashing one returns an
/// error instead of silently producing a different amount.
pub const BET_AMOUNT_HASH_MAX: u32 = 70_303;

/// Offset of the amounts-hash codec: encoded value = amount + `AMOUNTS_HASH_BASE`, where
/// "no amount" encodes as 0 + `AMOUNTS_HASH_BASE`. Three base-52 characters hold 52^3 =
/// 140,608 codes, so `BET_AMOUNT_HASH_MAX + AMOUNTS_HASH_BASE` is the last valid code.
const AMOUNTS_HASH_BASE: u32 = BET_AMOUNT_HASH_MAX + 1;

// WARNING: the literal integers in this file switches between hex and binary willy-nilly, mostly for readability.

// each arena, as if they were full. this is impossible to actually do.
// BIT_MASKS[i] will accept pirates from arena i and only them. BIT_MASKS[4] == 0b1111, BIT_MASKS[3] == 0b11110000, etc...
pub const BIT_MASKS: [u32; 5] = [0xF0000, 0xF000, 0xF00, 0xF0, 0xF];

// Used by `region_probability` to look up each pirate's odds one slot at a time.
// represents each arena with the same pirate index filled.
// PIRATE_SLOT_MASKS[i] will accept pirates of index i (from 0 to 3) PIRATE_SLOT_MASKS[0] = 0b10001000100010001000, PIRATE_SLOT_MASKS[1] = 0b01000100010001000100, PIRATE_SLOT_MASKS[2] = 0b00100010001000100010, PIRATE_SLOT_MASKS[3] = 0b00010001000100010001
// 0x88888 = (1, 1, 1, 1, 1), which is the first pirate in each arena, and so on.
const PIRATE_SLOT_MASKS: [u32; 4] = [0x88888, 0x44444, 0x22222, 0x11111];

// Turns one bet index (0 to 4, as stored in a bet's `[u8; 5]`) into the bits it accepts in an
// arena. Index 0 means "no pirate picked in this arena", which for a bet means any pirate is
// accepted, so it maps to all 20 bits (0xFFFFF = 0b11111111111111111111). Indexes 1 to 4 map to
// that pirate's slot across every arena; callers mask the result down to one arena with BIT_MASKS.
// Used by `build_payout_regions` to turn each bet into a region.
const ACCEPT_MASK_BY_INDEX: [u32; 5] = [0xFFFFF, 0x88888, 0x44444, 0x22222, 0x11111];

/// Returns the single bit for one pirate: the pirate at `index` (1 to 4) in `arena` (0 to 4).
/// An `index` of 0 means no pirate is picked, and returns 0.
/// ```
/// let bin = neofoodclub::math::pirate_bit(3, 2);
/// assert_eq!(bin, 0x200);
/// ```
#[inline]
pub fn pirate_bit(index: u8, arena: u8) -> u32 {
    // `index` is a 1-based pirate slot (0 = "no pirate", 1..=4 = the four pirates).
    // An out-of-range index would silently land in a neighboring arena, so fail fast.
    debug_assert!(index <= 4, "pirate_bit index out of range: {index}");
    let mask = (index != 0) as u32 * u32::MAX;
    let shift = (index.wrapping_sub(1) as u32 + arena as u32 * 4) & 31;
    (0x80000u32 >> shift) & mask
}

/// Combines one pirate index per arena into a single bet binary (inverse of
/// [`binary_to_indices`]). Each index is 0 (no pick) or 1 to 4.
/// ```
/// let bin = neofoodclub::math::indices_to_binary([0, 1, 2, 3, 4]);
/// assert_eq!(bin, 0x08421);
/// ```
#[inline]
pub fn indices_to_binary(bets_indices: [u8; 5]) -> u32 {
    pirate_bit(bets_indices[0], 0)
        | pirate_bit(bets_indices[1], 1)
        | pirate_bit(bets_indices[2], 2)
        | pirate_bit(bets_indices[3], 3)
        | pirate_bit(bets_indices[4], 4)
}

/// ```
/// let bin = neofoodclub::math::random_full_pirates_binary();
/// assert_eq!(bin.count_ones(), 5);
/// ```
#[inline]
pub fn random_full_pirates_binary() -> u32 {
    let mut rng = rand::rng();

    indices_to_binary([
        rng.random_range(1..=4),
        rng.random_range(1..=4),
        rng.random_range(1..=4),
        rng.random_range(1..=4),
        rng.random_range(1..=4),
    ])
}

// Maps a 4-bit arena nibble to its pirate index.
// Semantics: 0 => 0, otherwise => 4 - trailing_zeros(nibble)
const NIBBLE_TO_INDEX: [u8; 16] = [0, 4, 3, 4, 2, 4, 3, 4, 1, 4, 3, 4, 2, 4, 3, 4];

/// ```
/// let indices = neofoodclub::math::binary_to_indices(1);
/// assert_eq!(indices, [0, 0, 0, 0, 4]);
/// ```
#[inline]
pub fn binary_to_indices(binary: u32) -> [u8; 5] {
    [
        NIBBLE_TO_INDEX[((binary >> 16) & 0xF) as usize],
        NIBBLE_TO_INDEX[((binary >> 12) & 0xF) as usize],
        NIBBLE_TO_INDEX[((binary >> 8) & 0xF) as usize],
        NIBBLE_TO_INDEX[((binary >> 4) & 0xF) as usize],
        NIBBLE_TO_INDEX[(binary & 0xF) as usize],
    ]
}

/// Returns the index of `binary` in the [`RoundTables`] vecs.
///
/// The vecs are built by iterating arenas in a fixed nested order (0..5 each),
/// so the position is determined by base-5 arithmetic on the decoded pirate indices.
/// `binary` must be a valid non-zero bet binary.
#[inline]
pub fn binary_to_table_index(binary: u32) -> usize {
    // A zero binary decodes to all-zero pirate indices, and the trailing `- 1`
    // below would underflow (usize). Valid bet binaries are always non-zero.
    debug_assert_ne!(binary, 0, "binary_to_table_index called with a zero binary");
    let [a, b, c, d, e] = binary_to_indices(binary);
    a as usize * 625 + b as usize * 125 + c as usize * 25 + d as usize * 5 + e as usize - 1
}

#[inline]
pub fn bets_hash_check(bets_hash: &str) -> Result<(), NfcError> {
    if !bets_hash
        .as_bytes()
        .iter()
        .all(|&b| matches!(b, b'a'..=b'y'))
    {
        return Err(NfcError::BetsHash(format!(
            "Invalid bet hash '{}'. Must contain only characters a-y.",
            bets_hash
        )));
    }
    Ok(())
}

#[inline]
pub fn amounts_hash_check(amounts_hash: &str) -> Result<(), NfcError> {
    if !amounts_hash.len().is_multiple_of(3) {
        return Err(NfcError::AmountsHash(format!(
            "Invalid amounts hash '{}'. Length must be a multiple of 3.",
            amounts_hash
        )));
    }

    if !amounts_hash
        .as_bytes()
        .iter()
        .all(|&b| b.is_ascii_alphabetic())
    {
        return Err(NfcError::AmountsHash(format!(
            "Invalid amounts hash '{}'. Must contain only characters a-z and A-Z.",
            amounts_hash
        )));
    }

    Ok(())
}

fn decode_hash_raw(bets_hash: &str) -> Vec<u8> {
    // Pre-allocate output with exact capacity needed (2 values per hash char, rounded up to multiple of 5)
    let raw_len = bets_hash.len() * 2;
    let padded_len = raw_len.div_ceil(5) * 5;
    let mut output = vec![0u8; padded_len];

    // Decode directly using integer division (avoids float conversion)
    for (i, byte) in bets_hash.bytes().enumerate() {
        let e = byte - b'a';
        output[i * 2] = e / 5; // integer division instead of float
        output[i * 2 + 1] = e % 5;
    }
    output
}

#[inline]
fn nonzero_chunks(raw: &[u8]) -> impl Iterator<Item = [u8; 5]> + '_ {
    // due to the way this algorithm works, there could be resulting chunks that are entirely all 0,
    // so we filter them out.
    // good examples:
    // "faa" -> [[1, 0, 0, 0, 0,], [0]]
    // "faafaafaafaafaafaa" -> [[1, 0, 0, 0, 0], [0, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 1, 0], [0, 0, 0, 0, 1], [0, 0, 0, 0, 0], [1, 0, 0, 0, 0]]
    // --------------------------------------------------------------------------------------------------------------^ note the array containing all zeros
    raw.as_chunks::<5>().0.iter().filter_map(|c| {
        // Check if any value is non-zero using bitwise OR (faster than iterator)
        if (c[0] | c[1] | c[2] | c[3] | c[4]) != 0 {
            Some(*c)
        } else {
            None
        }
    })
}

/// Returns the bet indices from a given bet hash.
/// ```
/// let bin = neofoodclub::math::bets_hash_to_bet_indices("").unwrap();
/// assert_eq!(bin, Vec::<[u8;5]>::new());
///
/// let bin = neofoodclub::math::bets_hash_to_bet_indices("f").unwrap();
/// assert_eq!(bin, [[1, 0, 0, 0, 0]]);
///
/// let bin = neofoodclub::math::bets_hash_to_bet_indices("faa").unwrap();
/// assert_eq!(bin, [[1, 0, 0, 0, 0]]);
///
/// let bin = neofoodclub::math::bets_hash_to_bet_indices("faafaafaafaafaafaa").unwrap();
/// assert_eq!(bin, [[1, 0, 0, 0, 0], [0, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 1, 0], [0, 0, 0, 0, 1], [1, 0, 0, 0, 0]]);
///
/// let bin = neofoodclub::math::bets_hash_to_bet_indices("jmbcoemycobmbhofmdcoamyck").unwrap();
/// assert_eq!(bin, [[1, 4, 2, 2, 0], [1, 0, 2, 2, 4], [0, 4, 2, 2, 4], [4, 0, 2, 2, 4], [0, 1, 2, 2, 0], [1, 1, 2, 2, 4], [1, 0, 2, 2, 0], [3, 0, 2, 2, 4], [0, 0, 2, 2, 4], [4, 0, 2, 2, 0]]);
/// ```
#[inline]
pub fn bets_hash_to_bet_indices(bets_hash: &str) -> Result<Vec<[u8; 5]>, NfcError> {
    bets_hash_check(bets_hash)?;
    Ok(nonzero_chunks(&decode_hash_raw(bets_hash)).collect())
}

/// Returns the amount of bets from a given bet hash.
/// ```
/// let count = neofoodclub::math::bets_hash_to_bet_count("faa").unwrap();
/// assert_eq!(count, 1);
///
/// let count = neofoodclub::math::bets_hash_to_bet_count("faafaafaafaafaafaa").unwrap();
/// assert_eq!(count, 6);
///
/// let count = neofoodclub::math::bets_hash_to_bet_count("jmbcoemycobmbhofmdcoamyck").unwrap();
/// assert_eq!(count, 10);
///
/// let count = neofoodclub::math::bets_hash_to_bet_count("dgpqsxgtqsigqqsngrqsegpvsdgfqqsgsqsdgk").unwrap();
/// assert_eq!(count, 15);
/// ```
#[inline]
pub fn bets_hash_to_bet_count(bets_hash: &str) -> Result<usize, NfcError> {
    bets_hash_check(bets_hash)?;
    Ok(nonzero_chunks(&decode_hash_raw(bets_hash)).count())
}

/// Returns the hash of the given bet amounts.
///
/// Returns an error if any amount is above [`BET_AMOUNT_HASH_MAX`], since the hash has no
/// way to represent it.
/// ```
/// let hash = neofoodclub::math::bet_amounts_to_amounts_hash(&vec![Some(50), Some(100), Some(150), Some(200), Some(250)]).unwrap();
/// assert_eq!(hash, "AaYAbWAcUAdSAeQ");
///
/// let hash = neofoodclub::math::bet_amounts_to_amounts_hash(&vec![None, Some(50), Some(100), Some(150), Some(200), Some(250)]).unwrap();
/// assert_eq!(hash, "AaaAaYAbWAcUAdSAeQ");
///
/// let hash = neofoodclub::math::bet_amounts_to_amounts_hash(&vec![None, None, None, None, None, None, None, None, None, None]).unwrap();
/// assert_eq!(hash, "AaaAaaAaaAaaAaaAaaAaaAaaAaaAaa");
///
/// let too_big = neofoodclub::math::bet_amounts_to_amounts_hash(&[Some(70304)]);
/// assert!(too_big.is_err());
/// ```
#[inline]
pub fn bet_amounts_to_amounts_hash(bet_amounts: &[Option<u32>]) -> Result<String, NfcError> {
    if let Some(&amount) = bet_amounts
        .iter()
        .flatten()
        .find(|&&a| a > BET_AMOUNT_HASH_MAX)
    {
        return Err(NfcError::BetAmount(format!(
            "{amount} is above {BET_AMOUNT_HASH_MAX}, the largest amount an amounts hash can represent."
        )));
    }

    // Build as ASCII bytes directly; avoids `Vec<char>` + UTF-8 re-encoding on collect.
    let mut result = vec![0_u8; bet_amounts.len() * 3];
    let mut index = result.len();

    for &value in bet_amounts.iter().rev() {
        let mut state = value.unwrap_or(0) + AMOUNTS_HASH_BASE;

        for _ in 0..3 {
            index -= 1;
            let letter_index = (state % 52) as u8;
            state /= 52;

            result[index] = if letter_index < 26 {
                letter_index + b'a'
            } else {
                (letter_index - 26) + b'A'
            };
        }
    }

    // SAFETY: every byte written is a letter_index offset from b'a' or b'A',
    // both of which are ASCII; the result vec contains only valid UTF-8.
    Ok(unsafe { String::from_utf8_unchecked(result) })
}

/// Returns the bet amounts from a given bet amounts hash.
/// Each element in the resulting vector is an Option, where None means that the bet amount is invalid.
/// "Invalid" here means below 1.
/// ```
/// let amounts = neofoodclub::math::amounts_hash_to_bet_amounts("AaYAbWAcUAdSAeQ").unwrap();
/// assert_eq!(amounts, vec![Some(50), Some(100), Some(150), Some(200), Some(250)]);
/// let amounts = neofoodclub::math::amounts_hash_to_bet_amounts("EmxCoKCoKCglDKUCYqEXkByWBpqzGO").unwrap();
/// assert_eq!(amounts, vec![Some(11463), Some(6172), Some(6172), Some(5731), Some(10030), Some(8024), Some(13374), Some(4000), Some(3500), None]);
/// ```
#[inline]
pub fn amounts_hash_to_bet_amounts(amounts_hash: &str) -> Result<Vec<Option<u32>>, NfcError> {
    #[inline]
    fn decode_index(byte: u8) -> u32 {
        match byte {
            b'a'..=b'z' => (byte - b'a') as u32,
            b'A'..=b'Z' => (byte - b'A' + 26) as u32,
            _ => unreachable!("amounts_hash_check ensures only ASCII [a-zA-Z] bytes"),
        }
    }

    amounts_hash_check(amounts_hash)?;

    #[inline]
    fn push_decoded(out: &mut Vec<Option<u32>>, value: u32) {
        let decoded = value.saturating_sub(AMOUNTS_HASH_BASE);
        out.push(if decoded >= BET_AMOUNT_MIN {
            Some(decoded)
        } else {
            None
        });
    }

    let bytes = amounts_hash.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 3);

    // validates and decodes, pushing every 3 chars.
    let mut value = 0_u32;
    let mut n = 0_u8;

    for &b in bytes {
        let idx = decode_index(b);
        value = value * 52 + idx;
        n += 1;

        if n == 3 {
            push_decoded(&mut out, value);
            value = 0;
            n = 0;
        }
    }

    Ok(out)
}

/// Returns the bet binaries from a given bet hash.
/// ```
/// let bins = neofoodclub::math::bets_hash_to_bet_binaries("faa").unwrap();
/// assert_eq!(bins, vec![0x80000]);
///
/// let bins = neofoodclub::math::bets_hash_to_bet_binaries("faafaafaafaafaafaa").unwrap();
/// assert_eq!(bins, vec![0x80000, 0x8000, 0x800, 0x80, 0x8, 0x80000]);
///
/// let bins = neofoodclub::math::bets_hash_to_bet_binaries("ltqvqwgimhqtvrnywrwvijwnn").unwrap();
/// assert_eq!(bins, vec![0x48212, 0x81828, 0x14888, 0x24484, 0x28211, 0x82442, 0x11142, 0x41418, 0x82811, 0x44242]);
///```
#[inline]
pub fn bets_hash_to_bet_binaries(bets_hash: &str) -> Result<Vec<u32>, NfcError> {
    bets_hash_check(bets_hash)?;
    Ok(bets_hash_to_bet_indices(bets_hash)?
        .iter()
        .map(|&indices| indices_to_binary(indices))
        .collect())
}

/// Returns the hash value from a given bet indices.
/// ```
/// let hash = neofoodclub::math::bet_indices_to_bets_hash(vec![[1, 0, 0, 0, 0]]);
/// assert_eq!(hash, "faa");
/// ```
#[inline]
pub fn bet_indices_to_bets_hash(bets_indices: Vec<[u8; 5]>) -> String {
    let len = bets_indices.len();

    bets_indices
        .into_iter()
        .flatten()
        .chain(std::iter::once(0).take(len & 1))
        .tuples::<(u8, u8)>()
        .map(|(a, b)| (b'a' + a * 5 + b) as char)
        .collect()
}

/// Returns the bet binaries from bet indices.
/// ```
/// let bins = neofoodclub::math::bet_indices_to_bet_binaries(vec![[1, 0, 0, 0, 0]]);
/// assert_eq!(bins, vec![0x80000]);
///
/// let bins = neofoodclub::math::bet_indices_to_bet_binaries(vec![[1, 0, 0, 0, 0], [0, 1, 0, 0, 0], [0, 0, 1, 0, 0], [0, 0, 0, 1, 0], [0, 0, 0, 0, 1], [1, 0, 0, 0, 0]]);
/// assert_eq!(bins, vec![0x80000, 0x8000, 0x800, 0x80, 0x8, 0x80000]);
/// ```
#[inline]
pub fn bet_indices_to_bet_binaries(bets_indices: Vec<[u8; 5]>) -> Vec<u32> {
    bets_indices
        .iter()
        .map(|&indices| indices_to_binary(indices))
        .collect()
}

/// True if every arena has at least one pirate set. A region where this is false cannot
/// contain any winning outcome, so it is dropped instead of tracked.
#[inline]
fn is_nonempty(binary: u32) -> bool {
    (binary & 0xF0000 != 0)
        && (binary & 0xF000 != 0)
        && (binary & 0xF00 != 0)
        && (binary & 0xF0 != 0)
        && (binary & 0xF != 0)
}

/// Probability that the winning outcome falls inside the region `binary`. For each arena it
/// sums the win probabilities of the pirates the region accepts, then multiplies the arenas
/// together (arenas are independent). `probabilities[arena][0]` is unused here.
#[inline]
fn region_probability(binary: u32, probabilities: &[[f64; 5]; 5]) -> f64 {
    BIT_MASKS
        .iter()
        .enumerate()
        .fold(1.0, |total_prob, (x, bit_mask)| {
            let ar_prob: f64 = PIRATE_SLOT_MASKS
                .iter()
                .enumerate()
                .map(|(y, &pir_ib)| {
                    if binary & bit_mask & pir_ib > 0 {
                        probabilities[x][y + 1]
                    } else {
                        0.0
                    }
                })
                .sum();
            total_prob * ar_prob
        })
}

/// Splits the space of winning outcomes into disjoint regions and returns the total payout
/// (sum of bet odds) for each one, keyed by the region's bitmask.
///
/// A region is a 20-bit mask like a bet: one nibble per arena, where a set bit means that
/// pirate is accepted. Regions never overlap, so every possible outcome lands in exactly one
/// of them. Starting from a single region covering everything, each bet carves out the part
/// of every region it overlaps. The overlap gets the bet's odds added, and whatever is left
/// over is split into new disjoint regions that keep the old payout.
///
/// Returns an `FxHashMap` rather than a std `HashMap` since this sits on the odds/chance
/// hot path (`build_chances`, called by `Odds::new` for every bet set). `pyo3`'s
/// `HashMap<K, V, H>` conversions are generic over the hasher, so downstream consumers
/// (including the Python binding) can accept this directly without a SipHash re-hash.
pub fn build_payout_regions(bets: &[[u8; 5]], bet_odds: &[u32]) -> HashMap<u32, u32> {
    // identical bets share a mask, so merge them and sum their odds
    let mut odds_by_mask: HashMap<u32, u32> =
        HashMap::with_capacity_and_hasher(bets.len(), Default::default());
    for (bet, &odds) in bets.iter().zip(bet_odds) {
        let mask = bet.iter().zip(BIT_MASKS).fold(0, |acc, (&pirate, arena)| {
            acc | (ACCEPT_MASK_BY_INDEX[pirate as usize] & arena)
        });
        *odds_by_mask.entry(mask).or_insert(0) += odds;
    }
    let mut bet_masks: Vec<(u32, u32)> = odds_by_mask.into_iter().collect();
    bet_masks.sort_unstable();

    // disjoint (region, payout) pairs; the first region accepts every pirate in every arena
    let mut regions: Vec<(u32, u32)> = vec![(0xFFFFF, 0)];
    let mut leftovers: Vec<(u32, u32)> = Vec::new();
    for (bet, odds) in bet_masks {
        leftovers.clear();
        for (region, payout) in regions.iter_mut() {
            let overlap = bet & *region;
            if !is_nonempty(overlap) {
                continue;
            }
            // Peel off what is outside the bet one arena at a time. After handling an arena,
            // `remaining` is narrowed to the bet's pirates there, so pieces split off in later
            // arenas can't overlap the ones already emitted. Once every arena is narrowed,
            // `remaining` equals `overlap`.
            let mut remaining = *region;
            for arena in BIT_MASKS {
                let outside = remaining & !(overlap & arena);
                if is_nonempty(outside) {
                    leftovers.push((outside, *payout));
                    remaining = (remaining & !arena) | (overlap & arena);
                }
            }
            *region = overlap;
            *payout += odds;
        }
        regions.extend_from_slice(&leftovers);
    }

    regions.into_iter().collect()
}

/// Per-bet lookup tables for a round, one entry for each of the 3124 possible bets (every
/// combination of pirates, 0 to 4 per arena, except all zeros). All vecs share the same index,
/// which is what [`binary_to_table_index`] computes from a bet binary.
#[derive(Debug, Clone)]
pub struct RoundTables {
    pub bins: Vec<u32>,
    pub probs: Vec<f64>,
    pub odds: Vec<u32>,
    pub ers: Vec<f64>,
    pub maxbets: Vec<u32>,
}

/// Builds [`RoundTables`] from each arena's win probabilities (`stds`) and pirate odds.
pub fn build_round_tables(stds: [[f64; 5]; 5], odds: [[u8; 5]; 5]) -> RoundTables {
    let mut bins: Vec<u32> = Vec::with_capacity(3124);
    let mut probs: Vec<f64> = Vec::with_capacity(3124);
    let mut odds_vec: Vec<u32> = Vec::with_capacity(3124);
    let mut ers: Vec<f64> = Vec::with_capacity(3124);
    let mut maxbets: Vec<u32> = Vec::with_capacity(3124);

    // stds[arena][0] == 1.0 and odds[arena][0] == 1 by construction (validated on input),
    // so multiplying by index-0 values is always a no-op, no zero-checks needed.
    for a in 0..5usize {
        let prob_a = stds[0][a];
        let odds_a = odds[0][a] as u32;
        let bin_a = pirate_bit(a as u8, 0);
        for b in 0..5usize {
            let prob_ab = prob_a * stds[1][b];
            let odds_ab = odds_a * odds[1][b] as u32;
            let bin_ab = bin_a | pirate_bit(b as u8, 1);
            for c in 0..5usize {
                let prob_abc = prob_ab * stds[2][c];
                let odds_abc = odds_ab * odds[2][c] as u32;
                let bin_abc = bin_ab | pirate_bit(c as u8, 2);
                for d in 0..5usize {
                    let prob_abcd = prob_abc * stds[3][d];
                    let odds_abcd = odds_abc * odds[3][d] as u32;
                    let bin_abcd = bin_abc | pirate_bit(d as u8, 3);
                    for e in 0..5usize {
                        if a == 0 && b == 0 && c == 0 && d == 0 && e == 0 {
                            continue;
                        }
                        let total_probs = prob_abcd * stds[4][e];
                        let total_odds = odds_abcd * odds[4][e] as u32;
                        let total_bin = bin_abcd | pirate_bit(e as u8, 4);
                        let er = total_probs * total_odds as f64;
                        let maxbet = 1_000_000u32.div_ceil(total_odds);
                        bins.push(total_bin);
                        probs.push(total_probs);
                        odds_vec.push(total_odds);
                        ers.push(er);
                        maxbets.push(maxbet);
                    }
                }
            }
        }
    }

    RoundTables {
        bins,
        probs,
        odds: odds_vec,
        ers,
        maxbets,
    }
}

/// Builds the payout distribution for a set of bets: for each possible total payout (in odds,
/// not NP), the chance of hitting it, plus the cumulative and tail chances. Rows are sorted by
/// payout. `bets` are pirate indices per arena and `bet_odds` are each bet's odds.
pub fn build_chances(
    bets: &[[u8; 5]],
    bet_odds: &[u32],
    probabilities: [[f64; 5]; 5],
) -> Vec<Chance> {
    let expanded = build_payout_regions(bets, bet_odds);
    let mut win_table: HashMap<u32, f64> = HashMap::default();
    for (key, value) in expanded.iter() {
        *win_table.entry(*value).or_insert(0.0) += region_probability(*key, &probabilities);
    }

    let mut sorted: Vec<(u32, f64)> = win_table.into_iter().collect();
    sorted.sort_unstable_by_key(|&(k, _)| k);

    let mut cumulative: f64 = 0.0;
    let mut chances: Vec<Chance> = Vec::with_capacity(sorted.len());
    for (key, value) in sorted {
        // tail is the probability mass beyond the previous row, i.e. 1 - cumulative
        // before this row is added. Deriving it from `cumulative` (a single
        // subtraction) keeps the tail and cumulative columns mutually consistent;
        // accumulating `tail -= value` separately let the two drift by a few ULPs.
        let tail = 1.0 - cumulative;
        cumulative += value;
        chances.push(Chance {
            value: key,
            probability: value,
            cumulative,
            tail,
        });
    }
    chances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn amounts_hash_check_accepts_multiple_of_three_ascii_letters() {
        assert!(amounts_hash_check("").is_ok());
        assert!(amounts_hash_check("AaY").is_ok());
        assert!(amounts_hash_check("AaYAaY").is_ok());
        assert!(amounts_hash_check("abcDEF").is_ok());
    }

    #[test]
    fn amounts_hash_check_rejects_length_not_multiple_of_three() {
        let err = amounts_hash_check("Aa").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid amounts hash: Invalid amounts hash 'Aa'. Length must be a multiple of 3."
        );
    }

    #[test]
    fn amounts_hash_check_rejects_non_alphabetic_characters() {
        let err = amounts_hash_check("Aa1").unwrap_err();
        assert_eq!(
            err.to_string(),
            "Invalid amounts hash: Invalid amounts hash 'Aa1'. Must contain only characters a-z and A-Z."
        );
    }

    /// Brute-force oracle: for every one of the 4^5 winning outcomes, the payout is the sum of
    /// the odds of every bet that accepts it. Each outcome must land in exactly one region, and
    /// that region's payout must match.
    fn assert_matches_brute_force(bets: &[[u8; 5]], bet_odds: &[u32]) {
        let regions = build_payout_regions(bets, bet_odds);
        for outcome in 0..4u32.pow(5) {
            let winners: [u8; 5] = std::array::from_fn(|a| ((outcome >> (2 * a)) & 3) as u8 + 1);
            let outcome_bin = indices_to_binary(winners);
            let expected: u32 = bets
                .iter()
                .zip(bet_odds)
                .filter(|(bet, _)| bet.iter().zip(&winners).all(|(&b, &w)| b == 0 || b == w))
                .map(|(_, &odds)| odds)
                .sum();

            let mut hits = regions.iter().filter(|(&region, _)| {
                is_nonempty(region & outcome_bin) && region & outcome_bin == outcome_bin
            });
            let (_, &payout) = hits.next().expect("outcome not covered by any region");
            assert!(hits.next().is_none(), "outcome covered by multiple regions");
            assert_eq!(payout, expected, "outcome {winners:?}");
        }
    }

    #[test]
    fn build_payout_regions_overlapping_bets() {
        let bets = [
            [1, 2, 0, 0, 0],
            [1, 0, 3, 0, 0],
            [0, 2, 3, 4, 0],
            [1, 2, 3, 4, 1],
        ];
        assert_matches_brute_force(&bets, &[2, 3, 5, 7]);
    }

    #[test]
    fn build_payout_regions_duplicate_bets_sum_odds() {
        let bets = [[1, 1, 1, 1, 1], [1, 1, 1, 1, 1]];
        assert_matches_brute_force(&bets, &[4, 6]);
    }

    #[test]
    fn build_payout_regions_random_bet_sets() {
        // small deterministic LCG so failures are reproducible without a rand dependency
        let mut state = 0x2545_F491_4F6C_DD1Du64;
        let mut next = move |modulus: u64| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) % modulus
        };
        for _ in 0..200 {
            let count = next(12) as usize + 1;
            let bets: Vec<[u8; 5]> = (0..count)
                .map(|_| std::array::from_fn(|_| next(5) as u8))
                .collect();
            let odds: Vec<u32> = (0..count).map(|_| next(12) as u32 + 1).collect();
            assert_matches_brute_force(&bets, &odds);
        }
    }
}
