use anchor_lang::prelude::*;

pub const GAME_SIZE: usize = 8 + std::mem::size_of::<GameState>() + 64;
pub const GAME_PREFIX: &str = "game_state";
pub const PLAYER_SIZE: usize = 8 + std::mem::size_of::<PlayerState>() + 64; 
pub const PLAYER_PREFIX: &str = "player_state";

#[account]
pub struct GameState {
    pub game_id: String,
    pub game_bump: u8,
    pub vault: Pubkey, // system account
    pub vault_bump: u8,
    pub authority: Pubkey,
    pub token: String, // price feed id
    pub start_time: i64,
    pub duration: i64,
    pub start_price: f64,
    pub end_price: Option<f64>,
    pub long_amount: u64,
    pub short_amount: u64,
}


#[account]
pub struct PlayerState {
    pub player_id: String,
    pub player_bump: u8,
    pub game_state: Pubkey,
    pub guess: bool, // true for long and false for short
    pub bet_time: i64,
    pub bet_amount: u64,
    pub reward_amount: u64,
    pub reward_claimed: bool,
}
