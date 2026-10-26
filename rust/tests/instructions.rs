#![cfg(feature = "dev")]

//! End-to-end tests for every bonding curve instruction.
//!
//! The tests send real transactions to the RPC endpoint configured in the
//! workspace `.env`:
//!
//! * `TEST_RPC_URL`
//! * `TEST_PAYER_PATH`
//!
//! They are ignored by default, so a plain `cargo test` never touches the
//! network:
//!
//! ```text
//! cargo test --features dev -- --ignored
//! ```
//!
//! `global_config` is a singleton PDA, therefore every test takes a
//! process-wide async mutex and rewrites the config before launching its own
//! curve. Each test is self-contained and leaves its own global state behind.

use std::sync::OnceLock;

use borsh::BorshDeserialize;
use lastbuy_sdk::{accounts::*, constants::*, instructions::*, pda::*, types::*};
use solana_instruction::Instruction;
use solana_keypair::{EncodableKey, Keypair, Signer};
use solana_message::{
    VersionedMessage,
    v1::{Message as V1Message, TransactionConfig},
};
use solana_pubkey::{Pubkey, pubkey};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::instruction::create_associated_token_account_idempotent;
use tokio::sync::Mutex;

const TEST_PROTOCOL_VERSION: u8 = 1;
const TEST_LAST_BUY_TIMER_SECS: i64 = 0;
const TEST_MIGRATION_THRESHOLD_SOL: u64 = 1_000_000_000; // 1 SOL
const TEST_RAYDIUM_CPMM_MIGRATION_COVERAGE_SOL: u64 = 200_000_000; // 0.2 SOL
const TEST_BUY_SOL: u64 = 10_000_000; // 0.01 SOL
const TEST_SELL_TOKENS: u64 = 1; // 1 base unit

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
const RAYDIUM_CPMM_PROGRAM_ID: Pubkey = pubkey!("DRaycpLY18LhpbydsBWbVJtxpNv9oXPgjRSfpF2bWpYb");
const RAYDIUM_LOCK_PROGRAM_ID: Pubkey = pubkey!("DRay25Usp3YJAi7beckgpGUC7mGJ2cR1AVPxhYfwVCUX");
// AmmConfig index 0: 25 bps trade fee, 0.15 SOL create_pool_fee
const RAYDIUM_AMM_CONFIG: Pubkey = pubkey!("5MxLgy9oPdTC3YgkiePHqr3EoCRD9uLVYRQS2ANAs7wy");
const RAYDIUM_FEE_ACCOUNT: Pubkey = pubkey!("3oE58BKVt8KuYkGxx8zBojugnymWmBiyafWgMrnb6eYy");
#[allow(dead_code)]
const RAYDIUM_CPMM_AUTHORITY: Pubkey = pubkey!("CXniRufdq5xL8t8jZAPxsPZDpuudwuJSPWnbcD5Y5Nxq");
#[allow(dead_code)]
const RAYDIUM_LOCK_AUTHORITY: Pubkey = pubkey!("7qWVV8UY2bRJfDLP4s37YzBPKUkVB46DStYJBpYbQzu3");

/// `global_config` is a singleton, so tests that mutate it must not overlap.
fn test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

struct TestEnv {
    rpc: RpcClient,
    payer: Keypair,
    fee_recipient_1: Pubkey,
    fee_recipient_2: Pubkey,
}

impl TestEnv {
    async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();

        let rpc = RpcClient::new(std::env::var("TEST_RPC_URL")?);
        let payer = Keypair::read_from_file(std::env::var("TEST_PAYER_PATH")?)?;

