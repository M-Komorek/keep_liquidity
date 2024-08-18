use crate::types::{LpTokenAmount, Percentage, Price, StakedTokenAmount, TokenAmount};
use crate::FixedPointDecimal;

#[derive(Debug)]
pub struct LiquidityPool {
    price: Price,
    token_amount: TokenAmount,
    staked_token_amount: StakedTokenAmount,
    lp_token_amount: LpTokenAmount,
    liquidity_target: TokenAmount,
    min_fee: Percentage,
    max_fee: Percentage,
}

impl LiquidityPool {
    pub fn init(
        price: Price,
        liquidity_target: TokenAmount,
        min_fee: Percentage,
        max_fee: Percentage,
    ) -> Self {
        LiquidityPool {
            price,
            token_amount: TokenAmount::default(),
            staked_token_amount: StakedTokenAmount::default(),
            lp_token_amount: LpTokenAmount::default(),
            liquidity_target,
            min_fee,
            max_fee,
        }
    }

    pub fn add_liquidity(&mut self, amount_of_new_tokens: TokenAmount) -> LpTokenAmount {
        let total_pool_tokens = self.token_amount.0 + self.price.0 * self.staked_token_amount.0;

        let minted_token_amount = if total_pool_tokens == FixedPointDecimal::from(0) {
            amount_of_new_tokens.0
        } else {
            amount_of_new_tokens.0 * self.lp_token_amount.0 / total_pool_tokens
        };

        self.token_amount.0 += minted_token_amount;
        self.lp_token_amount.0 += minted_token_amount;

        LpTokenAmount(minted_token_amount)
    }

    #[allow(dead_code)]
    pub fn remove_liquidity(
        &mut self,
        _lp_token_amount: LpTokenAmount,
    ) -> (TokenAmount, StakedTokenAmount) {
        unimplemented!()
    }

    pub fn swap(&mut self, staked_token_amount: StakedTokenAmount) -> TokenAmount {
        let final_token_amount = self.token_amount.0 - staked_token_amount.0 * self.price.0;
        let fee_percentage = self.calculate_fee(TokenAmount(final_token_amount));
        let converted_tokens =
            staked_token_amount.0 * self.price.0 * (FixedPointDecimal::from(1) - fee_percentage.0);

        self.staked_token_amount.0 += staked_token_amount.0;
        self.token_amount.0 -= converted_tokens;

        TokenAmount(converted_tokens)
    }

    fn calculate_fee(&self, final_token_amount: TokenAmount) -> Percentage {
        if final_token_amount.0 > self.liquidity_target.0 {
            Percentage(self.min_fee.0)
        } else {
            Percentage(
                self.max_fee.0
                    - (self.max_fee.0 - self.min_fee.0) * final_token_amount.0
                        / self.liquidity_target.0,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_sut(
        token_amount: TokenAmount,
        staked_token_amount: StakedTokenAmount,
        lp_token_amount: LpTokenAmount,
    ) -> LiquidityPool {
        let price = Price(FixedPointDecimal::try_from(1.5).unwrap());
        let min_fee = Percentage(FixedPointDecimal::try_from(0.001).unwrap());
        let max_fee = Percentage(FixedPointDecimal::try_from(0.09).unwrap());
        let liquidity_target = TokenAmount(FixedPointDecimal::try_from(90.0).unwrap());

        LiquidityPool {
            price,
            token_amount,
            staked_token_amount,
            lp_token_amount,
            liquidity_target,
            min_fee,
            max_fee,
        }
    }

    mod add_liquidity {
        use super::*;

        #[test]
        fn calculates_lp_tokens_correctly_on_empty_pool() {
            let mut sut = create_sut(
                TokenAmount::default(),
                StakedTokenAmount::default(),
                LpTokenAmount::default(),
            );
            let lp_tokens = sut.add_liquidity(TokenAmount::from(100));

            assert_eq!(lp_tokens.0, 100);
            assert_eq!(sut.token_amount.0, 100);
            assert_eq!(sut.staked_token_amount.0, 0);
            assert_eq!(sut.lp_token_amount.0, 100);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_some_tokens_and_no_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount::from(100),
                StakedTokenAmount::default(),
                LpTokenAmount::from(100),
            );
            let lp_tokens = sut.add_liquidity(TokenAmount::from(10));
            assert_eq!(lp_tokens.0, 10);
            assert_eq!(sut.token_amount.0, 110);
            assert_eq!(sut.staked_token_amount.0, 0);
            assert_eq!(sut.lp_token_amount.0, 110);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_no_tokens_and_some_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount::default(),
                StakedTokenAmount::from(10),
                LpTokenAmount::from(15),
            );
            let lp_tokens = sut.add_liquidity(TokenAmount::from(10));
            assert_eq!(lp_tokens.0, 10);
            assert_eq!(sut.token_amount.0, 10);
            assert_eq!(sut.staked_token_amount.0, 10);
            assert_eq!(sut.lp_token_amount.0, 25);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_some_tokens_and_some_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount::from(10),
                StakedTokenAmount::from(10),
                LpTokenAmount::from(25),
            );
            let lp_tokens = sut.add_liquidity(TokenAmount::from(10));
            assert_eq!(lp_tokens.0, 10);
            assert_eq!(sut.token_amount.0, 20);
            assert_eq!(sut.staked_token_amount.0, 10);
            assert_eq!(sut.lp_token_amount.0, 35);
        }
    }

    mod swap {
        use super::*;

        #[test]
        fn should_swap_with_min_fee() {
            let mut sut = create_sut(
                TokenAmount::from(1000),
                StakedTokenAmount::default(),
                LpTokenAmount::default(),
            );

            let tokens = sut.swap(StakedTokenAmount::from(10));
            assert_eq!(tokens.0, FixedPointDecimal::try_from(14.985).unwrap());
            assert_eq!(
                sut.token_amount.0,
                FixedPointDecimal::try_from(985.015).unwrap()
            );
            assert_eq!(sut.staked_token_amount.0, 10);
        }

        #[test]
        fn should_swap_with_max_fee() {
            assert!(true);
        }
    }
}
