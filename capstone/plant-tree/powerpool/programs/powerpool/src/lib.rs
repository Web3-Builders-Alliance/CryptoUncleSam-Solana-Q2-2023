use anchor_lang::{prelude::*, solana_program::instruction::Instruction, system_program};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub mod nft_lending_v2 {
    use anchor_lang::declare_id;
    declare_id!("A66HabVL3DzNzeJgcHYtRRNW1ZRMKwBfrdSR4kLsZ9DJ");    
}

pub mod admin_pubkey {
    use anchor_lang::declare_id;
    declare_id!("tiosTcRdt9TW7baDB3BLL3LY16w5pP5XsTbeQNZJKjD");    
}

pub mod frakt_admin_pubkey {
    use anchor_lang::declare_id;
    declare_id!("9aTtUqAnuSMndCpjcPosRNf3fCkrTQAV8C8GERf3tZi3");    
}

mod structs;
mod contexts;


#[program]
pub mod powerpool {
    use super::*;
    //initialize
    pub fn initialize(ctx: Context<contexts::Initialize>) -> Result<()> {

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

    pub fn deposit(ctx: Context<contexts::Deposit>, amount: u64) -> Result<()> {
        
        fn deposit_ix(
            amount: u64, from: Pubkey, to: Pubkey, liquidity_pool: Pubkey, 
            deposit: Pubkey, system_p: Pubkey, rent: Pubkey
        ) -> Instruction {
            // Discriminator
            let mut data: Vec<u8> = vec!(0xf5, 0x63, 0x3b, 0x19, 0x97, 0x47, 0xe9, 0xf9);
            data.extend_from_slice(amount.to_le_bytes().as_ref());
        
            let accounts: Vec<AccountMeta> = vec![
                AccountMeta::new(liquidity_pool, false),
                AccountMeta::new(to, false),
                AccountMeta::new(deposit, false),
                AccountMeta::new(from, true),
                AccountMeta::new_readonly(system_p, false),
                AccountMeta::new_readonly(rent, false),
            ];
        
            Instruction {
                program_id: nft_lending_v2::id(),
                accounts,
                data
            }
        }

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
        let seeds = vec![
            b"auth".as_ref(), ctx.accounts.deposit_account_state.key().as_ref(), seeds.as_slice()
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
        );

        Ok(())

    }
    pub fn withdraw(ctx: Context<contexts::Withdraw>, amount: u64) -> Result<()> {

        fn withdraw_ix(bump: u8, amount: u64, from: Pubkey, to: Pubkey, 
            liquidity_pool: Pubkey, deposit: Pubkey, system_p: Pubkey, rent: Pubkey
        ) -> Instruction {
            // Discriminator
            let mut data: Vec<u8> = vec!(0x85, 0x8c, 0xea, 0x9c, 0x92, 0x5d, 0x28, 0xf4, bump);
            data.extend_from_slice(amount.to_le_bytes().as_ref());
        
            let accounts: Vec<AccountMeta> = vec![
                AccountMeta::new(liquidity_pool, false),
                AccountMeta::new(deposit, false),
                AccountMeta::new(to, false),
                AccountMeta::new(from, true),
                AccountMeta::new_readonly(frakt_admin_pubkey::id(), false),
                AccountMeta::new_readonly(system_p, false),
                AccountMeta::new_readonly(rent, false),
            ];
        
            Instruction {
                program_id: nft_lending_v2::id(),
                accounts,
                data
            }
        }

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
        let seeds = vec![
            b"auth".as_ref(), ctx.accounts.deposit_account_state.key().as_ref(), seeds.as_slice()
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
        );
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
                to: ctx.accounts.admin
            },
        );

        system_program::transfer(cpi_penalize, amount-withdraw_amount)?;
        
        Ok(())

    }
    pub fn run_lottery(ctx: Context<contexts::RunLottery>) -> Result<()> {

        fn harvest_ix(bump: u8, from: Pubkey, to: Pubkey, 
            liquidity_pool: Pubkey, deposit: Pubkey, system_p: Pubkey, rent: Pubkey
        ) -> Instruction {
            // Discriminator
            let data: Vec<u8> = vec!(0xd4, 0xd6, 0x21, 0xd3, 0x28, 0x56, 0x09, 0x76, bump);

            let accounts: Vec<AccountMeta> = vec![
                AccountMeta::new(liquidity_pool, false),
                AccountMeta::new(from, true),
                AccountMeta::new(deposit, false),
                AccountMeta::new(to, false),
                AccountMeta::new_readonly(frakt_admin_pubkey::id(), false),
                AccountMeta::new_readonly(system_p, false),
                AccountMeta::new_readonly(rent, false),
            ];

            Instruction {
                program_id: nft_lending_v2::id(),
                accounts,
                data
            }
        }

        Ok(())
    }
    
   
    
}

#[derive(Accounts)]
pub struct Initialize {}
