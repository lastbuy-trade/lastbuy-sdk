mod common;

use common::*;
use lastbuy_sdk::{constants::*, instructions::*, pda::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
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

    // Example: Buy tokens with exact SOL amount
    // Assumes global config is initialized and payer/fee recipients are funded
    let mint: Pubkey = pubkey!("11111111111111111111111111111111"); // your mint here
    let curve: Pubkey = derive_curve(&mint).0;
    let curve_state = fetch_curve(&rpc, &curve).await?;
    let global_config = fetch_global_config(&rpc).await?;

    let fee_recipient_1 = global_config.fees.fee_recipient_1;
    let fee_recipient_2 = global_config.fees.fee_recipient_2;

    // Create user's ATA for the mint (idempotent) - needed for buy_exact
    let ata_ix = create_ata_ix(&payer.pubkey(), &payer.pubkey(), &mint, &SPL_TOKEN_2022_PROGRAM_ID);

    let buy_ix = BuyExactInstruction::new(
        BuyExactInstructionCtx {
            user: payer.pubkey(),
            creator: curve_state.creator,
            mint,
            fee_recipient_1,
            fee_recipient_2,
        },
        BuyExactInstructionArgs {
            sol_in: 1_000_000_000, // 1 SOL
            min_tokens_out: 0,
        },
    )?;

    let recent_blockhash = rpc.get_latest_blockhash().await?;
    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(1_400_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);
    let message = V1Message::try_compile_with_config(&payer.pubkey(), &[ata_ix, buy_ix], recent_blockhash, tx_config)?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer])?;
    let sig = rpc.send_and_confirm_transaction(&tx).await?;
    println!("BuyExact tx: {}", sig);

    Ok(())
}
