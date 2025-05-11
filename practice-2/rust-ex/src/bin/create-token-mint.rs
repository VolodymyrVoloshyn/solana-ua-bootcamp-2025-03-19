use serde_json::from_str;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    program_pack::Pack,
    system_instruction::create_account,
    signature::{Signer, Keypair},
    transaction::Transaction,
};
use spl_token_2022::{id, instruction::initialize_mint, state::Mint};

fn main() {
    dotenv::dotenv().ok();

    let private_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let as_array: Vec<u8> = from_str(&private_key).expect("Failed to parse private key");
    //let as_array: Vec<u8> = from_str(&private_key).unwrap();

    let sender = Keypair::from_bytes(&as_array).expect("Failed to create Keypair from bytes");

    println!("Sender public key is: {:?}", sender.pubkey().to_string());

    let mint = Keypair::new();

    println!("Mint public key is: {:?}", mint.pubkey().to_string());

    let rpc_client: RpcClient = RpcClient::new(String::from("https://api.devnet.solana.com"));

    let token_program_id = &id();
    let mint_space = Mint::LEN;
    let rent = rpc_client.get_minimum_balance_for_rent_exemption(mint_space);

    let create_account_instruction = create_account(
        &sender.pubkey(),
        &mint.pubkey(),
        rent.unwrap(),
        mint_space as u64,
        token_program_id,
    );

    let initialize_mint_instruction = initialize_mint(
        token_program_id,
        &mint.pubkey(),
        &sender.pubkey(),
        Some(&sender.pubkey()),
        4,
    );

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

    let transaction = Transaction::new_signed_with_payer(
        &[
            create_account_instruction,
            initialize_mint_instruction.unwrap(),
        ],
        Some(&sender.pubkey()),
        &[&sender, &mint],
        blockhash,
    );

    let tx = rpc_client.send_and_confirm_transaction(&transaction);

    println!("Mint address: {}", mint.pubkey());
    println!("Transaction signature: {:?}", tx);
}