        Ok(Self {
            rpc,
            payer,
            fee_recipient_1: Pubkey::new_from_array([1; 32]),
            fee_recipient_2: Pubkey::new_from_array([2; 32]),
        })
    }

    fn payer_pubkey(&self) -> Pubkey {
        self.payer.pubkey()
    }

    async fn ensure_funded(&self, min_lamports: u64) -> Result<(), Box<dyn std::error::Error>> {
        let mut balance = self.rpc.get_balance(&self.payer_pubkey()).await?;

        while balance < min_lamports {
            let chunk = (min_lamports - balance).min(1_000_000_000);
            self.rpc
                .request_airdrop(&self.payer_pubkey(), chunk)
                .await?;

            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            balance = self.rpc.get_balance(&self.payer_pubkey()).await?;
        }

        Ok(())
    }

    async fn send(
        &self,
        instructions: &[Instruction],
        signers: &[&Keypair],
    ) -> Result<String, Box<dyn std::error::Error>> {
        let recent_blockhash = self.rpc.get_latest_blockhash().await?;

        let tx_config = TransactionConfig::empty()
            .with_compute_unit_limit(1_400_000)
            .with_loaded_accounts_data_size_limit(64 * 1024 * 1024);

        let message = V1Message::try_compile_with_config(
            &self.payer_pubkey(),
            instructions,
            recent_blockhash,
            tx_config,
        )?;

        let tx = VersionedTransaction::try_new(VersionedMessage::V1(message), signers)?;
        let signature = self.rpc.send_transaction(&tx).await?;

        self.rpc.confirm_transaction(&signature).await?;

        Ok(signature.to_string())
    }
}

#[derive(Clone, Copy)]
struct TestParams {
    last_buy_timer_secs: i64,
    migration_threshold_sol: u64,
    protocol_curve_fee_bps: u16,
    protocol_claim_creator_fee_bps: u16,
    creator_fee_bps: u16,
    last_buyer_fee_bps: u16,
}

impl TestParams {
    /// Realistic fee configuration, used by tests that do not trade the curve
    /// all the way to graduation.
    fn standard() -> Self {
        Self {
            last_buy_timer_secs: TEST_LAST_BUY_TIMER_SECS,
            migration_threshold_sol: TEST_MIGRATION_THRESHOLD_SOL,
            protocol_curve_fee_bps: 50,
            protocol_claim_creator_fee_bps: 100,
            creator_fee_bps: 30,
            last_buyer_fee_bps: 70,
        }
    }

    /// Zero fees make `net_sol == sol_in`, so a single buy for exactly
    /// `migration_threshold_sol` graduates the curve.
    fn zero_fees() -> Self {
        Self {
            last_buy_timer_secs: TEST_LAST_BUY_TIMER_SECS,
            migration_threshold_sol: TEST_MIGRATION_THRESHOLD_SOL,
            protocol_curve_fee_bps: 0,
            protocol_claim_creator_fee_bps: 0,
            creator_fee_bps: 0,
            last_buyer_fee_bps: 0,
        }
    }
}

fn fee_config(env: &TestEnv, params: TestParams) -> FeeConfig {
    FeeConfig {
        protocol_curve_fee_bps: params.protocol_curve_fee_bps,
        protocol_claim_creator_fee_bps: params.protocol_claim_creator_fee_bps,
        creator_fee_bps: params.creator_fee_bps,
        last_buyer_fee_bps: params.last_buyer_fee_bps,
        fee_recipient_1: env.fee_recipient_1,
        fee_recipient_2: env.fee_recipient_2,
    }
}

fn protocol_config(env: &TestEnv, params: TestParams) -> ProtocolConfig {
    ProtocolConfig {
        last_buy_timer_secs: params.last_buy_timer_secs,
        migration_threshold_sol: params.migration_threshold_sol,
        protocol_authority: env.payer_pubkey(),
    }
}

fn raydium_config() -> RaydiumConfig {
    RaydiumConfig {
        raydium_cpmm_program: RAYDIUM_CPMM_PROGRAM_ID,
        raydium_lock_program: RAYDIUM_LOCK_PROGRAM_ID,
        raydium_amm_config: RAYDIUM_AMM_CONFIG,
        raydium_fee_account: RAYDIUM_FEE_ACCOUNT,
        raydium_cpmm_migration_coverage_sol: TEST_RAYDIUM_CPMM_MIGRATION_COVERAGE_SOL,
    }
}

