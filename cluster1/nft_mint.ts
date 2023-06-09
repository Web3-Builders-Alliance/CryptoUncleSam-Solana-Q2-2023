import { Commitment, Connection, Keypair} from "@solana/web3.js"
import wallet from "./wba_wallet.json"
import { Metaplex, keypairIdentity, bundlrStorage } from "@metaplex-foundation/js";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

 // Metaplex connection
 const metaplex = Metaplex.make(connection)
    .use(keypairIdentity(keypair))
    .use(bundlrStorage({ 
            address: 'https://devnet.bundlr.network', 
            providerUrl: "https://api.devnet.solana.com",
            timeout: 60000
        })
    );

(async () => {
    try {
       const mint = await metaplex.nfts().create({
            uri: "https://arweave.net/0nXD4aOHQB-S7Tke476h9BbLcGPNSRAzPl4nKZbp-FM",
            name: "CTS RUG",
            symbol: "CTS",
            creators: [
                {
                  address: keypair.publicKey,
                  share: 100
                }
            ],
            sellerFeeBasisPoints: 420,
            isMutable: true,
       })
            
       console.log(`https://solscan.io/token/${mint.nft.address.toBase58()}?cluster=devnet#metadata`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();