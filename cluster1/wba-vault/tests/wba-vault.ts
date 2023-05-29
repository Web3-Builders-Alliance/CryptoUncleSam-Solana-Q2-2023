import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { WbaVault, IDL } from "../target/types/wba_vault";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getOrCreateAssociatedTokenAccount, mintToChecked } from "@solana/spl-token";
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";



describe("wba_vault", () => {

  // Configure the client to use the local cluster.
  const keypair = anchor.web3.Keypair.generate();
  const connection = new anchor.web3.Connection("http://localhost:8899");
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = new anchor.AnchorProvider(connection, new anchor.Wallet(keypair), { commitment: "finalized" } );
  
  // Program address
  const programAddres = new anchor.web3.PublicKey("ANWZywNbnqNqrXxz2Vv79QLtDqW4BQL4iUyaz26whbdX");

  // Create program
  const program = new Program<WbaVault>(IDL, programAddres, provider);

  // Create PDA VAULT STATE
  const vaultState =anchor.web3.Keypair.generate();

  // Create PDA VAULT AUTH
  const vault_auth_seeds = [Buffer.from("auth"), vaultState.publicKey.toBuffer()];
  const vault_auth = anchor.web3.PublicKey.findProgramAddressSync(vault_auth_seeds, program.programId)[0];

  // Create Vault system Program
  const vault_seeds = [Buffer.from("vault"), vault_auth.toBuffer()];
  const vault = anchor.web3.PublicKey.findProgramAddressSync(vault_seeds, program.programId)[0];

  it("Airdrop", async () => {
    // 1. Airdrop 100 SOL to payer
    const signature = await provider.connection.requestAirdrop(keypair.publicKey, 100*anchor.web3.LAMPORTS_PER_SOL);
    const latestBlockhash = await connection.getLatestBlockhash();
    await provider.connection.confirmTransaction(
    {
      signature,
      ...latestBlockhash,
    },
    "finalized"
    );  
  })


  it("Is initialized!", async () => {
    try {
      const txhash = await program.methods
      .initialize()
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vault:vault,
        systemProgram: anchor.web3.SystemProgram.programId,

      })
      .signers([
        keypair,
        vaultState

      ]).rpc();   
      console.log(`Init! Check out your TX here: 
      https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch(e){console.error(e)}
  });

  it("Deposit", async () => {
    try {
      const txhash = await program.methods
      .deposit(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL*0.1))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vault:vault,
        systemProgram: anchor.web3.SystemProgram.programId,

      })
      .signers([
        keypair,
      ]).rpc();   
      console.log(`Deposited! Check out your TX here: 
      https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch(e){console.error(e)}
  });

  it("Withdraw", async () => {
    try {
      const txhash = await program.methods
      .withdraw(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL*0.05))
      .accounts({
        owner: keypair.publicKey,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vault:vault,
        systemProgram: anchor.web3.SystemProgram.programId,

      })
      .signers([
        keypair,
      ]).rpc();   
      console.log(`Withdraw! Check out your TX here: 
      https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }
    catch(e){console.error(e)}
  });

  it("Deposit & Withdraw SPL", async () => {
    try {
      // Create mint account
      let mintPubkey = await createMint(
        connection, // conneciton
        keypair, // fee payer
        keypair.publicKey, // mint authority
        keypair.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
        6 // decimals
      );

      // Create ATA account for keypair
      let ata = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        keypair, // fee payer
        mintPubkey, // mint
        keypair.publicKey // owner,
      );

      // Create ATA account for vault
      let ataVault = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        keypair, // fee payer
        mintPubkey, // mint
        vault_auth, // owner,
        true // allowOwnerOffCurve 
      );

      // Mint to ATA account
      let txhashMint = await mintToChecked(
        connection, // connection
        keypair, // fee payer
        mintPubkey, // mint
        ata.address, // receiver (sholud be a token account)
        keypair, // mint authority
        1e6*10, // amount. if your decimals is 8, you mint 10^8 for 1 token.
        6 // decimals
      );

      // Deposit SPL to vault
      const txhashDeposit = await program.methods
      .depositSpl(new anchor.BN(1e6))
      .accounts({
        owner: keypair.publicKey,
        ownerAta: ata.address,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vaultAta: ataVault.address,
        tokenMint: mintPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,

      })
      .signers([
        keypair,
      ]).rpc();   

      console.log(`Deposited SPL! Check out your TX here: 
      https://explorer.solana.com/tx/${txhashDeposit}?cluster=devnet`);

      // Withdraw SPL from vault
      const txhashWithdraw = await program.methods
      .withdrawSpl(new anchor.BN(1e6*0.25))
      .accounts({
        owner: keypair.publicKey,
        ownerAta: ata.address,
        vaultState: vaultState.publicKey,
        vaultAuth: vault_auth,
        vaultAta: ataVault.address,
        tokenMint: mintPubkey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,

      })
      .signers([
        keypair,
      ]).rpc();

      console.log(`Withdraw SPL! Check out your TX here: 
      https://explorer.solana.com/tx/${txhashWithdraw}?cluster=devnet`);

      let vaultTokenAccountBalance = (await provider.connection.getTokenAccountBalance(ataVault.address)).value.amount;
      console.log("vault SPL Balance: ", vaultTokenAccountBalance);
      let tokenAccountBalance = (await provider.connection.getTokenAccountBalance(ata.address)).value.amount;
      console.log("My SPL Balance: ", tokenAccountBalance);
    }
    catch(e){console.error(e)}
  });
  
});

