use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct PlayerClaimCtx<'info> {
    #[account(mut)]
    player: Signer<'info>,

    #[account(
        mut,
        seeds = [
            PLAYER_PREFIX.as_bytes(),
            game_state.key().as_ref(),
            player.key().as_ref()
        ],
        bump,
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

pub fn handler(ctx: Context<PlayerClaimCtx>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;
    let player_state = &mut ctx.accounts.player_state;

    require!(
        game_state.end_price.is_some(),
        ErrorCode::GameNotFinalized
    );

    let start_price = game_state.start_price;
    let end_price = game_state.end_price.unwrap();

    let total_amount = game_state.long_amount + game_state.short_amount;

    if end_price > start_price && player_state.guess {
        player_state.reward_amount = player_state.bet_amount * total_amount / game_state.long_amount;
    } else if end_price < start_price && !player_state.guess {
        player_state.reward_amount = player_state.bet_amount * total_amount / game_state.short_amount;
    }
    
    require!(
        player_state.reward_amount > 0,
        ErrorCode::NoReward
    );
    
    let vault_seeds = &[
        b"vault",
        game_state.to_account_info().key.as_ref(),
        &[ctx.accounts.game_state.vault_bump],
    ];

    let signer_seeds = &[&vault_seeds[..]];

    // transfer to player
    let transfer_instruction = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.player.to_account_info(),
    };

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );

    transfer(transfer_ctx, player_state.reward_amount)?;

    player_state.reward_claimed = true;

    Ok(())
}