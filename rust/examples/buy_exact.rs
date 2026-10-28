use borsh::BorshDeserialize;
use lastbuy_sdk::{accounts::*, constants::*, instructions::*, pda::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::instruction::create_associated_token_account_idempotent;

fn decode_account<T: BorshDeserialize>(data: &[u8]) -> Result<T, Box<dyn std::error::Error>> {
    Ok(T::try_from_slice(&data[ANCHOR_DISCRIMINATOR_SIZE..])?)
}

async fn fetch_account_data(rpc: &RpcClient, pubkey: &Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = rpc
        .get_account_with_commitment(
            pubkey,
            solana_rpc_client::api::config::CommitmentConfig {
                commitment: solana_rpc_client::api::config::CommitmentLevel::Processed,
            },
        )
        .await?;
    let account = response.value.ok_or_else(|| format!("Account not found: {pubkey}"))?;
    Ok(account.data)
}

async fn fetch_global_config(rpc: &RpcClient) -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let data = fetch_account_data(rpc, &derive_global_config().0).await?;
    decode_account(&data)
}

async fn fetch_curve(rpc: &RpcClient, curve: &Pubkey) -> Result<CurveState, Box<dyn std::error::Error>> {
    let data = fetch_account_data(rpc, curve).await?;
    decode_account(&data)
}

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
    let curve_state: CurveState = fetch_curve(&rpc, &curve).await?;
    let global_config: GlobalConfig = fetch_global_config(&rpc).await?;

    let fee_recipient_1 = global_config.fees.fee_recipient_1;
    let fee_recipient_2 = global_config.fees.fee_recipient_2;

    // Create user's ATA for the mint (idempotent) - needed for buy_exact
    let ata_ix = create_associated_token_account_idempotent(
        &payer.pubkey(),
        &payer.pubkey(),
        &mint,
        &SPL_TOKEN_2022_PROGRAM_ID
    );

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
            min_tokens_out: 0,     // no slippage
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
