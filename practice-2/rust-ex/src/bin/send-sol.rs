use serde_json::from_str;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    signer::{Signer, keypair::Keypair},
    system_instruction,
    transaction::Transaction,
};
use spl_memo::build_memo;
use std::error::Error;

fn check_balance(rpc_client: &RpcClient, address: &Pubkey) -> Result<u64, Box<dyn Error>> {
    match rpc_client.get_balance(&address) {
        Ok(balance) => {
            println!("Balance of {} is {} lamports", address.to_string(), balance);
            Ok(balance)
        }
        Err(err) => {
            eprintln!("Error getting balance: {}", err);
            Err(Box::new(err))
        }
    }
}

fn main() {
    dotenv::dotenv().ok();

    let private_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let as_array: Vec<u8> = from_str(&private_key).expect("Failed to parse private key");
    //let as_array: Vec<u8> = from_str(&private_key).unwrap();

    let sender: Keypair =
        Keypair::from_bytes(&as_array).expect("Failed to create Keypair from bytes");

    println!("Sender public key is: {:?}", sender.pubkey().to_string());

    let rpc_client: RpcClient = RpcClient::new("https://api.devnet.solana.com");

    let recepient_address: String =
        dotenv::var("Phantom_Account1_Public").expect("Recipient address must be set");

    let recepient: Pubkey = recepient_address
        .parse()
        .expect("Failed to parse recipient address");

    check_balance(&rpc_client, &recepient).expect("Failed to check recipient balance");

    println!("Attempting to send 0.2 SOL to: {:?}", recepient.to_string());

    let send_sol_instruction = system_instruction::transfer(
        &sender.pubkey(),
        &recepient,
        (0.2 * LAMPORTS_PER_SOL as f64) as u64, // 0.2 SOL in lamports
    );

    let memo_text: String = String::from("Hello from Solana Rust!");
    let memo_instruction = build_memo(memo_text.as_bytes(), &[&sender.pubkey()]);

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

    let mut transaction: Transaction = Transaction::new_signed_with_payer(
        &[memo_instruction, send_sol_instruction],
        Some(&sender.pubkey()),
        &[sender],
        blockhash,
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction confirmed, signature: {:?}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    check_balance(&rpc_client, &recepient).expect("Failed to check recipient balance");
}
