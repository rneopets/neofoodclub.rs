use thiserror::Error;

/// Errors that can occur when constructing or working with [`crate::nfc::NeoFoodClub`].
#[derive(Debug, Error)]
pub enum NfcError {
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URL query string: {0}")]
    QueryString(String),

    #[error("Invalid modifier: {0}")]
    Modifier(String),

    #[error("No NeoFoodClub data found in URL")]
    InvalidUrl,

    #[error("Invalid round data: {0}")]
    RoundData(String),

    #[error("Too many pirates selected from one arena")]
    TooManyPiratesInArena,

    #[error("Invalid bets hash: {0}")]
    BetsHash(String),

    #[error("Invalid amounts hash: {0}")]
    AmountsHash(String),

    #[error("Invalid bet amount: {0}")]
    BetAmount(String),

    #[error("Invalid bet: {0}")]
    InvalidBet(String),
}
