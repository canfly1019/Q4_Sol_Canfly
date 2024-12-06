use anchor_lang::prelude::*;
use pyth_solana_receiver_sdk::price_update::{PriceUpdateV2, get_feed_id_from_hex};
use crate::state::*;
use crate::errors::ErrorCode;

#[derive(Accounts)]
pub struct FinalizeGameCtx<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    
    #[account(mut, has_one = authority)]
    game_state: Box<Account<'info, GameState>>,

    price_update: Account<'info, PriceUpdateV2>,
}

pub fn handler(ctx: Context<FinalizeGameCtx>) -> Result<()> {
    let game_state = &mut ctx.accounts.game_state;

    require!(
        Clock::get()?.unix_timestamp >= game_state.start_time + game_state.duration, // 改成快速結束 (測試用)
        ErrorCode::GameInDuration
    );

    let price_update = &mut ctx.accounts.price_update;
    let maximum_age: u64 = 90;
    let feed_id: [u8; 32] = get_feed_id_from_hex(&game_state.token)?;
    let price = price_update.get_price_no_older_than(&Clock::get()?, maximum_age, &feed_id)?;

    game_state.end_price = Some((price.price as f64) * 10_f64.powi(price.exponent));

    // output price game result
    msg!("The start price is {}. The end price is {}.", game_state.start_price, game_state.end_price.unwrap());

    Ok(())
}
