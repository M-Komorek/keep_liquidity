mod fixed_point_decimal;
mod liquidity_pool;
mod tokens;
mod utils;

use crate::fixed_point_decimal::FixedPointDecimal;
use crate::liquidity_pool::LiquidityPool;
use crate::tokens::{LpTokenAmount, StakedTokenAmount, TokenAmount};
use crate::utils::{Percentage, Price};

fn main() {
    let price = Price(FixedPointDecimal::try_from(1.5).unwrap());
    let min_fee = Percentage(FixedPointDecimal::try_from(0.001).unwrap());
    let max_fee = Percentage(FixedPointDecimal::try_from(0.09).unwrap());
    let liquidity_target = TokenAmount(FixedPointDecimal::try_from(90.0).unwrap());

    let mut liquidity_pool = LiquidityPool::init(price, liquidity_target, min_fee, max_fee);
    println!("Liquidity pool init done");
    println!("{}", liquidity_pool);

    let lp_tokens = liquidity_pool
        .add_liquidity(TokenAmount(FixedPointDecimal::try_from(100).unwrap()))
        .unwrap();
    println!("100 tokens has beed added: {}", lp_tokens);
    println!("{}", liquidity_pool);

    let swapped = liquidity_pool
        .swap(StakedTokenAmount(FixedPointDecimal::try_from(6).unwrap()))
        .unwrap();
    println!("6 stacked tokens has beed swapped: {}", swapped);
    println!("{}", liquidity_pool);

    let lp_tokens = liquidity_pool
        .add_liquidity(TokenAmount(FixedPointDecimal::try_from(10).unwrap()))
        .unwrap();
    println!("10 tokens has beed added: {}", lp_tokens);
    println!("{}", liquidity_pool);

    let swapped = liquidity_pool
        .swap(StakedTokenAmount(FixedPointDecimal::try_from(30).unwrap()))
        .unwrap();
    println!("30 stacked tokens has beed swapped: {}", swapped);
    println!("{}", liquidity_pool);

    let (returned_token_amount, returned_staked_token_amount) = liquidity_pool
        .remove_liquidity(LpTokenAmount(
            FixedPointDecimal::try_from(109.9991).unwrap(),
        ))
        .unwrap();

    println!("109.9991 lp tokens has been removed: returned_token_amount: {} returned_staked_token_amount: {}", returned_token_amount, returned_staked_token_amount);
    println!("{}", liquidity_pool);
}
