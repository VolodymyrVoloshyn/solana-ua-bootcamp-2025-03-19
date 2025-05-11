use serde_json::from_str;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token_2022::{id, instruction::mint_to};

fn main() {
    dotenv::dotenv().ok();

    let private_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let as_array: Vec<u8> = from_str(&private_key).expect("Failed to parse private key");

    let sender = Keypair::from_bytes(&as_array).expect("Failed to create Keypair from bytes");

    println!("Sender public key is: {:?}", sender.pubkey());

    let mint_key: String = dotenv::var("TOKEN_MINT_RUST").expect("TOKEN_MINT_RUST must be set");

    let mint: Pubkey = mint_key.parse().expect("Failed to parse mint address");

    println!("Mint public key is: {:?}", mint);

    let token_account_key: String = dotenv::var("Recepient_Associated_Token_Account_Rust")
        .expect("Associated Token Account address must be set");

    let token_account: Pubkey = token_account_key
        .parse()
        .expect("Failed to parse Associated Token Account address");

    println!(
        "Associated Token Account public key is: {:?}",
        token_account
    );

    let mint_to_instruction = mint_to(
        &id(),
        &mint,
        &token_account,
        &sender.pubkey(),
        &[&sender.pubkey()],
        10u64.pow(4), // 4 decimals
    );

    let rpc_client: RpcClient = RpcClient::new(String::from("https://api.devnet.solana.com"));

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[mint_to_instruction.unwrap()],
        Some(&sender.pubkey()),
        &[&sender],
        blockhash,
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Transaction confirmed, signature: {:?}", signature);
            println!(
                "Mind token transaction link: https://explorer.solana.com/tx/{:?}?cluster=devnet",
                signature
            );
        }
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    
}
