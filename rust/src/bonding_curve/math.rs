//! Integer quotes and derived metrics for the LastBuy bonding curve.

use std::cmp::Ordering;

use crate::{constants::*, error::*};

/// Virtual reserves after a buy and the number of token atoms received.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuyResult {
    pub new_virtual_sol: u64,
    pub new_virtual_tokens: u64,
    pub tokens_out: u64,
}

/// Virtual reserves after a sell and the SOL output in lamports before fees.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SellResult {
    pub new_virtual_sol: u64,
    pub new_virtual_tokens: u64,
    pub gross_sol_out: u64,
}

/// Total swap fee and its allocation in lamports.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwapFeeSplit {
    pub total_fee: u64,
    pub protocol_fee: u64,
    pub creator_fee: u64,
    pub last_buyer_fee: u64,
}

/// Quotes a buy from a net SOL input in lamports and virtual reserves in
/// lamports and token atoms. The new token reserve is rounded down.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] on overflow or division by zero and
/// [`MathError::InsufficientOutputAmount`] if the quote yields zero token atoms.
pub fn calculate_buy_amount_out(
    virtual_reserves_sol: u64,
    virtual_reserves_tokens: u64,
    net_sol: u64,
) -> SdkResult<BuyResult> {
    let virtual_sol = u128::from(virtual_reserves_sol);
    let virtual_tokens = u128::from(virtual_reserves_tokens);
    let invariant = virtual_sol
        .checked_mul(virtual_tokens)
        .ok_or(MathError::Overflow)?;

    let new_virtual_sol = virtual_sol
        .checked_add(u128::from(net_sol))
        .ok_or(MathError::Overflow)?;
    let new_virtual_tokens = invariant
        .checked_div(new_virtual_sol)
        .ok_or(MathError::Overflow)?;
    let tokens_out = virtual_tokens
        .checked_sub(new_virtual_tokens)
        .ok_or(MathError::Overflow)?;

    let new_virtual_sol = to_u64(new_virtual_sol)?;
    let new_virtual_tokens = to_u64(new_virtual_tokens)?;
    let tokens_out = to_u64(tokens_out)?;

    if tokens_out == 0 {
        return Err(MathError::InsufficientOutputAmount.into());
    }

    Ok(BuyResult {
        new_virtual_sol,
        new_virtual_tokens,
        tokens_out,
    })
}

/// Quotes a sell from token atoms and virtual reserves in lamports and token
/// atoms. Rounds the new SOL reserve up. Apply swap fees to the gross output
/// in lamports.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] on overflow or division by zero and
/// [`MathError::InsufficientOutputAmount`] if the quote yields zero lamports.
pub fn calculate_sell_amount_out(
    virtual_reserves_sol: u64,
    virtual_reserves_tokens: u64,
    tokens_in: u64,
) -> SdkResult<SellResult> {
    let virtual_sol = u128::from(virtual_reserves_sol);
    let virtual_tokens = u128::from(virtual_reserves_tokens);
    let invariant = virtual_sol
        .checked_mul(virtual_tokens)
        .ok_or(MathError::Overflow)?;

    let new_virtual_tokens = virtual_tokens
        .checked_add(u128::from(tokens_in))
        .ok_or(MathError::Overflow)?;
    let rounding_offset = new_virtual_tokens
        .checked_sub(1)
        .ok_or(MathError::Overflow)?;
    let new_virtual_sol = invariant
        .checked_add(rounding_offset)
        .and_then(|numerator| numerator.checked_div(new_virtual_tokens))
        .ok_or(MathError::Overflow)?;
    let gross_sol_out = virtual_sol
        .checked_sub(new_virtual_sol)
        .ok_or(MathError::Overflow)?;

    let new_virtual_sol = to_u64(new_virtual_sol)?;
    let new_virtual_tokens = to_u64(new_virtual_tokens)?;
    let gross_sol_out = to_u64(gross_sol_out)?;

    if gross_sol_out == 0 {
        return Err(MathError::InsufficientOutputAmount.into());
    }

    Ok(SellResult {
        new_virtual_sol,
        new_virtual_tokens,
        gross_sol_out,
    })
}

/// Calculates swap fees from a gross amount in lamports and rates in basis
/// points. Each fee is rounded down before redistribution.
///
/// Once the canonical winner is defined, the protocol and creator share the
/// last buyer fee. The creator receives the extra lamport on odd amounts.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] if a fee product or sum exceeds `u64`.
pub fn calculate_swap_fee_split(
    amount: u64,
    protocol_curve_fee_bps: u16,
    creator_fee_bps: u16,
    last_buyer_fee_bps: u16,
    is_winner_defined: bool,
) -> SdkResult<SwapFeeSplit> {
    let mut protocol_fee = calculate_fee(amount, protocol_curve_fee_bps)?;
    let mut creator_fee = calculate_fee(amount, creator_fee_bps)?;
    let mut last_buyer_fee = calculate_fee(amount, last_buyer_fee_bps)?;

    if is_winner_defined {
        let protocol_share = last_buyer_fee.checked_div(2).ok_or(MathError::Overflow)?;
        let creator_share = last_buyer_fee
            .checked_sub(protocol_share)
            .ok_or(MathError::Overflow)?;

        protocol_fee = protocol_fee
            .checked_add(protocol_share)
            .ok_or(MathError::Overflow)?;
        creator_fee = creator_fee
            .checked_add(creator_share)
            .ok_or(MathError::Overflow)?;
        last_buyer_fee = 0;
    }

    let total_fee = protocol_fee
        .checked_add(creator_fee)
        .and_then(|fee| fee.checked_add(last_buyer_fee))
        .ok_or(MathError::Overflow)?;

    Ok(SwapFeeSplit {
        total_fee,
        protocol_fee,
        creator_fee,
        last_buyer_fee,
    })
}

