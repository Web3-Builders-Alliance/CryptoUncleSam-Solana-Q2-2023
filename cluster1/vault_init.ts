import { Connection, Keypair, SystemProgram, PublicKey } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@project-serum/anchor"
import { vaultProgram, IDL } from "./programs/wba_vault";
import wallet from "./wba_wallet.json"

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed"});

// Program address
const programAddres = new PublicKey("D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o");

// Create program
const program = new Program<vaultProgram>(IDL, programAddres, provider);

// Create the PDA for vault state account
const vaultState = Keypair.generate();
console.log(`Vault state account: ${vaultState.publicKey.toBase58()}`);
//3SyvD3xBEpyZXQeRTkTLK3tLxUaVU6VnGEKa9FKzWt34

// Create the PDA for vault account
const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
const vault_auth_seeds_key = PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

// Create the PDA for vault account
const vault_seeds = [Buffer.from("vault"), vault_auth_seeds_key.toBuffer()];
const vault_seeds_key = PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

// Execute enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
        .initialize()
        .accounts({
            owner: keypair.publicKey,
            vaultState: vaultState.publicKey,
            vaultAuth: vault_auth_seeds_key,
            vault: vault_seeds_key,
            systemProgram: SystemProgram.programId,
        })
        .signers([
            keypair,
            vaultState,
        ]).rpc();
        console.log(`Success! Check out your TX here: 
        https://solscan.io/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();