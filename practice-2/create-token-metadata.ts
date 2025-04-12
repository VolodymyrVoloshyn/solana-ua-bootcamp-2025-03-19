import "dotenv/config";
import {
    Connection,
    Keypair,
    clusterApiUrl,
    PublicKey,
    sendAndConfirmTransaction,
    Transaction
} from "@solana/web3.js";
import {
    getExplorerLink,
} from "@solana-developers/helpers";
import { collectInstructionDiscriminator, createCreateMetadataAccountV3Instruction } from "@metaplex-foundation/mpl-token-metadata";

const privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
    console.error("Add SECRET_KEY to .env");
    process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const user = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

const TOKEN_METADATA_PROGRAM_ID = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);
const tokenMint = process.env["TOKEN_MINT"];
if (tokenMint === undefined) {
    console.error("Add TOKEN_MINT to .env");
    process.exit(1);
}
const tokenMintAccount = new PublicKey(tokenMint);

const metadataData = {
    name: "VVO Solana UA bootcamp 2025-04",
    symbol: "VVO",
    uri: "https://vvotoken.org",
    sellerFeeBasisPoints: 0,
    creators: null,
    collection: null,
    uses: null,
};

const [metadataPDA, _metadataBump] = await PublicKey.findProgramAddressSync(
    [
        Buffer.from("metadata"),
        TOKEN_METADATA_PROGRAM_ID.toBuffer(),
        tokenMintAccount.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
);

const transaction = new Transaction();
const createMetadataAccountInstruction = createCreateMetadataAccountV3Instruction(
    {
        metadata: metadataPDA,
        mint: tokenMintAccount,
        mintAuthority: user.publicKey,
        payer: user.publicKey,
        updateAuthority: user.publicKey,
    },
    {
        createMetadataAccountArgsV3: {
            collectionDetails: null,
            data: metadataData,
            isMutable: true,
        },
    }
);

transaction.add(createMetadataAccountInstruction);

await sendAndConfirmTransaction(
    connection,
    transaction,
    [user]
);

const tokenMintLink = getExplorerLink(
    "address",
    tokenMintAccount.toString(),
    "devnet"
);

console.log("Look at the token mint again: ", tokenMintLink);
