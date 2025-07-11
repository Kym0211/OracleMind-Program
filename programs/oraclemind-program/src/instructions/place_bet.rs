use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, Token, TokenAccount, Transfer, transfer}
};

use crate::{
    error::Errors, 
    state::{Bettor, Market}
};

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account( 
        mut,
        seeds = [b"market-account", market_account.title.as_ref()],
        bump = market_account.bump
    )]
    pub market_account: Account<'info, Market>,

    #[account(
        init,
        payer = signer,
        seeds = [b"bettor", market_account.key().as_ref(), signer.key().as_ref()],
        bump,
        space = 8 + Bettor::INIT_SPACE
    )]
    pub bettor: Account<'info, Bettor>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program
    )]
    pub bettor_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
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
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet_handler(
        &mut self, 
        bet_side: bool, 
        amount: u64, 
        bumps: &PlaceBetBumps
    ) -> Result<()> {
        let clock = Clock::get()?;
        let time_stamp = clock.unix_timestamp;
        let end_time = self.market_account.end_time;
        let is_resolved = self.market_account.is_resolved;

        let bettor_ata_amount = self.bettor_ata.amount;

        require!(time_stamp < end_time, Errors::MarketEndTimeExceeded);

        require!(is_resolved == false, Errors::MarketAlreadyResolved);

        require!(amount != 0, Errors::Unauthorized);

        require!(bettor_ata_amount >= amount, Errors::InsufficientBetAmount);

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer { 
            from: self.bettor_ata.to_account_info(), 
            to: self.vault.to_account_info(), 
            authority: self.signer.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, amount)?;

        if bet_side {
            self.market_account.yes_count += 1;
            self.market_account.yes_amount += amount;
        } else {
            self.market_account.no_count += 1;
            self.market_account.no_amount += amount;
        }

        self.bettor.set_inner(Bettor { 
            user: self.signer.key(), 
            market: self.market_account.key(), 
            amount, 
            bet_side, 
            is_claimed: false, 
            created_at: time_stamp, 
            bump: bumps.bettor
        });
        
        Ok(())
    }
}