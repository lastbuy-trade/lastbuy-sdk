# lastbuy-sdk

Low-level Rust SDK for the LastBuy bonding curve program on Solana.

This crate provides instruction builders, PDA derivation helpers, account/event types, discriminators, constants, and error types to interact with the LastBuy bonding curve program directly without pulling in a full high-level client.

An optional high-level client is planned and will live in this crate behind the `client` feature flag.

## Program
- Program ID: `Lbuyii1PVUvt9xToGuyUf6CQCDbdRRzzo8SsHuNYMkf` (same on mainnet-beta, devnet, and testnet)

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
lastbuy-sdk = "0.1.0"
```

By default the crate enables the `full` feature set. You can trim dependencies via feature flags:

```toml
[dependencies]
lastbuy-sdk = { version = "0.1.0", default-features = false, features = ["instructions", "pda", "types"] }
```

## Feature flags

| Feature | What it enables |
| --- | --- |
| `full` (default) | `events`, `accounts`, `instructions`, `pda`, `error` |
| `instructions` | Instruction builders (`buy_exact`, `sell_exact`, `launch`, etc.), requires `pda`, `types`, `error` |
| `accounts` | Account structs/deserialization helpers (`CurveState`, `GlobalConfig`, `PrizePool`, `PoolCreatorState`) |
| `events` | Anchor event types and discriminators |
| `types` | Shared types (`FeeConfig`, `ProtocolConfig`, `AdminConfig`, `RaydiumConfig`, `CurveStatus`) |
| `pda` | PDA derivation helpers for all program PDAs |
| `error` | `SdkError` / `SdkResult` |
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
    // can be fetched from `curve` PDA
    creator: curve_creator_pubkey,
    mint: mint_pubkey,
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

## Notes
- This is a **low-level** crate: it builds `solana_instruction::Instruction`s and derives accounts/PDAs, but does not send transactions, manage signers/wallets, or do RPC calls.
- Account fetching/deserialization is available via `accounts` feature; RPC integration is left to your client.
- API may evolve as the program changes (following the `0.1.x` series).

## License
Apache-2.0
