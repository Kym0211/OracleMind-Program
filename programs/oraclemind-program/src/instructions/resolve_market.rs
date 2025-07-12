use anchor_lang::prelude::*;

use crate::{error::Errors, state::Market};

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    
    #[account( 
        mut,
        constraint = market_account.creator == signer.key()
    )]
    pub market_account: Account<'info, Market>,
}

impl<'info> ResolveMarket<'info> {
    pub fn resolve_market_handler(&mut self, result: bool) -> Result<()> {
        let time_stamp = Clock::get()?.unix_timestamp;
        let end_time = self.market_account.end_time;
        let is_resolved = self.market_account.is_resolved;

        require!(time_stamp >= end_time, Errors::MarketNotEndedYet);

        require!(!is_resolved, Errors::MarketAlreadyResolved);

        self.market_account.is_resolved = true;
        self.market_account.result = Some(result);

        Ok(())
    }
}