#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("2qa3SpCoeuSb2FNHoJGx3q9ufeGvri68ahvx5vFi7QFZ");

#[program]
pub mod oraclemind_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
