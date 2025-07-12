use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, Token, TokenAccount}
};

use crate::{
    constants::MARKET_MAKER, 
    state::Market,
    error::Errors
};

#[derive(Accounts)]
#[instruction(title: String)]
pub struct CreateMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        seeds = [b"market-account", title.as_bytes()],
        bump,
        space = 8 + Market::INIT_SPACE
    )]
    pub market_account: Account<'info, Market>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = market_account,
        associated_token::token_program = token_program
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint: Account<'info, Mint>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateMarket<'info> {
    pub fn create_market_handler( 
        &mut self,
        title: String,
        end_time: i64,
        bumps: &CreateMarketBumps
    ) -> Result<()> {

        // require!(self.signer.key().to_string() == MARKET_MAKER, Errors::InvalidMarketMaker);
        
        self.market_account.set_inner(Market { 
            creator: self.signer.key(), 
            title,
            yes_amount: 0, 
            no_amount: 0, 
            yes_count: 0, 
            no_count: 0, 
            is_resolved: false, 
            result: None, 
            vault: self.vault.key(), 
            end_time, 
            mint: self.mint.key(), 
            bump: bumps.market_account,
            ai_insight: None
        });

        Ok(())
    }
}