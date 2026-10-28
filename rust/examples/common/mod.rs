#![allow(dead_code)]

use borsh::BorshDeserialize;
use lastbuy_sdk::{accounts::*, constants::*, pda::*};
use solana_instruction::Instruction;
use solana_pubkey::Pubkey;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use spl_associated_token_account_interface::instruction::create_associated_token_account_idempotent;

pub fn decode_account<T: BorshDeserialize>(data: &[u8]) -> Result<T, Box<dyn std::error::Error>> {
    Ok(T::try_from_slice(&data[ANCHOR_DISCRIMINATOR_SIZE..])?)
}

pub async fn fetch_account_data(rpc: &RpcClient, pubkey: &Pubkey) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let response = rpc
        .get_account_with_commitment(
            pubkey,
            solana_rpc_client::api::config::CommitmentConfig {
                commitment: solana_rpc_client::api::config::CommitmentLevel::Processed,
            },
        )
        .await?;
    let account = response
        .value
        .ok_or_else(|| format!("Account not found: {pubkey}"))?;
    Ok(account.data)
}

pub async fn fetch_global_config(rpc: &RpcClient) -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let data = fetch_account_data(rpc, &derive_global_config().0).await?;
    decode_account(&data)
}

pub async fn fetch_curve(rpc: &RpcClient, curve: &Pubkey) -> Result<CurveState, Box<dyn std::error::Error>> {
    let data = fetch_account_data(rpc, curve).await?;
    decode_account(&data)
}

pub fn create_ata_ix(
    payer: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Instruction {
    create_associated_token_account_idempotent(payer, owner, mint, token_program)
}
