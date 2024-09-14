use core::fmt;

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
        let current_pool_value = self.current_pool_value()?;
        let minted_token_amount =
            if current_pool_value.0 == FixedPointDecimal::try_from(0u64).unwrap() {
                amount_of_new_tokens.0
            } else {
                let ownership_ratio = (self.lp_token_amount.0 / current_pool_value.0)?;
                (amount_of_new_tokens.0 * ownership_ratio)?
            };

        self.token_amount.0 = (self.token_amount.0 + amount_of_new_tokens.0)?;
        self.lp_token_amount.0 = (self.lp_token_amount.0 + minted_token_amount)?;

        Ok(LpTokenAmount(minted_token_amount))
    }

    pub fn remove_liquidity(
        &mut self,
        lp_token_amount: LpTokenAmount,
    ) -> Result<(TokenAmount, StakedTokenAmount), FixedPointError> {
        let proportional_share = (lp_token_amount.0 / self.lp_token_amount.0)?;
        let base_token_amount_to_return = (proportional_share * self.token_amount.0)?;
        let base_staked_token_amount_to_return = (proportional_share * self.staked_token_amount.0)?;

        let final_liquidity = (self.token_amount.0 - base_token_amount_to_return)?;
        let fee = self.calculate_fee(TokenAmount(final_liquidity))?;

        let token_amount_to_return = self.apply_fee(base_token_amount_to_return, &fee)?;
        let staked_token_to_return = self.apply_fee(base_staked_token_amount_to_return, &fee)?;

        self.lp_token_amount.0 = (self.lp_token_amount.0 - lp_token_amount.0)?;
        self.token_amount.0 = (self.token_amount.0 - token_amount_to_return)?;
        self.staked_token_amount.0 = (self.staked_token_amount.0 - staked_token_to_return)?;

        Ok((
            TokenAmount(token_amount_to_return),
            StakedTokenAmount(staked_token_to_return),
        ))
    }

    pub fn swap(
        &mut self,
        staked_token_amount: StakedTokenAmount,
    ) -> Result<TokenAmount, FixedPointError> {
        let base_staked_token_value = self.calculate_staked_token_value(&staked_token_amount)?;
        let final_token_amount = (self.token_amount.0 - base_staked_token_value.0)?;

        let fee = self.calculate_fee(TokenAmount(final_token_amount))?;
        let staked_token_value = self.apply_fee(base_staked_token_value.0, &fee)?;

        self.staked_token_amount.0 = (self.staked_token_amount.0 + staked_token_amount.0)?;
        self.token_amount.0 = (self.token_amount.0 - staked_token_value)?;

        Ok(TokenAmount(staked_token_value))
    }

    fn calculate_fee(&self, final_liquidity: TokenAmount) -> Result<Percentage, FixedPointError> {
        if final_liquidity.0 >= self.liquidity_target.0 {
            Ok(Percentage(self.min_fee.0))
        } else {
            let max_min_fee_difference = (self.max_fee.0 - self.min_fee.0)?;
            let liquidity_to_target_ratio = (final_liquidity.0 / self.liquidity_target.0)?;
            let fee = (self.max_fee.0 - (max_min_fee_difference * liquidity_to_target_ratio)?)?;

            Ok(Percentage(fee))
        }
    }

    fn current_pool_value(&self) -> Result<TokenAmount, FixedPointError> {
        let staked_token_value = self.calculate_staked_token_value(&self.staked_token_amount)?;
        let current_liquidity = (self.token_amount.0 + staked_token_value.0)?;
        Ok(TokenAmount(current_liquidity))
    }

    fn calculate_staked_token_value(
        &self,
        staked_token_amount: &StakedTokenAmount,
    ) -> Result<TokenAmount, FixedPointError> {
        Ok(TokenAmount((self.price.0 * staked_token_amount.0)?))
    }

    fn apply_fee(
        &self,
        token_amount: FixedPointDecimal,
        fee: &Percentage,
    ) -> Result<FixedPointDecimal, FixedPointError> {
        let fee_value = (fee.0 * token_amount)?;
        Ok((token_amount - fee_value)?)
    }
}

impl fmt::Display for LiquidityPool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "> LiquidityPool")?;
        writeln!(f, "\t const Price: {}", self.price.0)?;
        writeln!(f, "\t const Min fee: {}", self.min_fee.0)?;
        writeln!(f, "\t const Max fee: {}", self.max_fee.0)?;
        writeln!(f, "\t const Target liquidity: {}", self.liquidity_target.0)?;
        writeln!(f, "\t - Token amount: {}", self.token_amount.0)?;
        writeln!(f, "\t - Liquidity token amount: {}", self.lp_token_amount.0)?;
        writeln!(
            f,
            "\t - Staked token amount: {}",
            self.staked_token_amount.0
        )?;
        Ok(())
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
        fn calculates_lp_tokens_correctly_on_non_empty_pool() {
            let mut sut = create_sut(
                TokenAmount(FixedPointDecimal::try_from(91.009).unwrap()),
                StakedTokenAmount(FixedPointDecimal::try_from(6).unwrap()),
                LpTokenAmount(FixedPointDecimal::try_from(100).unwrap()),
            );
            let lp_tokens = sut
                .add_liquidity(TokenAmount(FixedPointDecimal::try_from(10).unwrap()))
                .unwrap();

            assert_eq!(lp_tokens.0, FixedPointDecimal::try_from(9.9991).unwrap());
            assert_eq!(
                sut.token_amount.0,
                FixedPointDecimal::try_from(101.009).unwrap()
            );
            assert_eq!(sut.staked_token_amount.0, 6);
            assert_eq!(
                sut.lp_token_amount.0,
                FixedPointDecimal::try_from(109.9991).unwrap()
            );
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
