import "dotenv/config";
import {
    Connection,
    LAMPORTS_PER_SOL,
    PublicKey,
    clusterApiUrl,
} from "@solana/web3.js";
import { airdropIfRequired } from "@solana-developers/helpers";

let publicKeyStr = process.env["Phantom_Account1_Public"];

if (publicKeyStr === undefined){
    console.log("Add public key to .env");
    process.exit(1);
}

const connection = new Connection(clusterApiUrl("devnet"));
console.log("Connected to devnet");

const publicKey = new PublicKey(publicKeyStr);
let balanceInLamports= await connection.getBalance(publicKey);

let balanceInSol= balanceInLamports/LAMPORTS_PER_SOL;

console.log(`The ballance at address ${publicKey} is ${balanceInSol}`);

console.log("Adding Sol")

await airdropIfRequired(connection, publicKey, 2 * LAMPORTS_PER_SOL, 1.5 * LAMPORTS_PER_SOL);

balanceInLamports= await connection.getBalance(publicKey);

balanceInSol= balanceInLamports/LAMPORTS_PER_SOL;

console.log(`The ballance after adding Sol at address ${publicKey} is ${balanceInSol}`);
