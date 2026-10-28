use borsh::BorshDeserialize;
use lastbuy_sdk::{accounts::*, constants::*, instructions::*, pda::*};
use solana_instruction::Instruction;
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{v1::{Message as V1Message, TransactionConfig}, VersionedMessage};
use solana_pubkey::{pubkey, Pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::{address::get_associated_token_address_with_program_id, instruction::create_associated_token_account_idempotent};

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

fn create_ata_ix(payer: &Pubkey, owner: &Pubkey, mint: &Pubkey, token_program: &Pubkey) -> Instruction {
    create_associated_token_account_idempotent(payer, owner, mint, token_program)
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

    // Example: Sell exact token amount
    // Assumes global config is initialized and payer/fee recipients are funded
    let mint: Pubkey = pubkey!("11111111111111111111111111111111"); // your mint here
    let curve: Pubkey = derive_curve(&mint).0;
    let curve_state: CurveState = fetch_curve(&rpc, &curve).await?;
    let global_config: GlobalConfig = fetch_global_config(&rpc).await?;

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
            min_sol_out: 0,    // no slippage
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
