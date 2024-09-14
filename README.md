# Keep Liquidity

Keep Liquidity is a Rust-based Liquidity Pool protocol inspired by the [Marinade Protocol](https://docs.marinade.finance/marinade-protocol/system-overview/unstake-liquidity-pool). The core idea is to facilitate smooth swaps between liquid tokens and staked tokens while empowering liquidity providers to manage the pool. Two main actors interact with the pool:
  - Swappers who exchange Token for StakedToken (and vice versa).
  - Liquidity Providers who stake their assets into the pool to earn passive returns.

## 🌟 Features
  - Token Swap: Seamlessly swap between Token and StakedToken assets.
  - LP Management: Add and remove liquidity using LpToken to earn rewards.
  - Customizable Fees: Adjustable fees for token swaps.
  - Rust Implementation: Built with the speed and safety of Rust.

## Usage
Initialize the Pool
```rust
let liquidity_pool = LiquidityPool::init(price, liquidity_target, min_fee, max_fee);
```

Add Liquidity
```rust
liquidity_pool.add_liquidity(amount_of_new_tokens);
```

Swap Tokens
```rust
liquidity_pool.swap(staked_token_amount);
```

Remove Liquidity
```rust
liquidity_pool.remove_liquidity(lp_token_amount);
```

## Tests

Run the tests to verify everything works as expected:
```bash
cargo test
```