fn initialize_args(env: &TestEnv, params: TestParams) -> InitializeGlobalInstructionArgs {
    InitializeGlobalInstructionArgs {
        protocol_version: TEST_PROTOCOL_VERSION,
        fees: fee_config(env, params),
        protocol: protocol_config(env, params),
        raydium: raydium_config(),
        admin: AdminConfig::default(),
    }
}

fn update_args(env: &TestEnv, params: TestParams) -> UpdateGlobalArgs {
    UpdateGlobalArgs {
        protocol_version: TEST_PROTOCOL_VERSION,
        fees: fee_config(env, params),
        protocol: protocol_config(env, params),
        raydium: raydium_config(),
        admin: AdminConfig::default(),
    }
}

/// Creates `global_config` the first time, updates it on every subsequent call.
async fn ensure_global_config(
    env: &TestEnv,
    params: TestParams,
) -> Result<(), Box<dyn std::error::Error>> {
    if env.rpc.get_account(&derive_global_config().0).await.is_ok() {
        let ix = UpdateGlobalInstruction::new(
            UpdateGlobalCtx {
                user: env.payer_pubkey(),
            },
            update_args(env, params),
        )?;
        env.send(&[ix], &[&env.payer]).await?;
    } else {
        let ix = InitializeGlobalInstruction::new(
            InitializeGlobalInstructionCtx {
                user: env.payer_pubkey(),
            },
            initialize_args(env, params),
        )?;
        env.send(&[ix], &[&env.payer]).await?;
    }

    Ok(())
}

fn decode_account<T: BorshDeserialize>(data: &[u8]) -> Result<T, Box<dyn std::error::Error>> {
    Ok(T::try_from_slice(&data[ANCHOR_DISCRIMINATOR_SIZE..])?)
}

async fn fetch_global_config(env: &TestEnv) -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let data = env.rpc.get_account_data(&derive_global_config().0).await?;
    decode_account(&data)
}

async fn fetch_curve(
    env: &TestEnv,
    curve: &Pubkey,
) -> Result<CurveState, Box<dyn std::error::Error>> {
    let data = env.rpc.get_account_data(curve).await?;
    decode_account(&data)
}

async fn fetch_prize_pool(
    env: &TestEnv,
    prize_pool: &Pubkey,
) -> Result<PrizePool, Box<dyn std::error::Error>> {
    let data = env.rpc.get_account_data(prize_pool).await?;
    decode_account(&data)
}

async fn fetch_pool_creator_state(
    env: &TestEnv,
    pool_creator_state: &Pubkey,
) -> Result<PoolCreatorState, Box<dyn std::error::Error>> {
    let data = env.rpc.get_account_data(pool_creator_state).await?;
    decode_account(&data)
}

struct CurveFixture {
    mint: Keypair,
    curve: Pubkey,
    prize_pool: Pubkey,
}

async fn launch_curve(env: &TestEnv) -> Result<CurveFixture, Box<dyn std::error::Error>> {
    // The on-chain program requires `WSOL_ID < mint.key()`.
    let mint = loop {
        let candidate = Keypair::new();
        if WSOL_ID < candidate.pubkey() {
            break candidate;
        }
    };

    let ix = LaunchInstruction::new(
        LaunchInstructionCtx {
            user: env.payer_pubkey(),
            mint: mint.pubkey(),
        },
        LaunchInstructionArgs {
            token_name: "Test Token".to_string(),
            token_symbol: "TEST".to_string(),
            token_uri: "https://example.com/test.json".to_string(),
        },
    )?;

    env.send(&[ix], &[&env.payer, &mint]).await?;

    let curve = derive_curve(&mint.pubkey()).0;
    let prize_pool = derive_prize_pool(&curve).0;

    Ok(CurveFixture {
        mint,
        curve,
        prize_pool,
    })
}

