#![allow(unexpected_cfgs, deprecated)]
use anchor_lang::prelude::*;

declare_id!("2qa3SpCoeuSb2FNHoJGx3q9ufeGvri68ahvx5vFi7QFZ");

pub mod instructions;
pub mod state;
pub mod error;
pub mod constants;

pub use instructions::*;

#[program]
pub mod oraclemind_program {
    use super::*;

    pub fn create_market(
        ctx: Context<CreateMarket>, 
        title: String,
        end_time: i64,
    ) -> Result<()> {
        ctx.accounts.create_market_handler(title, end_time, &ctx.bumps)
    }

    pub fn place_bet(
        ctx: Context<PlaceBet>,  
        bet_side: bool, 
        amount: u64,
    ) -> Result<()> {
        ctx.accounts.place_bet_handler(bet_side, amount, &ctx.bumps)
    }

    pub fn resolve_market(
        ctx: Context<ResolveMarket>,  
        result: bool,
    ) -> Result<()> {
        ctx.accounts.resolve_market_handler(result)
    }

    pub fn claim_winnings(ctx: Context<ResolveMarket>, result: bool) -> Result<()> {
        ctx.accounts.resolve_market_handler(result)
    }


}