use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};
use crate::state::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct StartGameIx {
    identifier: String, // unique game id
    token: String, // price feed id
    duration: i64, // game session
    initial_amount: u64 // initial vault fund
}

#[derive(Accounts)]
#[instruction(ix: StartGameIx)]
pub struct StartGameCtx<'info> {
    #[account(mut)]
    authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = GAME_SIZE,
        seeds = [
            GAME_PREFIX.as_bytes(),
            ix.identifier.as_ref()
        ],
        bump,
    )]
    game_state: Box<Account<'info, GameState>>,

    #[account(
        mut,
        seeds = [
            b"vault",
            game_state.key().as_ref()
        ],
        bump
    )]
    vault: SystemAccount<'info>,

    price_update: Account<'info, PriceUpdateV2>,

    system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StartGameCtx>, ix: StartGameIx) -> Result<()> {

    let initial_amount = ix.initial_amount;
    let game_state = &mut ctx.accounts.game_state;
    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 90;
    let feed_id: [u8; 32] = get_feed_id_from_hex(&ix.token)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    game_state.game_id = ix.identifier;
    game_state.game_bump = ctx.bumps.game_state;
    game_state.vault = ctx.accounts.vault.key();
    game_state.vault_bump = ctx.bumps.vault;
    game_state.authority = ctx.accounts.authority.key();
    game_state.token = ix.token;
    game_state.start_time = Clock::get()?.unix_timestamp;
    game_state.duration = ix.duration;
    game_state.start_price = (price.price as f64) * 10_f64.powi(price.exponent);
    game_state.long_amount = 0;
    game_state.short_amount = 0;

    // add initial vault fund
    let transfer_instruction = Transfer {
        from: ctx.accounts.authority.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let transfer_ctx = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        transfer_instruction
    );
    
    transfer(transfer_ctx, initial_amount)?;
    
    Ok(())
}