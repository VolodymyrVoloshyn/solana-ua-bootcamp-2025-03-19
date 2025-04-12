import "dotenv/config";
import {
    getExplorerLink,
} from "@solana-developers/helpers";
import {
    Keypair,
    clusterApiUrl,
    Connection,
    PublicKey,
} from "@solana/web3.js";
import {
    getOrCreateAssociatedTokenAccount,
}
    from "@solana/spl-token";

const privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
    console.error("Add SECRET_KEY to .env");
    process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));
console.log("Our public key is", sender.publicKey.toBase58());

const tokenMint = process.env["TOKEN_MINT"];
if (tokenMint === undefined) {
    console.error("Add TOKEN_MINT to .env");
    process.exit(1);
}

const tokenMintAccount = new PublicKey(tokenMint);

const recepientStr = process.env["Phantom_Account1_Public"];
if (recepientStr === undefined) {
    console.error("Add Phantom_Account1_Public to .env");
    process.exit(1);
}

const recepient = new PublicKey(recepientStr);

const tokenAccount = await getOrCreateAssociatedTokenAccount(
    connection,
    sender,
    tokenMintAccount,
    recepient,
);

console.log("Token account: ", tokenAccount.address.toBase58());

const link = getExplorerLink("address", tokenAccount.address.toString(), "devnet");

console.log("Created token account: ", link);
