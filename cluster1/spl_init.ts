import {
    Connection,
    Keypair,
} from "@solana/web3.js";
import {
    createMint,
} from "@solana/spl-token";
import wallet from "./wba_wallet.json"

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

  (async () => {
    // Create a devnet connection
    const connection = new Connection("https://api.devnet.solana.com");
  
  
    // 1) use build-in function
    let mintPubkey = await createMint(
      connection, // conneciton
      keypair, // fee payer
      keypair.publicKey, // mint authority
      keypair.publicKey, // freeze authority (you can use `null` to disable it. when you disable it, you can't turn it on again)
      6 // decimals
    );
    
    console.log(`mint: ${mintPubkey.toBase58()}`);    
  
  })();