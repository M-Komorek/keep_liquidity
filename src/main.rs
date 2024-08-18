mod fixed_point_decimal;
mod liquidity_pool;
mod types;

use crate::fixed_point_decimal::FixedPointDecimal;
use crate::liquidity_pool::LiquidityPool;
use crate::types::{Percentage, Price, StakedTokenAmount, TokenAmount};

fn main() {
    let price = Price(FixedPointDecimal::try_from(1.5).unwrap());
    let min_fee = Percentage(FixedPointDecimal::try_from(0.001).unwrap());
    let max_fee = Percentage(FixedPointDecimal::try_from(0.09).unwrap());
    let liquidity_target = TokenAmount(FixedPointDecimal::try_from(90.0).unwrap());

    let mut liquidity_pool = LiquidityPool::init(price, liquidity_target, min_fee, max_fee);

    let lp_tokens = liquidity_pool.add_liquidity(TokenAmount(FixedPointDecimal::from(100)));
    println!("100 tokens has beed added: {}", lp_tokens);

    let swapped = liquidity_pool.swap(StakedTokenAmount(FixedPointDecimal::from(6)));
    println!("6 stacked tokens has beed swapped: {}", swapped.0);

    let lp_tokens = liquidity_pool.add_liquidity(TokenAmount(FixedPointDecimal::from(10)));
    println!("10 tokens has beed added: {}", lp_tokens.0);

    let swapped = liquidity_pool.swap(StakedTokenAmount(FixedPointDecimal::from(30)));
    println!("30 stacked tokens has beed swapped: {}", swapped.0);
}
