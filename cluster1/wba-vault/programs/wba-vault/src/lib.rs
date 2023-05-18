use anchor_lang::prelude::*;

declare_id!("ANWZywNbnqNqrXxz2Vv79QLtDqW4BQL4iUyaz26whbdX");

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.vault_state.score = 0;
        ctx.accounts.vault_state.bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.vault_state.owner = *ctx.accounts.owner.key;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(init, payer=owner, space=Vault::LEN)]
    pub vault_state: Account<'info, Vault>,
    /// CHECK
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    pub vault_auth : UncheckedAccount<'info>,
    #[account(seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault: SystemAccount <'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Vault {
    pub owner: Pubkey,
    pub bump: u8,
    pub auth_bump: u8,
    pub score: u8,
}

impl Vault {
    const LEN: usize =8 + 32 + 1 + 1 + 1;
}