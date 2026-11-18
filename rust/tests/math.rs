use lastbuy_sdk::constants::*;

#[test]
fn test_protocol_constants() {
    assert_eq!(program::TOTAL_SUPPLY, 1_000_000_000_000_000);
    assert_eq!(program::TOKEN_DECIMALS, 6);
    assert_eq!(program::INITIAL_VIRTUAL_SOL_RESERVES, 20_000_000_000);
    assert_eq!(program::INITIAL_VIRTUAL_TOKEN_RESERVES, 1_088_900_000_000_000);
    assert_eq!(program::constraints::MIN_SOL_IN_THRESHOLD, 1_000_000);
    assert_eq!(program::constraints::MAX_TOKEN_NAME_LEN, 64);
    assert_eq!(program::constraints::MAX_TOKEN_SYMBOL_LEN, 20);
    assert_eq!(program::constraints::MAX_TOKEN_URI_LEN, 200);
    assert_eq!(program::constraints::MAX_ALLOWED_FEE_BPS, 500);
    assert_eq!(program::math::FEE_DENOMINATOR, 10_000);
}

#[cfg(feature = "math")]
mod quotes {
    use lastbuy_sdk::{constants::*, error::*, math::*};

    const BUY_VIRTUAL_SOL: u64 = 20_980_000_000;
    const BUY_VIRTUAL_TOKENS: u64 = 1_038_036_224_976_167;
    const BUY_TOKENS_OUT: u64 = 50_863_775_023_833;

    #[test]
    fn test_buy_and_sell_round_trip() {
        let buy = calculate_buy_amount_out(
            INITIAL_VIRTUAL_SOL_RESERVES,
            INITIAL_VIRTUAL_TOKEN_RESERVES,
            980_000_000,
        )
        .unwrap();
        assert_eq!(
            buy,
            BuyResult {
                new_virtual_sol: BUY_VIRTUAL_SOL,
                new_virtual_tokens: BUY_VIRTUAL_TOKENS,
                tokens_out: BUY_TOKENS_OUT,
            }
        );

        let sell =
            calculate_sell_amount_out(buy.new_virtual_sol, buy.new_virtual_tokens, buy.tokens_out)
                .unwrap();
        assert_eq!(
            sell,
            SellResult {
                new_virtual_sol: INITIAL_VIRTUAL_SOL_RESERVES,
                new_virtual_tokens: INITIAL_VIRTUAL_TOKEN_RESERVES,
                gross_sol_out: 980_000_000,
            }
        );
    }

    #[test]
    fn test_sell_rounding() {
        assert_eq!(
            calculate_sell_amount_out(
                INITIAL_VIRTUAL_SOL_RESERVES,
                INITIAL_VIRTUAL_TOKEN_RESERVES,
                INITIAL_VIRTUAL_TOKEN_RESERVES,
            )
            .unwrap(),
            SellResult {
                new_virtual_sol: 10_000_000_000,
                new_virtual_tokens: 2_177_800_000_000_000,
                gross_sol_out: 10_000_000_000,
            }
        );
        assert_eq!(
            calculate_sell_amount_out(
                INITIAL_VIRTUAL_SOL_RESERVES,
                INITIAL_VIRTUAL_TOKEN_RESERVES,
                217_780_000_000_000,
            )
            .unwrap(),
            SellResult {
                new_virtual_sol: 16_666_666_667,
                new_virtual_tokens: 1_306_680_000_000_000,
                gross_sol_out: 3_333_333_333,
            }
        );
        assert!(matches!(
            calculate_sell_amount_out(BUY_VIRTUAL_SOL, BUY_VIRTUAL_TOKENS, 49_477),
            Err(SdkError::Math(MathError::InsufficientOutputAmount))
        ));
        assert_eq!(
            calculate_sell_amount_out(BUY_VIRTUAL_SOL, BUY_VIRTUAL_TOKENS, 49_478)
                .unwrap()
                .gross_sol_out,
            1
        );
    }