/// Calculates market cap in lamports from virtual SOL lamports and token atoms.
/// Scales by the total supply in atoms before rounding down.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] if the token reserve is zero or the result
/// exceeds `u64`.
pub fn market_cap_lamports(virtual_sol: u64, virtual_tokens: u64) -> SdkResult<u64> {
    reserve_value_lamports(virtual_sol, virtual_tokens, TOTAL_SUPPLY)
}

/// Calculates the price of one whole token from virtual SOL lamports and token
/// atoms. The result is rounded down to whole lamports and may be zero.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] if the token reserve is zero or the result
/// exceeds `u64`.
pub fn price_lamports(virtual_sol: u64, virtual_tokens: u64) -> SdkResult<u64> {
    let atoms_per_token = 10_u64
        .checked_pow(u32::from(TOKEN_DECIMALS))
        .ok_or(MathError::Overflow)?;
    reserve_value_lamports(virtual_sol, virtual_tokens, atoms_per_token)
}

/// Calculates the largest gross buy in lamports whose net input fits the
/// remaining migration headroom after independently flooring each fee.
/// Reserves and the threshold are in lamports, and rates are in basis points.
/// Returns zero when the threshold has been reached.
///
/// # Errors
///
/// Returns [`MathError::Overflow`] on arithmetic overflow.
pub fn max_buy_lamports(
    real_reserves_sol: u64,
    migration_threshold_sol_snapshot: u64,
    protocol_curve_fee_bps: u16,
    creator_fee_bps: u16,
    last_buyer_fee_bps: u16,
) -> SdkResult<u64> {
    let remaining_lamports = migration_threshold_sol_snapshot
        .checked_sub(real_reserves_sol)
        .unwrap_or_default();
    if remaining_lamports == 0 {
        return Ok(0);
    }

    let largest_fee_bps = protocol_curve_fee_bps
        .max(creator_fee_bps)
        .max(last_buyer_fee_bps);
    let fee_product_bound = u64::MAX
        .checked_div(u64::from(largest_fee_bps.max(1)))
        .ok_or(MathError::Overflow)?;
    let total_fee_bps = u128::from(protocol_curve_fee_bps)
        .checked_add(u128::from(creator_fee_bps))
        .and_then(|total| total.checked_add(u128::from(last_buyer_fee_bps)))
        .ok_or(MathError::Overflow)?;

    // The three fee floors add less than three lamports to proportional net.
    let denominator = u128::from(FEE_DENOMINATOR);
    let rounding_bound = denominator.checked_mul(3).ok_or(MathError::Overflow)?;
    let (gross_bound, search_window) = match total_fee_bps.cmp(&denominator) {
        Ordering::Less => {
            let net_bps = denominator
                .checked_sub(total_fee_bps)
                .ok_or(MathError::Overflow)?;
            let gross_bound = u128::from(remaining_lamports)
                .checked_mul(denominator)
                .and_then(|scaled| scaled.checked_div(net_bps))
                .ok_or(MathError::Overflow)?;
            let search_window = rounding_bound
                .checked_div(net_bps)
                .and_then(|window| window.checked_add(1))
                .ok_or(MathError::Overflow)?;
            (gross_bound, search_window)
        }
        // At 100% the exact net repeats every FEE_DENOMINATOR lamports.
        Ordering::Equal => (u128::from(fee_product_bound), denominator),
        Ordering::Greater => {
            let excess_bps = total_fee_bps
                .checked_sub(denominator)
                .ok_or(MathError::Overflow)?;
            let nonnegative_bound = rounding_bound
                .checked_div(excess_bps)
                .ok_or(MathError::Overflow)?;
            (nonnegative_bound, nonnegative_bound)
        }
    };
    let max_gross = to_u64(gross_bound.min(u128::from(fee_product_bound)))?;
    let min_gross = max_gross
        .checked_sub(to_u64(search_window)?)
        .unwrap_or_default();

    for gross in (min_gross..=max_gross).rev() {
        let fees = calculate_swap_fee_split(
            gross,
            protocol_curve_fee_bps,
            creator_fee_bps,
            last_buyer_fee_bps,
            false,
        )?;
        if gross
            .checked_sub(fees.total_fee)
            .is_some_and(|net| net <= remaining_lamports)
        {
            return Ok(gross);
        }
    }

    Err(MathError::Overflow.into())
}

fn calculate_fee(amount: u64, fee_bps: u16) -> SdkResult<u64> {
    amount
        .checked_mul(u64::from(fee_bps))
        .and_then(|product| product.checked_div(u64::from(FEE_DENOMINATOR)))
        .ok_or(MathError::Overflow.into())
}

fn reserve_value_lamports(
    virtual_sol: u64,
    virtual_tokens: u64,
    token_atoms: u64,
) -> SdkResult<u64> {
    let lamports = u128::from(virtual_sol)
        .checked_mul(u128::from(token_atoms))
        .and_then(|scaled| scaled.checked_div(u128::from(virtual_tokens)))
        .ok_or(MathError::Overflow)?;
    to_u64(lamports)
}

fn to_u64(value: u128) -> SdkResult<u64> {
    u64::try_from(value).map_err(|_| MathError::Overflow.into())
}
