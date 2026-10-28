use borsh::BorshDeserialize;
use lastbuy_sdk::{accounts::*, constants::*, instructions::*, pda::*};
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;

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

    // Example: Claim creator fee (after migration)
    // Assumes global config is initialized, curve is migrated, and accounts are funded
    let mint: Pubkey = pubkey!("11111111111111111111111111111111"); // your mint here
    let global_config: GlobalConfig = fetch_global_config(&rpc).await?;

    let fee_recipient_1 = global_config.fees.fee_recipient_1;
    let fee_recipient_2 = global_config.fees.fee_recipient_2;
    let raydium_amm_config = global_config.raydium.raydium_amm_config;
    let raydium_cpmm_program = global_config.raydium.raydium_cpmm_program;
    let raydium_lock_program = global_config.raydium.raydium_lock_program;

    let ix = ClaimCreatorFeeInstruction::new(ClaimCreatorFeeInstructionCtx {
        user: payer.pubkey(),
        mint,
        raydium_amm_config,
        raydium_cpmm_program,
        raydium_lock_program,
        fee_recipient_1,
        fee_recipient_2,
    });

    let recent_blockhash = rpc.get_latest_blockhash().await?;
    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(1_400_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);
    let message = V1Message::try_compile_with_config(&payer.pubkey(), &[ix], recent_blockhash, tx_config)?;
    let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), &[&payer])?;
    let sig = rpc.send_and_confirm_transaction(&tx).await?;
    println!("ClaimCreatorFee tx: {}", sig);

    Ok(())
}
