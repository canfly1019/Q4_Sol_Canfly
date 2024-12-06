use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct PlayerGuessIx {
    pub guess: bool,
    pub bet_amount: u64,
}

#[derive(Accounts)]
#[instruction(ix: PlayerGuessIx)]
pub struct PlayerGuessCtx<'info> {
    #[account(mut)]
    player: Signer<'info>,

    #[account(
        init,
        payer = player,
        space = PLAYER_SIZE,
        seeds = [
            PLAYER_PREFIX.as_bytes(),
            game_state.key().as_ref(),
            player.key().as_ref()
            // &anchor_lang::solana_program::hash::hash(ctx.identifier.as_bytes()).to_bytes()
        ],
        bump
    )]
    player_state: Box<Account<'info, PlayerState>>,

    #[account(mut)]
    game_state: Box<Account<'info, GameState>>,

    #[account(
        mut,
        seeds = [
            b"vault",
            game_state.key().as_ref()
        ],
        bump = game_state.vault_bump
    )]
    vault: SystemAccount<'info>,
    
    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PlayerGuessCtx>, ix: PlayerGuessIx) -> Result<()> {
    let game_state = &ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;
    let current_time = Clock::get()?.unix_timestamp;

    require!(
        current_time >= game_state.start_time && current_time <= game_state.start_time + game_state.duration,
        ErrorCode::GameFinalized
    );

    require!(
        player_state.bet_amount == 0,
        ErrorCode::PlayerJoined
    );

    player_state.player_bump = ctx.bumps.player_state;
    player_state.game_state = ctx.accounts.game_state.key();
    player_state.guess = ix.guess;
    player_state.bet_time = current_time;
    player_state.bet_amount = ix.bet_amount;
    player_state.reward_amount = 0;
    player_state.reward_claimed = false;

    // add long or short amount in game state
    if ix.guess==true {
        ctx.accounts.game_state.long_amount += ix.bet_amount;
    } else if ix.guess==false {
        ctx.accounts.game_state.short_amount += ix.bet_amount;
    }
    
    // add bet amount to vault
    let transfer_instruction = Transfer {
        from: ctx.accounts.player.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
    );

    transfer(transfer_ctx, ix.bet_amount)?;

    Ok(())
}