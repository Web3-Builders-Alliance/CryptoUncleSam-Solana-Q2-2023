use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, CloseAccount, Mint, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {

    use anchor_spl::token::transfer;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        escrow_seed: u64,
        deposit_amount: u64,
        taker_amount: u64,
    ) -> Result<()> {

        ctx.accounts.escrow_state.maker = *ctx.accounts.maker.key;
        ctx.accounts.escrow_state.vault_auth_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.escrow_state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.escrow_state.escrow_state_bump = *ctx.bumps.get("escrow_state").unwrap();
        ctx.accounts.escrow_state.taker_amount = taker_amount;
        ctx.accounts.escrow_state.escrow_seed = escrow_seed;
        ctx.accounts.escrow_state.maker_ata = *ctx.accounts.maker_ata.to_account_info().key;
        ctx.accounts.escrow_state.maker_receive_ata = *ctx.accounts.maker_receive_ata.to_account_info().key;

        let cpi_context = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.maker_ata.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.maker.to_account_info(),
            },
        );

        transfer(cpi_context, deposit_amount);
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> Result<()> {

        let seeds = &[
            b"authority",
            &[*ctx.accounts.escrow_state.vault_auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_context = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.maker_ata.to_account_info(),
                authority: ctx.accounts.vault_auth.to_account_info(),
            },
            signer_seeds,
        );
        transfer(cpi_context, ctx.accounts.vault.amount);
        Ok(())
    }

    pub fn exchange(_ctx: Context<Exchange>) -> Result<()> {
        

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(escrow_seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = maker)]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(seeds=[b"authority".as_ref()], bump)]
    pub vault_auth: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        init, 
        payer = maker, 
        seeds=[b"vault", escrow_seed.to_le_bytes().as_ref(), escrow_state.key.as_ref()], 
        bump,
        token::mint = mint,
        token::authority = vault_auth,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub maker_receive_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        init, 
        payer = maker, 
        seeds=[b"state".as_ref(), escrow_seed.to_le_bytes().as_ref(), maker.key.as_ref()], 
        bump,
        space = EscrowState::LEN,
    )]
    pub escrow_state: Account<'info, EscrowState>,
}

#[derive(Accounts)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = maker)]
    pub maker_ata: Account<'info, TokenAccount>,
    #[account(seeds=[b"authority".as_ref()], bump = escrow_state.vault_auth_bump)]
    pub vault_auth: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[b"vault", escrow_state.escrow_seed.to_le_bytes().as_ref(), escrow_state.key.as_ref()], 
        bump,
        token::mint = mint,
        token::authority = vault_auth,
    )]
    pub vault: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    #[account(
        mut,
        seeds=[b"state".as_ref(), escrow_seed.to_le_bytes().as_ref(), maker.key.as_ref()], 
        bump,
        close = maker,
        has_one = maker,
        has_one = maker_ata,
    )]
    pub escrow_state: Account<'info, EscrowState>,
}
          
#[derive(Accounts)]
pub struct Exchange<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut, associated_token::mint = mint, associated_token::authority = taker)]
    pub taker_ata: Account<'info, TokenAccount>,
    #[account(seeds=[b"authority".as_ref()], bump = escrow_state.vault_auth_bump)]
    pub vault_auth: UncheckedAccount<'info>,
    pub mint: Account<'info, Mint>,
    #[account(
        seeds=[b"vault", escrow_state.escrow_seed.to_le_bytes().as_ref(), escrow_state.key.as_ref()], 
        bump,
        token::mint = mint,
        token::authority = vault_auth,
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"state".as_ref(), escrow_seed.to_le_bytes().as_ref(), maker.key.as_ref()], 
        bump,
        close = maker,
        has_one = maker,
        has_one = maker_ata,
    )]
    pub escrow_state: Account<'info, EscrowState>,
    #[account(mut)]
    pub taker_receive_ata: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub taker_receive_ata: Account<'info, TokenAccount>,
}

#[account]
pub struct EscrowState {
    pub maker: Pubkey,
    pub vault_auth_bump: u8,
    pub vault_bump: u8,
    pub maker_ata: Pubkey,
    pub escrow_seed: u64,
    pub maker_receive_ata: Pubkey,
    pub taker_amount: u64,
    pub escrow_state_bump: u8,
}

impl EscrowState {
    //u64 - 8 bytes
    //pubkey is 32 bytes
    pub const LEN: usize = 8 + 32*3 + 3 + 8*2;
}
