use mpl_token_metadata::{
    instructions::{CreateMetadataAccountV3Builder, CreateV1Builder},
    types::{DataV2, PrintSupply, TokenStandard},
};
use serde_json::from_str;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    message::Message, signature::{Keypair, Signer}, transaction::Transaction
};
use spl_token_2022::{id, instruction::initialize_account, state::Account};

fn main() {
    dotenv::dotenv().ok();

    let private_key: String = dotenv::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let as_array: Vec<u8> = from_str(&private_key).expect("Failed to parse private key");

    let sender = Keypair::from_bytes(&as_array).expect("Failed to create Keypair from bytes");

    println!("Sender public key is: {:?}", sender.pubkey());

    let mint_key: String = dotenv::var("TOKEN_MINT_RUST").expect("TOKEN_MINT_RUST must be set");

    let mint: Pubkey = mint_key.parse().expect("Failed to parse mint address");

    println!("Mint public key is: {:?}", mint);

     let token_metadata: Pubkey = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
         .parse()
         .expect("Failed to parse Token Metadata address");

    let (metadata_account, metadata_bump) = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            mpl_token_metadata::ID.as_ref(), //&token_metadata.to_bytes(),
            &mint.to_bytes(),
        ],
        &Pubkey::try_from(mpl_token_metadata::ID.as_ref()).unwrap(),
    );

    println!("Metadata account: {:?}", metadata_account);

    let data_v2 = DataV2 {
        name: String::from("VVO Rust Solana UA bootcamp 2025-04"),
        symbol: String::from("VVO"),
        uri: String::from("https://vvotoken.org"),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    // let mut create_metadata_account_builder = CreateMetadataAccountV3Builder::new();
    // create_metadata_account_builder
    //     .metadata(metadata_account)
    //     .mint(mint)
    //     .mint_authority(sender.pubkey())
    //     .payer(sender.pubkey())
    //     .update_authority(sender.pubkey(), true)
    //     .is_mutable(true)
    //     .data(data_v2);

    // let create_metadata_accounts_instruction = create_metadata_account_builder.instruction();

    let create_ix = CreateV1Builder::new()
    .metadata(metadata_account)
    //.master_edition(Some(master_edition))
    .mint(mint, false)
    .authority(sender.pubkey())
    .payer(sender.pubkey())
    .update_authority(sender.pubkey(), true)
    //.spl_token_program(Some(spl_token::id()))
    .name(String::from("VVO Rust Solana UA bootcamp 2025-04"))
    .symbol(String::from("VVO"))
    .uri(String::from("https://vvotoken.org"))
    .seller_fee_basis_points(550)
    .token_standard(TokenStandard::NonFungible)
    .print_supply(PrintSupply::Zero)
    .instruction();

    // let message = Message::new(
    //     &[create_ix],
    //     Some(&sender.pubkey()),
    // );

    let rpc_client: RpcClient = RpcClient::new(String::from("https://api.devnet.solana.com"));

    let blockhash = rpc_client
        .get_latest_blockhash()
        .expect("Failed to get latest blockhash");

    // let transaction = Transaction::new_signed_with_payer(
    //     &[create_ix], //&[create_metadata_accounts_instruction],
    //     Some(&sender.pubkey()),
    //     &[&sender],
    //     blockhash,
    // );

    //let mut transaction = Transaction::new(&[&sender], message, blockhash);

    let transaction = Transaction::new_signed_with_payer(
        &[create_ix], 
        Some(&sender.pubkey()),
        &[&sender], //&[&sender, mint],//error
         blockhash,);

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => println!("Transaction confirmed, signature: {:?}", signature),
        Err(err) => eprintln!("Error sending transaction: {}", err),
    }
}

// let create_metadata_accounts = CreateMetadataAccountsV3::new {
//     metadata: Pubkey::find_program_address(
//         &[
//             "metadata".as_bytes(),
//             &token_metadata.to_bytes(),
//             &mint.to_bytes(),
//         ],
//         &token_metadata,
//     )
//     .0,
//     mint,
//     update_authority: sender.pubkey(),
//     metadata_data: Metadata {
//         name: "My Token".to_string(),
//         symbol: "MTK".to_string(),
//         uri: "https://example.com/metadata.json".to_string(),
//         seller_fee_basis_points: 500,
//         creators: None,
//         collection: None,
//         uses: None,
//     },
//     is_mutable: true,
//     collection_details: None,
// };
