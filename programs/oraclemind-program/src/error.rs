use anchor_lang::prelude::*;

#[error_code]
pub enum Error {
    #[msg("Market has already been resolved")]
    MarketAlreadyResolved,

    #[msg("Market end time has been exceeded")]
    MarketEndTimeExceeded,

    #[msg("Market has not ended yet")]
    MarketNotEndedYet,

    #[msg("Market has not been resolved yet")]
    MarketNotResolved,

    #[msg("Winnings have already been claimed")]
    AlreadyClaimed,

    #[msg("User did not bet on the winning side")]
    NotAWinner,

    #[msg("Invalid vault token account")]
    InvalidVault,

    #[msg("Betting on an invalid side")]
    InvalidBetSide,

    #[msg("Insufficient amount to bet")]
    InsufficientBetAmount,

    #[msg("Unauthorized action")]
    Unauthorized,
}
