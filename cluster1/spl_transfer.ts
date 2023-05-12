import {
    Connection,
    Keypair,
    PublicKey,
  } from "@solana/web3.js";
  import {
    transferChecked,
    transfer,
    getOrCreateAssociatedTokenAccount
  } from "@solana/spl-token";
  import wallet from "./wba_wallet.json"
  
  // Import keypair from the wallet file
  const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

  (async () => {
    // Create a devnet connection
    const connection = new Connection("https://api.devnet.solana.com");
    
    // Account of token mint
    const mintAccountPublicKey = new PublicKey("J9Qip1kFuAH1nxoRbZwVJZS6yfvbR6jm9KcQT3TXwb8J");
    
    // Account of the token's owner
    const tokenAccountPublicKeyFrom = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        keypair, // fee payer
        mintAccountPublicKey, // mint
        keypair.publicKey // owner,
    ) 
    
    // Account of the token's receiver
    const tokenAccountPublicKeyTo = await getOrCreateAssociatedTokenAccount(
        connection, // connection
        keypair, // fee payer
        mintAccountPublicKey, // mint
        new PublicKey("DFv93QkrEkfraeJsaXWSuVE6gEtH5tX5ysQxkCJV8MQG") // owner,
    )
    

    let txhash = await transferChecked(
        connection, // connection
        keypair, // payer
        tokenAccountPublicKeyFrom.address, // from (should be a token account)
        mintAccountPublicKey, // mint
        tokenAccountPublicKeyTo.address, // to (should be a token account)
        keypair, // from's owner
        5e7, // amount, if your deciamls is 8, send 10^8 for 1 token
        6 // decimals
      );
      
      console.log(`txhash: ${txhash}`);
  
  })();
  
  