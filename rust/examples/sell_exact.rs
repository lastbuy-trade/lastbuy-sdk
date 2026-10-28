mod common;

use common::*;
use lastbuy_sdk::{constants::*, instructions::*, pda::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::address::get_associated_token_address_with_program_id;

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

    // Example: Sell exact token amount
    // Assumes global config is initialized and payer/fee recipients are funded
    let mint: Pubkey = pubkey!("11111111111111111111111111111111"); // your mint here
    let curve: Pubkey = derive_curve(&mint).0;
    let curve_state = fetch_curve(&rpc, &curve).await?;
    let global_config = fetch_global_config(&rpc).await?;

    let fee_recipient_1 = global_config.fees.fee_recipient_1;
    let fee_recipient_2 = global_config.fees.fee_recipient_2;

    // Ensure user's ATA exists (idempotent)
    let ata_ix = create_ata_ix(&payer.pubkey(), &payer.pubkey(), &mint, &SPL_TOKEN_2022_PROGRAM_ID);

    let ata = get_associated_token_address_with_program_id(&payer.pubkey(), &mint, &SPL_TOKEN_2022_PROGRAM_ID);
    let tokens_balance = rpc.get_token_account_balance(&ata).await?.amount.parse::<u64>()?;

    let sell_ix = SellExactInstruction::new(
        SellExactInstructionCtx {
            user: payer.pubkey(),
            creator: curve_state.creator,
            mint,
            fee_recipient_1,
            fee_recipient_2,
        },
        SellExactInstructionArgs {
            tokens_in: tokens_balance,
            min_sol_out: 0,
        },
    )?;

    let recent_blockhash = rpc.get_latest_blockhash().await?;
    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(1_400_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);
    let message = V1Message::try_compile_with_config(&payer.pubkey(), &[ata_ix, sell_ix], recent_blockhash, tx_config)?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer])?;
    let sig = rpc.send_and_confirm_transaction(&tx).await?;
    println!("SellExact tx: {}", sig);

    Ok(())
}
