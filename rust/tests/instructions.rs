use lastbuy_sdk::{instructions::*, types::*};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_pubkey::{Pubkey, pubkey};
use solana_message::{VersionedMessage, v1::{Message as V1Message, TransactionConfig}};
use solana_transaction::versioned::VersionedTransaction;

const LAST_BUY_TIMER_SECS: i64 = 300;  // 5 min
const MIGRATION_THRESHOLD_SOL: u64 = 50_000_000_000;  // 50 sol
const RAYDIUM_CPMM_MIGRATION_COVERAGE_SOL: u64 = 200_000_000;  // 0.2 SOL

// https://github.com/raydium-io/raydium-sdk-V2/blob/master/src/common/programId.ts
// mainnet
// const RAYDIUM_CPMM_PROGRAM_ID: Pubkey = pubkey!(
//   "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C"
// );
// const RAYDIUM_LOCK_PROGRAM_ID: Pubkey = pubkey!(
//   "LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE"
// );
// // AmmConfig index 0: 25 bps trade fee, 0.15 SOL create_pool_fee
// const RAYDIUM_AMM_CONFIG: Pubkey = pubkey!(
//   "D4FPEruKEHrG5TenZ2mpDGEfu1iUvTiqBxvpU8HLBvC2"
// );
// const RAYDIUM_FEE_ACCOUNT: Pubkey = pubkey!(
//   "DNXgeM9EiiaAbaWvwjHj9fQQLAX5ZsfHyvmYUNRAdNC8"
// );
// #[allow(dead_code)]
// const RAYDIUM_CPMM_AUTHORITY: Pubkey = pubkey!(
//   "GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL"
// );
// #[allow(dead_code)]
// const RAYDIUM_LOCK_AUTHORITY: Pubkey = pubkey!(
//   "3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH"
// );

// devnet
const RAYDIUM_CPMM_PROGRAM_ID: Pubkey = pubkey!(
  "DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb"
);
const RAYDIUM_LOCK_PROGRAM_ID: Pubkey = pubkey!(
  "DRay25Usp3YJAi7beckgpGUC7mGJ2cR1AVPxhYfwVCUX"
);
// AmmConfig index 0: 25 bps trade fee, 0.15 SOL create_pool_fee
const RAYDIUM_AMM_CONFIG: Pubkey = pubkey!(
  "5MxLgy9oPdTC3YgkiePHqr3EoCRD9uLVYRQS2ANAs7wy"
);
const RAYDIUM_FEE_ACCOUNT: Pubkey = pubkey!(
  "3oE58BKVt8KuYkGxx8zBojugnymWmBiyafWgMrnb6eYy"
);
#[allow(dead_code)]
const RAYDIUM_CPMM_AUTHORITY: Pubkey = pubkey!(
  "CXniRufdq5xL8t8jZAPxsPZDpuudwuJSPWnbcD5Y5Nxq"
);
#[allow(dead_code)]
const RAYDIUM_LOCK_AUTHORITY: Pubkey = pubkey!(
  "7qWVV8UY2bRJfDLP4s37YzBPKUkVB46DStYJBpYbQzu3"
);

#[tokio::test]
#[ignore]
async fn test_initialize_global() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    
    let url = std::env::var("TEST_RPC_URL")?;
    let payer_keypair_path = std::env::var("TEST_PAYER_PATH")?; 

    let rpc_client = RpcClient::new(url);
    
    let signer = Keypair::read_from_file(payer_keypair_path)?;
    let signer_pubkey = signer.pubkey();

    let protocol_version = 1;

    let random_fee_recipient_1 = Keypair::new();
    let random_fee_recipient_2 = Keypair::new();

    let fees_config = FeeConfig { 
        protocol_curve_fee_bps: 50,           // 0.5%
        protocol_claim_creator_fee_bps: 100,  // 1%
        creator_fee_bps: 30,                  // 0.3%
        last_buyer_fee_bps: 70,               // 0.7%
        fee_recipient_1: random_fee_recipient_1.pubkey(), 
        fee_recipient_2: random_fee_recipient_2.pubkey() 
    };

    let protocol_config = ProtocolConfig { 
        last_buy_timer_secs: LAST_BUY_TIMER_SECS, 
        migration_threshold_sol: MIGRATION_THRESHOLD_SOL, 
        protocol_authority: signer_pubkey  // signer is the protocol authority, should be changed later 
    };

    let raydium_config = RaydiumConfig { 
        raydium_cpmm_program: RAYDIUM_CPMM_PROGRAM_ID, 
        raydium_lock_program: RAYDIUM_LOCK_PROGRAM_ID, 
        raydium_amm_config: RAYDIUM_AMM_CONFIG, 
        raydium_fee_account: RAYDIUM_FEE_ACCOUNT, 
        raydium_cpmm_migration_coverage_sol: RAYDIUM_CPMM_MIGRATION_COVERAGE_SOL 
    };

    let admin_config = AdminConfig::default();

    let initialize_global_ctx = InitializeGlobalInstructionCtx {
        user: signer_pubkey
    };

    let initialize_global_args = InitializeGlobalInstructionArgs {
        protocol_version,
        fees: fees_config,
        protocol: protocol_config,
        raydium: raydium_config,
        admin: admin_config
    };

    let initialize_global_ix: Instruction = InitializeGlobalInstruction::new(
        initialize_global_ctx, 
        initialize_global_args
    )?;

    let recent_blockhash = rpc_client.get_latest_blockhash().await?;

    let tx_config = TransactionConfig::empty()
        .with_compute_unit_limit(200_000)
        .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);

    let v1_message = V1Message::try_compile_with_config(
        &signer_pubkey, 
        &[initialize_global_ix], 
        recent_blockhash, 
        tx_config
    )?;

    let v1_tx = VersionedTransaction::try_new(
        VersionedMessage::V1(v1_message), 
        &[&signer]
    )?;

    let tx_sig = rpc_client.send_transaction(&v1_tx).await?;

    println!("successfully executed `initialize_global` instruction: {}", tx_sig);

    Ok(())
}

#[test]
#[ignore]
fn test_raydium_cpmm_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let lock_cp_program_id = pubkey!("CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C");
    let seed = "vault_and_lp_mint_auth_seed"; 
    
    let expected = pubkey!("GpMZbSM2GgvTKHJirzeGfMFoaZ8UR2X7F4v8vHTvxFbL");
    
    let derived = Pubkey::find_program_address(
        &[seed.as_ref()], 
        &lock_cp_program_id
    ).0;

    assert_eq!(expected, derived);

    Ok(())
}

#[test]
#[ignore]
fn test_raydium_lock_authority_derivation() -> Result<(), Box<dyn std::error::Error>> {
    let lock_cp_program_id = pubkey!("LockrWmn6K5twhz3y9w1dQERbmgSaRkfnTeTKbpofwE");
    let seed = "lock_cp_authority_seed"; 
    
    let expected = pubkey!("3f7GcQFG397GAaEnv51zR6tsTVihYRydnydDD1cXekxH");
    
    let derived = Pubkey::find_program_address(
        &[seed.as_ref()], 
        &lock_cp_program_id
    ).0;

    assert_eq!(expected, derived);

    Ok(())
}