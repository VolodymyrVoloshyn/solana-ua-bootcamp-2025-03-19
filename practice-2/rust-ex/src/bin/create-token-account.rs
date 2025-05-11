use serde_json::from_str;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    program_pack::Pack,
    signature::{Keypair, Signer},
    system_instruction::create_account,
    transaction::Transaction,
};
use spl_token_2022::{id, instruction::initialize_account, state::Account};

fn main() {
    dotenv::dotenv().ok();

    let private_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let as_array: Vec<u8> = from_str(&private_key).expect("Failed to parse private key");

    let sender = Keypair::from_bytes(&as_array).expect("Failed to create Keypair from bytes");

    println!("Sender public key is: {:?}", sender.pubkey().to_string());

    let mint_key: String = dotenv::var("TOKEN_MINT_RUST").expect("TOKEN_MINT_RUST must be set");

    let mint: Pubkey = mint_key.parse().expect("Failed to parse mint address");

    println!("Mint public key is: {:?}", mint);

    let token_account = Keypair::new();

    let mint_space = Account::LEN;
    let rpc_client: RpcClient = RpcClient::new(String::from("https://api.devnet.solana.com"));
    let rent = rpc_client.get_minimum_balance_for_rent_exemption(mint_space);

    let create_token_account_instruction = create_account(
        &sender.pubkey(),
        &token_account.pubkey(),
        rent.unwrap(),
        mint_space as u64,
        &id(),
    );

    let initialize_token_account_instruction =
        initialize_account(&id(), &token_account.pubkey(), &mint, &sender.pubkey());

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

    
    let transaction = Transaction::new_signed_with_payer(
        &[
            create_token_account_instruction,
            initialize_token_account_instruction.unwrap(),
        ],
        Some(&sender.pubkey()),
        &[&sender, &token_account],
        blockhash,
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction confirmed, signature: {:?}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }

    println!("Token account address: {}", token_account.pubkey());
    println!("Solana explorer link: https://explorer.solana.com/address/{}?cluster=devnet", token_account.pubkey());
}
