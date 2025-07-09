use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Market {
    pub creator: Pubkey,
    #[max_len(100)]
    pub title: String,
    pub total_pool: u64,
    pub yes_amount: u64,
    pub no_amount: u64,
    pub yes_count: u64,
    pub no_count: u64,
    pub is_resolved: bool,
    pub result: Option<bool>,
    pub vault: Pubkey,
    pub end_time: i64,
    pub mint: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    pub ai_insight: Option<Pubkey>
}

#[account]
pub struct Bettor{
    pub user: Pubkey,
    pub market: Pubkey,
    pub amount: u64,
    pub bet_side: bool,
    pub is_claimed: bool,
    pub created_at: i64,
    pub bump: u8
}