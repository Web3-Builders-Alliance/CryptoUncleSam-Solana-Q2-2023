import { Connection, Keypair, SystemProgram, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider, BN } from "@project-serum/anchor"
import { vaultProgram, IDL } from "./programs/wba_vault";
import {transferChecked, transfer, getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID} from "@solana/spl-token";
import wallet from "./wba_wallet.json"

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Account of token mint
const mintAccountPublicKey = new PublicKey("J9Qip1kFuAH1nxoRbZwVJZS6yfvbR6jm9KcQT3TXwb8J");

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Program address
const programAddres = new PublicKey("D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o");

// Create program
const program = new Program<vaultProgram>(IDL, programAddres, provider);

// PDA for vault state account
const vaultState = new PublicKey("3SyvD3xBEpyZXQeRTkTLK3tLxUaVU6VnGEKa9FKzWt34");


// Create the PDA for vault account
const vault_auth_seeds = [Buffer.from("auth"), vaultState.toBuffer()];
const vault_auth_key = PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

// Create the PDA for vault account
const vault_seeds = [Buffer.from("vault"), vault_auth_key.toBuffer()];
const vault_seeds_key = PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

// Execute enrollment transaction
(async () => {
    try {
        // Account of the token's owner
        const ownerAta = await getOrCreateAssociatedTokenAccount(
            connection, // connection
            keypair, // fee payer
            mintAccountPublicKey, // mint
            keypair.publicKey // owner,
        ) 
        // Account of the token's owner
        const vaultAta = await getOrCreateAssociatedTokenAccount(
            connection, // connection
            keypair, // fee payer
            mintAccountPublicKey, // mint
            vault_auth_key, // owner,
            true
        ) 
        const txhash = await program.methods
        .withdrawSpl(new BN(5))
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultState,
            vaultAuth: vault_auth_key,
            systemProgram: SystemProgram.programId,
            ownerAta: ownerAta.address,
            vaultAta: vaultAta.address,
            tokenProgram: TOKEN_PROGRAM_ID,
            tokenMint: mintAccountPublicKey,
        })
        .signers([
            keypair,
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://solscan.io/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();