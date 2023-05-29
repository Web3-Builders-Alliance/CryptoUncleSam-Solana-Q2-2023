import { Commitment, Connection, Keypair, PublicKey, Transaction, sendAndConfirmTransaction, SystemProgram} from "@solana/web3.js"
import wallet from "./wba_wallet.json"
import { createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Define our Mint address
const mint = new PublicKey("J9Qip1kFuAH1nxoRbZwVJZS6yfvbR6jm9KcQT3TXwb8J")

// Add the Token Metadata Program
const token_metadata_program_id = new PublicKey('metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s')

// Create PDA for token metadata
const metadata_seeds = [
    Buffer.from('metadata'),
    token_metadata_program_id.toBuffer(),
    mint.toBuffer(),
];
const [metadata_pda, _bump] = PublicKey.findProgramAddressSync(metadata_seeds, token_metadata_program_id);

(async () => {
    try {
        // Start here
        const metadaAccount = createCreateMetadataAccountV3Instruction(
            {
              metadata: metadata_pda,
              mint: mint,
              mintAuthority:keypair.publicKey,
              payer: keypair.publicKey,
              updateAuthority: keypair.publicKey,
              systemProgram: SystemProgram.programId,
            },
            {createMetadataAccountArgsV3: 
              {
                data:{
                  name: "CTS coin",
                  symbol: "CTS",
                  uri: "",
                  sellerFeeBasisPoints: 0,
                  creators: null,
                  collection: null,
                  uses: null

                },
                isMutable: true, 
                collectionDetails: null
              }
            }
        );
        const tx = new Transaction().add(metadaAccount);
        const txhash = await sendAndConfirmTransaction(
            connection,
            tx,
            [keypair]
        );
        console.log(`Success! Check out your TX here: https://solscan.io/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();