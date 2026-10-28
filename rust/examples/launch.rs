use lastbuy_sdk::{instructions::*, pda::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;

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

    // Example: Launch a new token on the bonding curve
    // Assumes global config is initialized and payer is funded
    let mint = Keypair::new();

    let ix = LaunchInstruction::new(
        LaunchInstructionCtx {
            user: payer.pubkey(),
            mint: mint.pubkey(),
        },
        LaunchInstructionArgs {
            token_name: "My Token".to_string(),
            token_symbol: "MTK".to_string(),
            token_uri: "https://example.com/metadata.json".to_string(),
        },
    )?;

    let recent_blockhash = rpc.get_latest_blockhash().await?;
    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(1_400_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);
    let message = V1Message::try_compile_with_config(&payer.pubkey(), &[ix], recent_blockhash, tx_config)?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer, &mint])?;
    let sig = rpc.send_and_confirm_transaction(&tx).await?;
    println!("Launch tx: {}", sig);
    println!("Curve: {}", derive_curve(&mint.pubkey()).0);

    Ok(())
}
