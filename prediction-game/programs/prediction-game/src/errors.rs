use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Game has been finalized.")]
    GameFinalized,  // Error code 6000
    
    #[msg("Player joined the game already.")]
    PlayerJoined,  // Error code 6001

    #[msg("Game still in duration.")]
    GameInDuration,  // Error code 6002

    #[msg("Game has not been finalized.")]
    GameNotFinalized,  // Error code 6003

    #[msg("No reward to claim.")]
    NoReward,  // Error code 6004
}