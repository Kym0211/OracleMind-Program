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


}