# lastbuy-sdk

Low-level Rust SDK for the LastBuy bonding curve program on Solana.

This crate provides instruction builders, PDA derivation helpers, account/event types, discriminators, constants, error types, and integer math for trading on the LastBuy bonding curve.

Your client handles RPC calls, wallets and signers, and transaction submission.

An optional high-level client is planned and will live in this crate behind the `client` feature flag.

## Program
- Program ID: `Lbuyii1PVUvt9xToGuyUf6CQCDbdRRzzo8SsHuNYMkf` (same on mainnet-beta, devnet, and testnet)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lastbuy-sdk = "0.3.0"
```

By default the crate enables the `full` feature set. You can trim dependencies via feature flags:

```toml
[dependencies]
lastbuy-sdk = { version = "0.3.0", default-features = false, features = ["instructions", "pda", "types"] }
```

## Feature flags

| Feature | What it enables |
| --- | --- |
| `full` (default) | `events`, `parse-events`, `accounts`, `instructions`, `pda`, `error`, `math` |
| `instructions` | Instruction builders (`buy_exact`, `sell_exact`, `launch`, etc.), requires `pda`, `types`, `error` |
| `accounts` | Borsh types for decoding account data (`CurveState`, `GlobalConfig`, `PrizePool`, `PoolCreatorState`) |
| `events` | Anchor event types and discriminators |
| `parse-events` | Event instruction parsing, requires `events`, `error` |
| `types` | Shared types (`FeeConfig`, `ProtocolConfig`, `AdminConfig`, `RaydiumConfig`, `CurveStatus`) |
| `pda` | PDA derivation helpers for all program PDAs |
| `error` | `SdkError` / `SdkResult` |
| `math` | Integer trade quotes, swap fees, curve metrics, and `MathError`. Enables `error`. |
| `dev` | Admin-only instructions (enabled additionally when needed) |

## Usage

### Derive PDAs
```rust
use lastbuy_sdk::{derive_curve, derive_global_config};
use solana_pubkey::Pubkey;

let mint = Pubkey::new_unique();
let (curve, _bump) = derive_curve(&mint);
let (global_config, _bump) = derive_global_config();
```

### Build a `buy_exact` instruction
```rust
use lastbuy_sdk::{
    BuyExactInstruction, BuyExactInstructionCtx, BuyExactInstructionArgs,
};
use solana_pubkey::Pubkey;

let ctx = BuyExactInstructionCtx {
    user: user_pubkey,
    // curve creator can be fetched from `curve` PDA
    creator: curve_creator_pubkey,
    mint: mint_pubkey,
    // fee recipients can be fetched from `global_config` PDA
    fee_recipient_1: fee1_pubkey,
    fee_recipient_2: fee2_pubkey,
};
let args = BuyExactInstructionArgs {
    sol_in: 1_000_000_000,      // 1 SOL
    min_tokens_out: 1,          // slippage protection
};
let ix = BuyExactInstruction::new(ctx, args)?;
```

### Access account discriminators/layout constants
```rust
use lastbuy_sdk::constants::{CURVE_STATE_DISCRIMINATOR, BUY_EXACT_SIZE};
```

### Quote trades with integer math

```toml
[dependencies]
lastbuy-sdk = { version = "0.3.0", default-features = false, features = ["math"] }
```

```rust
use lastbuy_sdk::{
    constants::program::{INITIAL_VIRTUAL_SOL_RESERVES, INITIAL_VIRTUAL_TOKEN_RESERVES},
    error::{MathError, SdkResult},
    math::{calculate_buy_amount_out, calculate_swap_fee_split},
};

fn main() -> SdkResult<()> {
    let gross_lamports = 1_000_000_000;
    // Protocol, creator, and last-buyer fees in basis points. Winner undefined.
    let fees = calculate_swap_fee_split(gross_lamports, 100, 30, 70, false)?;
    let net_lamports = gross_lamports
        .checked_sub(fees.total_fee)
        .ok_or(MathError::Overflow)?;
    let quote = calculate_buy_amount_out(
        INITIAL_VIRTUAL_SOL_RESERVES,
        INITIAL_VIRTUAL_TOKEN_RESERVES,
        net_lamports,
    )?;
    assert_eq!(quote.tokens_out, 50_863_775_023_833);
    Ok(())
}
```

- SOL amounts are in lamports, token amounts are in atoms, and fee rates are in
  basis points. Pass net SOL after fees to `calculate_buy_amount_out`. For a
  sell, subtract swap fees from the `gross_sol_out` returned by
  `calculate_sell_amount_out`. For an existing curve, use its current virtual
  reserves and the swap rates in the `fees` field of `GlobalConfig`.
- `calculate_swap_fee_split` rounds each fee down separately. Set
  `is_winner_defined` once the canonical winner is defined to split the last-buyer
  fee between the protocol and creator. The creator receives any odd lamport.
- `market_cap_lamports` scales by total supply in atoms. `price_lamports` gives
  the price of one whole token and can round down to zero.
- `max_buy_lamports` returns the largest gross input whose net amount fits the
  remaining migration headroom. Pass the live curve's `real_reserves_sol` and
  `migration_threshold_sol_snapshot`. At launch, use zero real reserves and the
  `protocol.migration_threshold_sol` field of `GlobalConfig`. The instruction
  checks trade status, reserve availability, and slippage. `MIN_SOL_IN_THRESHOLD`
  is the minimum gross buy that can update the last buyer and timer.
- Rust indexers can use these helpers directly. Frontend ports can use `BigInt`
  and check overflow at the same product and output widths. The
  [math tests](./tests/math.rs) pin launch, dust, fee, and maximum-buy cases.

The `constants::program` module includes the initial reserves, token supply and
decimals, `constraints`, and `math::FEE_DENOMINATOR`. These constants are
available with any feature set.

## Examples

The [`examples/`](./examples) directory contains minimal, runnable examples that send transactions to a Solana RPC. Each example shows only the logic for that instruction and its direct dependencies.

### Prerequisites

Create a `.env` file in `rust/` with:

```env
TEST_RPC_URL=https://api.devnet.solana.com
TEST_PAYER_PATH=/path/to/your/keypair.json
```

Assumptions:
- The global config is already initialized.
- The payer and fee recipients are already funded (no airdrops).
- Replace placeholder pubkeys in each example with values relevant to your setup.

### Running examples

```bash
cd rust
cargo run --example launch
cargo run --example buy_exact
cargo run --example sell_exact
cargo run --example fund_migration
cargo run --example define_winner_idempotent
cargo run --example claim_prize_idempotent
cargo run --example claim_creator_fee
cargo run --example migrate  # runs prepare_migration + migrate together
```

> **Note:** These examples hit a real RPC and mutate on-chain state. Use devnet and a dedicated test keypair. Admin/dev-gated instructions are not included.

## License
Apache-2.0
