use lastbuy_sdk::{instructions::*, constants::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::instruction::create_associated_token_account_idempotent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let rpc = RpcClient::new_with_commitment(
        std::env::var("TEST_RPC_URL")?,
        solana_rpc_client::api::config::CommitmentConfig {
            commitment: solana_rpc_client::api::config::CommitmentLevel::Confirmed,
        },
    );
    let payer = Keypair::read_from_file(std::env::var("TEST_PAYER_PATH")?)?;

    // Example: Define winner idempotent
    // Assumes global config is initialized and payer is funded
    let mint: Pubkey = pubkey!("11111111111111111111111111111111"); // your mint here

    // Create user's ATA for the mint (idempotent) - needed for define_winner_idempotent
    let ata_ix = create_associated_token_account_idempotent(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint,
        &SPL_TOKEN_2022_PROGRAM_ID
    );

    let define_winner_ix = DefineWinnerIdempotentInstruction::new(DefineWinnerIdempotentInstructionCtx {
        user: payer.pubkey(),
        mint,
    });

    let recent_blockhash = rpc.get_latest_blockhash().await?;
    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(1_400_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);
    let message = V1Message::try_compile_with_config(&payer.pubkey(), &[ata_ix, define_winner_ix], recent_blockhash, tx_config)?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer])?;
    let sig = rpc.send_and_confirm_transaction(&tx).await?;
    println!("DefineWinnerIdempotent tx: {}", sig);

    Ok(())
}
