import {
  Connection,
  Keypair,
  PublicKey,
} from "@solana/web3.js";
import {
  mintToChecked
} from "@solana/spl-token";
import wallet from "./wba_wallet.json"

// Import keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

(async () => {
  // Create a devnet connection
  const connection = new Connection("https://api.devnet.solana.com");

  const mintAccountPublicKey = new PublicKey("J9Qip1kFuAH1nxoRbZwVJZS6yfvbR6jm9KcQT3TXwb8J");

  const tokenAccountPublicKey = new PublicKey("3B8tScL8LeH8YQkXssANW4tUj99xSwU9SHpg9FndcvJb");

  let txhash = await mintToChecked(
    connection, // connection
    keypair, // fee payer
    mintAccountPublicKey, // mint
    tokenAccountPublicKey, // receiver (sholud be a token account)
    keypair, // mint authority
    1e8, // amount. if your decimals is 8, you mint 10^8 for 1 token.
    6 // decimals
  );
  console.log(`txhash: ${txhash}`); 

})();

