use anchor_lang::prelude::*;
use instructions::*;
pub mod state;
pub mod errors;
pub mod instructions;

declare_id!("7sBHvSPH2BaNhTEk9nMWyhEwYttbrvC458RLrR4X3rqw");

#[program]
pub mod prediction_game {
    use super::*;
    
    pub fn start_game(ctx: Context<StartGameCtx>, ix: StartGameIx) -> Result<()> {
        instructions::start_game::handler(ctx, ix)
    }
    
    pub fn player_guess(ctx: Context<PlayerGuessCtx>, ix: PlayerGuessIx) -> Result<()> {
        instructions::player_guess::handler(ctx, ix)
    }
    
    pub fn finalize_game(ctx: Context<FinalizeGameCtx>) -> Result<()> {
        instructions::finalize_game::handler(ctx)
    }

    pub fn player_claim(ctx: Context<PlayerClaimCtx>) -> Result<()> {
        instructions::player_claim::handler(ctx)
    }

    pub fn close_game(ctx: Context<CloseGameCtx>, ix: CloseGameIx) -> Result<()> {
        instructions::close_game::handler(ctx,ix)
    }
}