    #[test]
    fn test_invalid_trade_quotes() {
        assert!(matches!(
            calculate_buy_amount_out(10, 10, 0),
            Err(SdkError::Math(MathError::InsufficientOutputAmount))
        ));
        assert!(matches!(
            calculate_buy_amount_out(0, 1, 0),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert!(matches!(
            calculate_sell_amount_out(1, 0, 0),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert!(matches!(
            calculate_buy_amount_out(u64::MAX, 1, 1),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert!(matches!(
            calculate_sell_amount_out(1, u64::MAX, 1),
            Err(SdkError::Math(MathError::Overflow))
        ));
    }

    #[test]
    fn test_swap_fee_redistribution() {
        assert_eq!(FEE_DENOMINATOR, 10_000);
        assert_eq!(
            calculate_swap_fee_split(501, 100, 30, 70, false).unwrap(),
            SwapFeeSplit {
                total_fee: 9,
                protocol_fee: 5,
                creator_fee: 1,
                last_buyer_fee: 3,
            }
        );
        assert_eq!(
            calculate_swap_fee_split(501, 100, 30, 70, true).unwrap(),
            SwapFeeSplit {
                total_fee: 9,
                protocol_fee: 6,
                creator_fee: 3,
                last_buyer_fee: 0,
            }
        );
    }

    #[test]
    fn test_swap_fee_rounding_and_overflow() {
        assert_eq!(
            calculate_swap_fee_split(99, 100, 100, 100, false)
                .unwrap()
                .total_fee,
            0
        );
        assert!(calculate_swap_fee_split(u64::MAX / 100, 100, 0, 0, false).is_ok());
        assert!(matches!(
            calculate_swap_fee_split(u64::MAX / 100 + 1, 100, 0, 0, false),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert_eq!(
            calculate_swap_fee_split(u64::MAX, 0, 0, 0, true)
                .unwrap()
                .total_fee,
            0
        );
        assert_eq!(
            calculate_swap_fee_split(1_000, 501, 0, 0, false)
                .unwrap()
                .total_fee,
            50
        );
    }

    #[test]
    fn test_market_cap_lamports() {
        assert_eq!(
            market_cap_lamports(INITIAL_VIRTUAL_SOL_RESERVES, INITIAL_VIRTUAL_TOKEN_RESERVES)
                .unwrap(),
            18_367_159_518
        );
        assert!(matches!(
            market_cap_lamports(1, 0),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert!(matches!(
            market_cap_lamports(u64::MAX, 1),
            Err(SdkError::Math(MathError::Overflow))
        ));
    }

    #[test]
    fn test_price_lamports() {
        assert_eq!(
            price_lamports(INITIAL_VIRTUAL_SOL_RESERVES, INITIAL_VIRTUAL_TOKEN_RESERVES).unwrap(),
            18
        );
        assert_eq!(price_lamports(1, 1_000_001).unwrap(), 0);
        assert!(matches!(
            price_lamports(0, 0),
            Err(SdkError::Math(MathError::Overflow))
        ));
        assert!(matches!(
            price_lamports(u64::MAX, 1),
            Err(SdkError::Math(MathError::Overflow))
        ));
    }

    #[test]
    fn test_max_buy_limits() {
        assert_eq!(max_buy_lamports(10, 20, 0, 0, 0).unwrap(), 10);
        assert_eq!(max_buy_lamports(20, 20, 100, 100, 100).unwrap(), 0);
        assert_eq!(max_buy_lamports(21, 20, 100, 100, 100).unwrap(), 0);
        assert_eq!(
            max_buy_lamports(0, u64::MAX, 100, 0, 0).unwrap(),
            u64::MAX / 100
        );
        assert_eq!(max_buy_lamports(0, u64::MAX, 0, 0, 0).unwrap(), u64::MAX);
        assert_eq!(max_buy_lamports(0, 100, 200, 200, 101).unwrap(), 105);
    }

    #[test]
    fn test_max_buy_with_high_fee_rates() {
        assert_eq!(max_buy_lamports(0, 1, 1_111, 3_333, 5_554).unwrap(), 3_565);
        assert_eq!(
            max_buy_lamports(0, 1, 3_333, 3_333, 3_334).unwrap(),
            5_532_916_638_785_108
        );
        assert_eq!(max_buy_lamports(0, 1, 3_333, 3_333, 3_335).unwrap(), 25_997);
    }

    #[test]
    fn test_max_buy_with_independent_fee_floors() {
        // Gross 99 nets 99, gross 100 nets 97, and gross 101 nets 98.
        assert_eq!(max_buy_lamports(0, 97, 100, 100, 100).unwrap(), 100);

        let fee_rates = [(0, 0, 0), (100, 30, 70), (100, 100, 100), (500, 0, 0)];
        for (protocol_bps, creator_bps, last_buyer_bps) in fee_rates {
            for headroom in [1, 96, 97, 98, 99, 100, 333, 499, 500, 1_000] {
                let expected = (0..=2_000)
                    .rev()
                    .find(|&gross| {
                        let fees = calculate_swap_fee_split(
                            gross,
                            protocol_bps,
                            creator_bps,
                            last_buyer_bps,
                            false,
                        )
                        .unwrap();
                        gross
                            .checked_sub(fees.total_fee)
                            .is_some_and(|net| net <= headroom)
                    })
                    .unwrap();

                assert_eq!(
                    max_buy_lamports(0, headroom, protocol_bps, creator_bps, last_buyer_bps)
                        .unwrap(),
                    expected,
                    "headroom {headroom}, fees {protocol_bps}/{creator_bps}/{last_buyer_bps} bps"
                );
            }
        }
    }
}
