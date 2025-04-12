import "dotenv/config";
import {
    Connection,
    Keypair,
    PublicKey,
    clusterApiUrl,
} from "@solana/web3.js";
import {
    mintTo,
} from "@solana/spl-token";
import {
    getExplorerLink,
} from "@solana-developers/helpers";


const privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) { 
    console.error("Add SECRET_KEY to .env");
    process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

const MINOR_UNITS_PER_MAJOR_UNIT = Math.pow(10, 2); // 2 decimal places were set when mint was created

const tokenMint = process.env["TOKEN_MINT"];
if (tokenMint === undefined) { 
    console.error("Add TOKEN_MINT to .env");
    process.exit(1);
}
const tokenMintAccount = new PublicKey(tokenMint);

const recepientAssociatedTokenAccountStr = process.env["Recepient_Associated_Token_Account"];
if (recepientAssociatedTokenAccountStr === undefined) { 
    console.error("Add Recepient_Associated_Token_Account to .env");
    process.exit(1);
}
const recepientAssociatedTokenAccount = new PublicKey(recepientAssociatedTokenAccountStr);


const transactionSignature = await mintTo(
    connection,
    sender,
    tokenMintAccount,
    recepientAssociatedTokenAccount,
    sender,
    10 * MINOR_UNITS_PER_MAJOR_UNIT
);

const link= getExplorerLink("transaction", transactionSignature, "devnet");

console.log("Success");
console.log("Mint token Transaction: ", link);
