use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub user: Pubkey,
    pub amount: u64,
    pub last_deposit_slot: u64,
}

impl User {
    pub const LEN: usize =8 + 32 + 3;
}

#[account]
pub struct Jackpot {
    pub amount: u64,
    pub last_lottery_slot: u64,
    pub jackpot_bump: u8,
    pub jackpot_auth_bump: u8,
}

impl Jackpot {
    pub const LEN: usize =8 + 3*2 + 2;
}

#[account]
pub struct DepositAccount {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub vault_bump: u8,
    pub auth_bump: u8,
}

impl DepositAccount {
    pub const LEN: usize =8 + 32 + 1 + 1;
}


