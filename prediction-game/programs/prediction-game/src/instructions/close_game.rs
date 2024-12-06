use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CloseGameIx {
    reclaim_amount: u64,
}

#[derive(Accounts)]
pub struct CloseGameCtx<'info> {
    #[account(
        mut,
        constraint = authority.key() == game_state.authority
    )]
    authority: Signer<'info>,
    
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

pub fn handler(ctx: Context<CloseGameCtx>, ix: CloseGameIx) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;

    require!(
        game_state.end_price.is_some(),
        ErrorCode::GameNotFinalized
    );

    let vault_seeds = &[
        b"vault",
        game_state.to_account_info().key.as_ref(),
        &[ctx.accounts.game_state.vault_bump],
    ];
    let signer_seeds = &[&vault_seeds[..]];

    // transfer to authority
    let transfer_instruction = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.authority.to_account_info(),
    };

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction,
        signer_seeds,
    );

    transfer(transfer_ctx, ix.reclaim_amount)?;

    Ok(())
}
