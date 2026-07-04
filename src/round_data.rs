use crate::oddschange::OddsChange;
use crate::utils::{convert_from_utc_to_nst, timestamp_to_utc};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RoundData {
    pub foods: Option<[[u8; 10]; 5]>,
    pub round: u16,
    pub start: Option<String>,
    pub pirates: [[u8; 4]; 5],
    pub currentOdds: [[u8; 5]; 5],
    pub customOdds: Option<[[u8; 5]; 5]>,
    pub openingOdds: [[u8; 5]; 5],
    pub winners: Option<[u8; 5]>,
    pub timestamp: Option<String>,
    pub changes: Option<Vec<OddsChange>>,
    pub lastChange: Option<String>,
}

/// Validates that a timestamp string is a well-formed RFC3339 timestamp.
///
/// This is used by [`crate::nfc::NeoFoodClub`]'s construction-time validation
/// (`validate_round_data`) to reject malformed `start`/`lastChange`/`timestamp`/
/// `changes[].t` strings *before* they ever reach the accessor methods below
/// (e.g. [`RoundData::start_utc`], [`crate::oddschange::OddsChange::timestamp_utc`]),
/// which call [`crate::utils::timestamp_to_utc`] and panic on invalid input.
/// Because that invariant is enforced here, at construction time,
/// `timestamp_to_utc` itself is allowed to keep unwrapping internally.
pub(crate) fn validate_timestamp(timestamp: &str) -> Result<(), crate::error::NfcError> {
    chrono::DateTime::parse_from_rfc3339(timestamp).map_err(|e| {
        crate::error::NfcError::RoundData(format!("invalid timestamp '{}': {}", timestamp, e))
    })?;
    Ok(())
}

impl RoundData {
    /// Returns the start time of the round in NST.
    /// If the start time is not available, returns None.
    pub fn start_nst(&self) -> Option<DateTime<Tz>> {
        self.start
            .as_ref()
            .map(|start| convert_from_utc_to_nst(timestamp_to_utc(start)))
    }

    /// Returns the last change time of the round in NST.
    /// If the last change time is not available, returns None.
    pub fn last_change_nst(&self) -> Option<DateTime<Tz>> {
        self.lastChange
            .as_ref()
            .map(|last_change| convert_from_utc_to_nst(timestamp_to_utc(last_change)))
    }

    /// Returns the timestamp of the round in NST.
    /// If the timestamp is not available, returns None.
    pub fn timestamp_nst(&self) -> Option<DateTime<Tz>> {
        self.timestamp
            .as_ref()
            .map(|timestamp| convert_from_utc_to_nst(timestamp_to_utc(timestamp)))
    }

    /// Returns the start time of the round in UTC.
    /// If the start time is not available, returns None.
    pub fn start_utc(&self) -> Option<DateTime<Utc>> {
        self.start.as_ref().map(|start| timestamp_to_utc(start))
    }

    /// Returns the last change time of the round in UTC.
    /// If the last change time is not available, returns None.
    pub fn last_change_utc(&self) -> Option<DateTime<Utc>> {
        self.lastChange
            .as_ref()
            .map(|last_change| timestamp_to_utc(last_change))
    }

    /// Returns the timestamp of the round in UTC.
    /// If the timestamp is not available, returns None.
    pub fn timestamp_utc(&self) -> Option<DateTime<Utc>> {
        self.timestamp
            .as_ref()
            .map(|timestamp| timestamp_to_utc(timestamp))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_timestamp_valid() {
        assert!(validate_timestamp("2023-01-01T00:00:00+00:00").is_ok());
    }

    #[test]
    fn test_validate_timestamp_invalid() {
        assert!(validate_timestamp("not-a-timestamp").is_err());
    }
}