fn create_ata_ix(
    payer: &Pubkey,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Instruction {
    create_associated_token_account_idempotent(payer, owner, mint, token_program)
}

async fn create_ata(
    env: &TestEnv,
    owner: &Pubkey,
    mint: &Pubkey,
    token_program: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = create_ata_ix(&env.payer_pubkey(), owner, mint, token_program);
    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

async fn buy(
    env: &TestEnv,
    fixture: &CurveFixture,
    sol_in: u64,
    min_tokens_out: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = BuyExactInstruction::new(
        BuyExactInstructionCtx {
            user: env.payer_pubkey(),
            creator: env.payer_pubkey(),
            mint: fixture.mint.pubkey(),
            fee_recipient_1: env.fee_recipient_1,
            fee_recipient_2: env.fee_recipient_2,
        },
        BuyExactInstructionArgs {
            sol_in,
            min_tokens_out,
        },
    )?;

    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

async fn sell(
    env: &TestEnv,
    fixture: &CurveFixture,
    tokens_in: u64,
    min_sol_out: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = SellExactInstruction::new(
        SellExactInstructionCtx {
            user: env.payer_pubkey(),
            creator: env.payer_pubkey(),
            mint: fixture.mint.pubkey(),
            fee_recipient_1: env.fee_recipient_1,
            fee_recipient_2: env.fee_recipient_2,
        },
        SellExactInstructionArgs {
            tokens_in,
            min_sol_out,
        },
    )?;

    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

async fn fund_migration(
    env: &TestEnv,
    fixture: &CurveFixture,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = FundMigrationInstruction::new(FundMigrationInstructionCtx {
        user: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
    });

    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

async fn define_winner(
    env: &TestEnv,
    fixture: &CurveFixture,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = DefineWinnerIdempotentInstruction::new(DefineWinnerIdempotentInstructionCtx {
        user: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
    });

    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

async fn claim_prize(
    env: &TestEnv,
    fixture: &CurveFixture,
) -> Result<(), Box<dyn std::error::Error>> {
    let ix = ClaimPrizeIdempotentInstruction::new(ClaimPrizeIdempotentInstructionCtx {
        user: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
    });

    env.send(&[ix], &[&env.payer]).await?;
    Ok(())
}

/// Creates the associated token accounts that `prepare_migration` and
/// `migrate` expect on the `pool_creator` PDA.
async fn create_pool_creator_atas(
    env: &TestEnv,
    fixture: &CurveFixture,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let pool_creator = derive_pool_creator(&env.payer_pubkey(), &fixture.curve).0;

    let instructions = [
        // `prepare_migration` receives the WSOL ATA and adds the migration SOL
        // to it directly, so it has to exist beforehand.
        create_ata_ix(
            &env.payer_pubkey(),
            &pool_creator,
            &WSOL_ID,
            &SPL_TOKEN_PROGRAM_ID,
        ),
        // `migrate` moves the curve token balance into this ATA.
        create_ata_ix(
            &env.payer_pubkey(),
            &pool_creator,
            &fixture.mint.pubkey(),
            &SPL_TOKEN_2022_PROGRAM_ID,
        ),
    ];

    env.send(&instructions, &[&env.payer]).await?;

    Ok(pool_creator)
}

/// Full migration flow: update config, launch, graduate with one buy, fund the
/// migration, then execute `prepare_migration` and `migrate` in one
/// transaction.
async fn migrate_curve(
    env: &TestEnv,
    params: TestParams,
) -> Result<CurveFixture, Box<dyn std::error::Error>> {
    ensure_global_config(env, params).await?;

    let fixture = launch_curve(env).await?;

    create_ata(
        env,
        &env.payer_pubkey(),
        &fixture.mint.pubkey(),
        &SPL_TOKEN_2022_PROGRAM_ID,
    )
    .await?;

    buy(env, &fixture, params.migration_threshold_sol, 0).await?;

    let curve = fetch_curve(env, &fixture.curve).await?;
    assert_eq!(curve.status, CurveStatus::Graduated);

    fund_migration(env, &fixture).await?;
    create_pool_creator_atas(env, &fixture).await?;

    let prepare = PrepareMigrationInstruction::new(PrepareMigrationInstructionCtx {
        user: env.payer_pubkey(),
        migration_funder: env.payer_pubkey(),
        creator: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
        fee_recipient_1: env.fee_recipient_1,
        fee_recipient_2: env.fee_recipient_2,
    });

    let migrate = MigrateInstruction::new(MigrateInstructionCtx {
        user: env.payer_pubkey(),
        migration_funder: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
        raydium_amm_config: RAYDIUM_AMM_CONFIG,
        raydium_fee_account: RAYDIUM_FEE_ACCOUNT,
        raydium_cpmm_program: RAYDIUM_CPMM_PROGRAM_ID,
        raydium_lock_program: RAYDIUM_LOCK_PROGRAM_ID,
    });

    // Order matters: `prepare_migration` first, `migrate` second.
    env.send(&[prepare, migrate], &[&env.payer]).await?;

    let curve = fetch_curve(env, &fixture.curve).await?;
    assert_eq!(curve.status, CurveStatus::Migrated);

    Ok(fixture)
}

#[tokio::test]
#[ignore]
async fn test_initialize_global() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    if env.rpc.get_account(&derive_global_config().0).await.is_ok() {
        eprintln!("global_config is already initialized; skipping test_initialize_global");
        return Ok(());
    }

    let ix = InitializeGlobalInstruction::new(
        InitializeGlobalInstructionCtx {
            user: env.payer_pubkey(),
        },
        initialize_args(&env, TestParams::standard()),
    )?;

    env.send(&[ix], &[&env.payer]).await?;

    let config = fetch_global_config(&env).await?;
    assert_eq!(config.protocol.protocol_authority, env.payer_pubkey());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_update_global() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let mut params = TestParams::standard();
    params.migration_threshold_sol = 2_000_000_000;

    let ix = UpdateGlobalInstruction::new(
        UpdateGlobalCtx {
            user: env.payer_pubkey(),
        },
        update_args(&env, params),
    )?;

    env.send(&[ix], &[&env.payer]).await?;

    let config = fetch_global_config(&env).await?;
    assert_eq!(
        config.protocol.migration_threshold_sol,
        params.migration_threshold_sol
    );

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_launch() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;
    let curve = fetch_curve(&env, &fixture.curve).await?;

    assert_eq!(curve.status, CurveStatus::Active);
    assert_eq!(curve.creator, env.payer_pubkey());
    assert_eq!(curve.mint, fixture.mint.pubkey());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_buy_exact() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;

    create_ata(
        &env,
        &env.payer_pubkey(),
        &fixture.mint.pubkey(),
        &SPL_TOKEN_2022_PROGRAM_ID,
    )
    .await?;

    buy(&env, &fixture, TEST_BUY_SOL, 0).await?;

    let curve = fetch_curve(&env, &fixture.curve).await?;
    assert!(curve.real_reserves_sol > 0);
    assert!(curve.real_reserves_tokens > 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_sell_exact() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;

    create_ata(
        &env,
        &env.payer_pubkey(),
        &fixture.mint.pubkey(),
        &SPL_TOKEN_2022_PROGRAM_ID,
    )
    .await?;

    buy(&env, &fixture, TEST_BUY_SOL, 0).await?;
    sell(&env, &fixture, TEST_SELL_TOKENS, 0).await?;

    let curve = fetch_curve(&env, &fixture.curve).await?;
    assert!(curve.real_reserves_tokens > 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_fund_migration() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(3_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;
    fund_migration(&env, &fixture).await?;

    let curve = fetch_curve(&env, &fixture.curve).await?;
    assert_eq!(curve.migration_funder, env.payer_pubkey());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_define_winner_idempotent() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;

    create_ata(
        &env,
        &env.payer_pubkey(),
        &fixture.mint.pubkey(),
        &SPL_TOKEN_2022_PROGRAM_ID,
    )
    .await?;

    buy(&env, &fixture, TEST_BUY_SOL, 0).await?;
    define_winner(&env, &fixture).await?;

    let curve = fetch_curve(&env, &fixture.curve).await?;
    let prize_pool = fetch_prize_pool(&env, &fixture.prize_pool).await?;

    assert_eq!(curve.status, CurveStatus::WinnerDefined);
    assert_eq!(prize_pool.winner, env.payer_pubkey());

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_claim_prize_idempotent() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(2_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;

    create_ata(
        &env,
        &env.payer_pubkey(),
        &fixture.mint.pubkey(),
        &SPL_TOKEN_2022_PROGRAM_ID,
    )
    .await?;

    buy(&env, &fixture, TEST_BUY_SOL, 0).await?;
    define_winner(&env, &fixture).await?;
    claim_prize(&env, &fixture).await?;

    let prize_pool = fetch_prize_pool(&env, &fixture.prize_pool).await?;
    assert!(prize_pool.is_claimed);
    assert_eq!(prize_pool.amount_sol, 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_emergency_drain_curve() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(3_000_000_000).await?;

    ensure_global_config(&env, TestParams::standard()).await?;

    let fixture = launch_curve(&env).await?;

    // Funding the migration exercises the optional `migration_funder` account
    // and the coverage refund branch.
    fund_migration(&env, &fixture).await?;

    let ix = EmergencyDrainCurveInstruction::new(EmergencyDrainCurveInstructionCtx {
        user: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
        recipient: env.payer_pubkey(),
        migration_funder: Some(env.payer_pubkey()),
    });

    env.send(&[ix], &[&env.payer]).await?;

    let curve = fetch_curve(&env, &fixture.curve).await?;
    assert_eq!(curve.status, CurveStatus::Inactive);
    assert_eq!(curve.real_reserves_sol, 0);
    assert_eq!(curve.real_reserves_tokens, 0);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_prepare_migration_and_migrate() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(5_000_000_000).await?;

    let fixture = migrate_curve(&env, TestParams::zero_fees()).await?;

    let pool_creator = derive_pool_creator(&env.payer_pubkey(), &fixture.curve).0;
    let pool_creator_state = derive_pool_creator_state(&pool_creator).0;
    let state = fetch_pool_creator_state(&env, &pool_creator_state).await?;

    assert!(!state.is_fee_claim_locked);

    let curve = fetch_curve(&env, &fixture.curve).await?;
    assert_eq!(curve.status, CurveStatus::Migrated);

    Ok(())
}

#[tokio::test]
#[ignore]
async fn test_claim_creator_fee() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = test_lock().lock().await;
    let env = TestEnv::new().await?;
    env.ensure_funded(5_000_000_000).await?;

    let fixture = migrate_curve(&env, TestParams::zero_fees()).await?;

    let ix = ClaimCreatorFeeInstruction::new(ClaimCreatorFeeInstructionCtx {
        user: env.payer_pubkey(),
        mint: fixture.mint.pubkey(),
        raydium_amm_config: RAYDIUM_AMM_CONFIG,
        raydium_cpmm_program: RAYDIUM_CPMM_PROGRAM_ID,
        raydium_lock_program: RAYDIUM_LOCK_PROGRAM_ID,
        fee_recipient_1: env.fee_recipient_1,
        fee_recipient_2: env.fee_recipient_2,
    });

    env.send(&[ix], &[&env.payer]).await?;

    Ok(())
}
