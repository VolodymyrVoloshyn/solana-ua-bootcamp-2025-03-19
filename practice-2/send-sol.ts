import "dotenv/config";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    Transaction,
    clusterApiUrl,
    Connection,
    sendAndConfirmTransaction,
    TransactionInstruction,
} from "@solana/web3.js";

let privatKey = process.env["SECRET_KEY"];
if (privatKey === undefined) {
    console.error("SECRETE_KEY is not defined");
    process.exit(1);
}

const asArray = Uint8Array.from(JSON.parse(privatKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

console.log("Our public key: ", sender.publicKey.toBase58());

const recipientAddress = process.env["Phantom_Account1_Public"];

if (recipientAddress === undefined) {
    console.error("Recipient address is not defined");
    process.exit(1);
}

const recipient = new PublicKey(recipientAddress);

console.log("Attempting to send 0.2 SOL to ", recipient.toBase58(), " ...");

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
    fromPubkey: sender.publicKey,
    toPubkey: recipient,
    lamports: 0.2 * LAMPORTS_PER_SOL,
});
transaction.add(sendSolInstruction);

// https://spl.solana.com/memo
const memoProgram= new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
const memoText = "Hello from Solana!";

const addMemoInstruction = new TransactionInstruction({
    keys: [{pubkey: sender.publicKey, isSigner: true, isWritable: false}],
    data: Buffer.from(memoText, "utf-8"),
    programId: memoProgram,
});
transaction.add(addMemoInstruction);

console.log("Memo is : ", memoText);

const signature = await sendAndConfirmTransaction(connection, transaction, [sender]);

console.log("Transaction confirmed, signature: ", signature);
