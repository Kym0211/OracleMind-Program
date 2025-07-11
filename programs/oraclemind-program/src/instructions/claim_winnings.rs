use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, Transfer, transfer};

use crate::{error::Errors, state::{Bettor, Market}};

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market_account:Account<'info, Market>,

    #[account(
        mut,
        seeds = [b"bettor", market_account.key().as_ref(), signer.key().as_ref()],
        bump = bettor.bump,
        constraint = bettor.market == market_account.key()
    )]
    pub bettor: Account<'info, Bettor>,

    #[account(mut)]
    pub bettor_ata: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

impl<'info> ClaimWinnings<'info> {
    pub fn claim_winnings_handler(&mut self) -> Result<()> {

        let market = &mut self.market_account;
        let bettor = &mut self.bettor;

        require!(market.is_resolved, Errors::MarketNotResolved);
        require!(!bettor.is_claimed, Errors::AlreadyClaimed);

        let won = if market.result.unwrap() {
            bettor.bet_side == true && market.yes_amount > 0
        } else {
            bettor.bet_side == false && market.no_amount > 0
        };

        require!(won, Errors::NotAWinner);

        // payout calculations
        let total_pool = market.yes_amount + market.no_amount;
        let winning_pool = if market.result.unwrap() {
            market.yes_amount
        } else {
            market.no_amount
        };

        let protocol_fee = total_pool / 100; //1% fee for testing
        let distributing_pool = total_pool - protocol_fee;

        let payout = bettor.amount * (distributing_pool / winning_pool);

        //transfer funds
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from: self.vault.to_account_info(),
            to: self.bettor_ata.to_account_info(),
            authority: market.to_account_info()
        };

        let market_key = market.key();

        let seeds = [b"vault", market_key.as_ref(), &[market.bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer(cpi_ctx, payout)?;

        bettor.is_claimed = true;

        Ok(())
    }
}