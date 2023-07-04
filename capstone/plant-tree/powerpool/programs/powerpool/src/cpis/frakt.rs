use solana_program::{pubkey::Pubkey, instruction::{Instruction, AccountMeta}};

pub mod admin_pubkey {
    use anchor_lang::declare_id;
    declare_id!("tiosTcRdt9TW7baDB3BLL3LY16w5pP5XsTbeQNZJKjD");    
}

pub mod frakt_admin_pubkey {
    use anchor_lang::declare_id;
    declare_id!("9aTtUqAnuSMndCpjcPosRNf3fCkrTQAV8C8GERf3tZi3");    
}

pub mod nft_lending_v2 {
    use anchor_lang::declare_id;
    declare_id!("A66HabVL3DzNzeJgcHYtRRNW1ZRMKwBfrdSR4kLsZ9DJ");    
}

pub fn deposit_ix(
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

pub fn withdraw_ix(bump: u8, amount: u64, from: Pubkey, to: Pubkey, 
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

pub fn harvest_ix(bump: u8, from: Pubkey, to: Pubkey, 
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
