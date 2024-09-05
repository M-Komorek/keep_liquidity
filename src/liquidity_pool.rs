use crate::fixed_point_decimal::FixedPointError;
use crate::tokens::{LpTokenAmount, StakedTokenAmount, TokenAmount};
use crate::utils::{Percentage, Price};
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

    pub fn add_liquidity(
        &mut self,
        amount_of_new_tokens: TokenAmount,
    ) -> Result<LpTokenAmount, FixedPointError> {
        let staked_token_value = self.calculate_staked_token_value(&self.staked_token_amount)?;
        let total_pool_tokens = (self.token_amount.0 + staked_token_value.0)?;

        let minted_token_amount = if total_pool_tokens == FixedPointDecimal::try_from(0u64).unwrap()
        {
            amount_of_new_tokens.0
        } else {
            ((amount_of_new_tokens.0 * self.lp_token_amount.0)? / total_pool_tokens)?
        };

        self.token_amount.0 += minted_token_amount;
        self.lp_token_amount.0 += minted_token_amount;

        Ok(LpTokenAmount(minted_token_amount))
    }

    #[allow(dead_code)]
    pub fn remove_liquidity(
        &mut self,
        _lp_token_amount: LpTokenAmount,
    ) -> (TokenAmount, StakedTokenAmount) {
        unimplemented!()
    }

    pub fn swap(
        &mut self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, FixedPointError> {
        let staked_token_value = self.calculate_staked_token_value(&staked_token_amount)?;
        let final_token_amount = (self.token_amount.0 - staked_token_value.0)?;
        let fee_percentage = self.calculate_fee(TokenAmount(final_token_amount))?;
        let converted_tokens =
            (staked_token_value.0 * (FixedPointDecimal::try_from(1)? - fee_percentage.0)?)?;

        self.staked_token_amount.0 += staked_token_amount.0;
        self.token_amount.0 -= converted_tokens;

        Ok(TokenAmount(converted_tokens))
    }

    fn calculate_fee(
        &self,
        final_token_amount: TokenAmount,
    ) -> Result<Percentage, FixedPointError> {
        if final_token_amount.0 > self.liquidity_target.0 {
            Ok(Percentage(self.min_fee.0))
        } else {
            let max_min_fee_difference = (self.max_fee.0 - self.min_fee.0).unwrap();
            let sub_fee_calculation =
                ((max_min_fee_difference * final_token_amount.0)? / self.liquidity_target.0)?;

            Ok(Percentage((self.max_fee.0 - sub_fee_calculation)?))
        }
    }

    fn calculate_staked_token_value(
        &self,
        staked_token_amount: &StakedTokenAmount,
    ) -> Result<TokenAmount, FixedPointError> {
        Ok(TokenAmount((self.price.0 * staked_token_amount.0)?))
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
            let lp_tokens = sut
                .add_liquidity(TokenAmount(FixedPointDecimal::try_from(100).unwrap()))
                .unwrap();

            assert_eq!(lp_tokens.0, 100);
            assert_eq!(sut.token_amount.0, 100);
            assert_eq!(sut.staked_token_amount.0, 0);
            assert_eq!(sut.lp_token_amount.0, 100);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_some_tokens_and_no_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount(FixedPointDecimal::try_from(100).unwrap()),
                StakedTokenAmount::default(),
                LpTokenAmount(FixedPointDecimal::try_from(100).unwrap()),
            );
            let lp_tokens = sut
                .add_liquidity(TokenAmount(FixedPointDecimal::try_from(10).unwrap()))
                .unwrap();

            assert_eq!(lp_tokens.0, 10);
            assert_eq!(sut.token_amount.0, 110);
            assert_eq!(sut.staked_token_amount.0, 0);
            assert_eq!(sut.lp_token_amount.0, 110);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_no_tokens_and_some_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount::default(),
                StakedTokenAmount(FixedPointDecimal::try_from(10).unwrap()),
                LpTokenAmount(FixedPointDecimal::try_from(15).unwrap()),
            );
            let lp_tokens = sut
                .add_liquidity(TokenAmount(FixedPointDecimal::try_from(10).unwrap()))
                .unwrap();

            assert_eq!(lp_tokens.0, 10);
            assert_eq!(sut.token_amount.0, 10);
            assert_eq!(sut.staked_token_amount.0, 10);
            assert_eq!(sut.lp_token_amount.0, 25);
        }

        #[test]
        fn calculates_lp_tokens_correctly_on_pool_with_some_tokens_and_some_staked_tokens() {
            let mut sut = create_sut(
                TokenAmount(FixedPointDecimal::try_from(10).unwrap()),
                StakedTokenAmount(FixedPointDecimal::try_from(10).unwrap()),
                LpTokenAmount(FixedPointDecimal::try_from(25).unwrap()),
            );
            let lp_tokens = sut
                .add_liquidity(TokenAmount(FixedPointDecimal::try_from(10).unwrap()))
                .unwrap();

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
                TokenAmount(FixedPointDecimal::try_from(1000).unwrap()),
                StakedTokenAmount::default(),
                LpTokenAmount::default(),
            );
            let tokens = sut
                .swap(StakedTokenAmount(FixedPointDecimal::try_from(10).unwrap()))
                .unwrap();

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
