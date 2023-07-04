use anchor_lang::{prelude::*, system_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

mod structs;
mod contexts;
use contexts::*;
mod cpis;
use cpis::*;

#[program]
pub mod powerpool {
    use super::*;
    //initialize
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        assert!(ctx.accounts.admin.key() == admin_pubkey::id(), "You are not the admin");
        ctx.accounts.deposit_account_state.auth_bump = *ctx.bumps.get("deposit_account_auth").unwrap();
        ctx.accounts.deposit_account_state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.deposit_account_state.admin = *ctx.accounts.admin.key;
        ctx.accounts.deposit_account_state.vault = *ctx.accounts.vault.key;

        ctx.accounts.jackpot_state.jackpot_auth_bump = *ctx.bumps.get("jackpot_account_auth").unwrap();
        ctx.accounts.jackpot_state.jackpot_bump = *ctx.bumps.get("jackpot").unwrap();
        ctx.accounts.jackpot_state.amount = 0;
        ctx.accounts.jackpot_state.last_lottery_slot = Clock::get()?.slot;

        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {

        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer{
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.vault.to_account_info()
            },
        );

        system_program::transfer(cpi, amount)?;

        ctx.accounts.user_state.amount = amount;
        ctx.accounts.user_state.user = ctx.accounts.user.key();
        ctx.accounts.user_state.last_deposit_slot = Clock::get()?.slot;

        let deposit_ix = deposit_ix(
            amount, ctx.accounts.vault.key(), ctx.accounts.liq_owner.key(), 
            ctx.accounts.liquidity_pool.key(), ctx.accounts.deposit.key(), 
            ctx.accounts.system_program.key(), ctx.accounts.rent.to_account_info().key()
        );

        let bump = *ctx.bumps.get("deposit_account_auth").unwrap();
        let seeds = vec![bump];
        let deposit_account_state_key = ctx.accounts.deposit_account_state.key();
        let seeds = vec![
            b"auth".as_ref(), deposit_account_state_key.as_ref(), seeds.as_slice()
        ];
        let seeds = vec![seeds.as_slice()];
        let seeds = seeds.as_slice();

        solana_program::program::invoke_signed(
            &deposit_ix,
            &[
                ctx.accounts.vault.to_account_info().clone(),
                ctx.accounts.liq_owner.to_account_info().clone(),
                ctx.accounts.deposit_account_auth.to_account_info().clone(),
            ],
            seeds,
        )?;

        Ok(())

    }
    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {

        let mut withdraw_amount = amount;

        assert!(ctx.accounts.user_state.amount >= withdraw_amount, "Not enough funds");

        let withdraw_ix = withdraw_ix(
            *ctx.bumps.get("deposit").unwrap(), withdraw_amount,
            ctx.accounts.liq_owner.key(), ctx.accounts.vault.key(), 
            ctx.accounts.liquidity_pool.key(), ctx.accounts.deposit.key(),
            ctx.accounts.system_program.key(), ctx.accounts.rent.to_account_info().key()
        );
        
        let bump = *ctx.bumps.get("deposit_account_auth").unwrap();
        let seeds = vec![bump];
        let deposit_account_state_key = ctx.accounts.deposit_account_state.key();
        let seeds = vec![
            b"auth".as_ref(), deposit_account_state_key.as_ref(), seeds.as_slice()
        ];
        let seeds = vec![seeds.as_slice()];
        let seeds = seeds.as_slice();

        solana_program::program::invoke_signed(
            &withdraw_ix,
            &[
                ctx.accounts.liq_owner.to_account_info().clone(),
                ctx.accounts.vault.to_account_info().clone(),
                ctx.accounts.deposit_account_auth.to_account_info().clone(),
            ],
            seeds,
        )?;
        if ctx.accounts.user_state.last_deposit_slot+1512000>Clock::get()?.slot{
            //Penalize 2%
            withdraw_amount = withdraw_amount*98/100;

        }
        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer{
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.user.to_account_info()
            },
        );

        system_program::transfer(cpi, withdraw_amount)?;

        ctx.accounts.user_state.amount -= amount;
        ctx.accounts.user_state.user = ctx.accounts.user.key();

        let cpi_penalize = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer{
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.admin.to_account_info()
            },
        );

        system_program::transfer(cpi_penalize, amount-withdraw_amount)?;
        
        Ok(())

    }
    pub fn run_lottery(ctx: Context<RunLottery>) -> Result<()> {
        // harvest_ix(bump, from, to, liquidity_pool, deposit, system_p, rent)
        Ok(())
    }
    
   
    
}