use anchor_lang::prelude::*;
use crate::structs::DepositAccount;
use crate::structs::Jackpot;

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(init, payer=admin, space=DepositAccount::LEN, has_one = admin)]
    pub deposit_account_state: Account<'info, DepositAccount>,
    /// CHECK
    #[account(seeds = [b"auth", deposit_account_state.key().as_ref()], bump)]
    pub deposit_account_auth : UncheckedAccount<'info>,
    #[account(seeds = [b"vault", deposit_account_auth.key().as_ref()], bump)]
    pub vault: SystemAccount <'info>,
    #[account(init, payer=admin, space=Jackpot::LEN)]
    pub jackpot_state: Account<'info, Jackpot>,
    /// CHECK
    #[account(seeds = [b"auth", jackpot_state.key().as_ref()], bump)]
    pub jackpot_account_auth : UncheckedAccount<'info>,
    #[account(seeds = [b"jackpot", jackpot_account_auth.key().as_ref()], bump)]
    pub jackpot: SystemAccount <'info>,
    pub system_program: Program<'info, System>,
    
}