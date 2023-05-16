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
       const {uri} = await metaplex.nfts().uploadMetadata({
            name: "CTS RUG",
            symbol: "CTS",
            description: "Rug of CryptoTioSam",
            image: "https://arweave.net/mzaLieujvi5oCDqh46hov2RN1g8hFg9BZxEaJzeoGIk",
            attributes: [
                {trait_type: 'Feature', value: 'Blue Sea'},
                {trait_type: 'Style', value: 'Pixelated'},
                {trait_type: 'Background', value: 'White'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: "https://arweave.net/mzaLieujvi5oCDqh46hov2RN1g8hFg9BZxEaJzeoGIk",
                    },
                ]
            },
            creators: [
                {
                  address: keypair.publicKey.toBase58(),
                  share: 100
                }
            ]
       })

       console.log(uri);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();