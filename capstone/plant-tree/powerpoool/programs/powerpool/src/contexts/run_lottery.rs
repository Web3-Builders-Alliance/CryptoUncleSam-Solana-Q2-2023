use anchor_lang::prelude::*;
use crate::structs::User;
use crate::structs::DepositAccount;
use crate::structs::Jackpot;

pub mod nft_lending_v2 {
    use anchor_lang::declare_id;
    declare_id!("A66HabVL3DzNzeJgcHYtRRNW1ZRMKwBfrdSR4kLsZ9DJ");    
}

#[derive(Accounts)]
pub struct RunLottery <'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(init_if_needed, payer=user, space=User::LEN, has_one = user)]
    pub user_state: Account<'info, User>,
    #[account(has_one = vault)]
    pub deposit_account_state: Account<'info, DepositAccount>,
    #[account(
        seeds = [b"auth", deposit_account_state.key().as_ref()], 
        bump = deposit_account_state.auth_bump
    )]
    pub deposit_account_auth : UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault", deposit_account_auth.key().as_ref()], 
        bump = deposit_account_state.vault_bump
    )]
    pub vault: SystemAccount <'info>,
    #[account(mut)]
    pub liquidity_pool: SystemAccount<'info>,
    #[account(
        seeds = [b"nftlendingv2", liquidity_pool.key().as_ref()], 
        bump, 
        seeds::program = nft_lending_v2::id()
    )]
    pub liq_owner: SystemAccount <'info>,
    #[account(
        seeds = [b"deposit", liquidity_pool.key().as_ref(), vault.key().as_ref()], 
        bump, 
        seeds::program = nft_lending_v2::id()
    )]
    pub deposit: SystemAccount <'info>,
    #[account(mut)]
    pub jackpot: Account<'info, Jackpot>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}