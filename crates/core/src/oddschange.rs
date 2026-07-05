use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::{
    arena::ARENA_NAMES,
    nfc::NeoFoodClub,
    pirates::PartialPirate,
    utils::{convert_from_utc_to_nst, timestamp_to_utc},
};

/// Represents a change in odds.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OddsChange {
    pub t: String,
    pub new: u8,
    pub old: u8,
    arena: u8,
    pirate: u8,
}

impl OddsChange {
    /// Returns the pirate associated with the change.
    pub fn pirate(&self, nfc: &NeoFoodClub) -> PartialPirate {
        PartialPirate {
            id: self.pirate_id(nfc),
        }
    }

    /// Returns the pirate ID associated with the change.
    pub fn pirate_id(&self, nfc: &NeoFoodClub) -> usize {
        nfc.pirates()[self.arena_index()][self.pirate_index() - 1] as usize
    }

    /// Returns the name of the arena associated with the change.
    pub fn arena(&self) -> &str {
        ARENA_NAMES[self.arena as usize]
    }

    /// Returns the index of the pirate associated with the change.
    #[inline]
    pub fn pirate_index(&self) -> usize {
        self.pirate as usize
    }

    /// Returns the index of the arena associated with the change.
    #[inline]
    pub fn arena_index(&self) -> usize {
        self.arena as usize
    }

    /// Returns the timestamp of the change in NST.
    pub fn timestamp_nst(&self) -> DateTime<Tz> {
        convert_from_utc_to_nst(self.timestamp_utc())
    }

    /// Returns the timestamp of the change in UTC.
    pub fn timestamp_utc(&self) -> DateTime<Utc> {
        timestamp_to_utc(&self.t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Real-world fixture JSON (round 8765), same as used in tests/integration_test.rs.
    const ROUND_DATA_JSON: &str = r#"
{"foods":[[5,20,24,21,18,7,34,29,38,8],[26,24,20,36,33,40,5,13,8,25],[5,29,22,31,40,27,30,4,8,19],[35,19,36,5,12,37,6,3,29,30],[28,24,36,17,18,9,1,33,19,3]],"round":8765,"start":"2023-05-05T23:14:57+00:00","changes":[{"t":"2023-05-06T00:17:30+00:00","new":7,"old":5,"arena":1,"pirate":3},{"t":"2023-05-06T00:21:43+00:00","new":10,"old":8,"arena":3,"pirate":2}],"pirates":[[6,11,4,3],[14,15,2,9],[10,16,18,20],[1,12,13,5],[8,19,17,7]],"winners":[3,2,3,2,2],"timestamp":"2023-05-06T23:14:20+00:00","lastChange":"2023-05-06T19:21:01+00:00","currentOdds":[[1,11,3,2,3],[1,13,2,7,13],[1,13,2,4,2],[1,2,10,6,6],[1,13,4,2,4]],"openingOdds":[[1,11,3,2,4],[1,13,2,5,13],[1,13,2,5,2],[1,2,8,5,5],[1,13,3,2,4]]}
"#;

    fn make_test_nfc() -> NeoFoodClub {
        NeoFoodClub::from_json(ROUND_DATA_JSON, Some(8000), None, None).expect("valid test JSON")
    }

    fn make_change(arena: u8, pirate: u8) -> OddsChange {
        OddsChange {
            t: "2023-05-06T00:17:30+00:00".to_string(),
            new: 7,
            old: 5,
            arena,
            pirate,
        }
    }

    #[test]
    fn test_arena_index_and_name() {
        let change = make_change(1, 3);
        assert_eq!(change.arena_index(), 1);
        assert_eq!(change.arena(), "Lagoon");
        assert!(ARENA_NAMES.contains(&change.arena()));
    }

    #[test]
    fn test_arena_index_and_name_all_arenas() {
        for (arena_index, &name) in ARENA_NAMES.iter().enumerate() {
            let change = make_change(arena_index as u8, 1);
            assert_eq!(change.arena_index(), arena_index);
            assert_eq!(change.arena(), name);
        }
    }

    #[test]
    fn test_pirate_index() {
        let change = make_change(3, 2);
        assert_eq!(change.pirate_index(), 2);
    }

    #[test]
    fn test_pirate_id_and_pirate() {
        let nfc = make_test_nfc();

        // fixture pirates: arena index 1 is [14, 15, 2, 9]
        // pirate index 3 (one-indexed) -> pirates()[1][3 - 1] == 2
        let change = make_change(1, 3);
        assert_eq!(change.pirate_id(&nfc), 2);
        assert_eq!(change.pirate(&nfc).id, 2);

        // fixture pirates: arena index 3 is [1, 12, 13, 5]
        // pirate index 2 (one-indexed) -> pirates()[3][2 - 1] == 12
        let change2 = make_change(3, 2);
        assert_eq!(change2.pirate_id(&nfc), 12);
        assert_eq!(change2.pirate(&nfc).id, 12);
    }

    #[test]
    fn test_timestamp_utc_parses_known_rfc3339_string() {
        let change = make_change(1, 3);
        let utc = change.timestamp_utc();

        assert_eq!(utc.to_rfc3339(), "2023-05-06T00:17:30+00:00");
        assert_eq!(utc.timestamp(), 1683332250);
    }

    #[test]
    fn test_timestamp_nst_parses_known_rfc3339_string() {
        let change = make_change(1, 3);
        let utc = change.timestamp_utc();
        let nst = change.timestamp_nst();

        // NST conversion should preserve the same instant in time.
        assert_eq!(nst.with_timezone(&Utc), utc);
    }

    #[test]
    fn test_changes_from_round_data_parse_correctly() {
        let nfc = make_test_nfc();
        let changes = nfc.changes().as_ref().expect("fixture has changes");

        assert_eq!(changes.len(), 2);

        let first = &changes[0];
        assert_eq!(first.arena_index(), 1);
        assert_eq!(first.arena(), "Lagoon");
        assert_eq!(first.pirate_index(), 3);
        assert_eq!(first.pirate_id(&nfc), 2);
        assert_eq!(
            first.timestamp_utc().to_rfc3339(),
            "2023-05-06T00:17:30+00:00"
        );
    }
}
