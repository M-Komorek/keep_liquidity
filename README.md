# Liquidity Pool Model in Rust

This repository implements a mathematical model of a Liquidity Pool inspired by the Marinade Protocol. The model *will* provides immediate liquidity unstaking, allowing users to swap between Token and StakedToken using a pool of liquidity managed by liquidity providers.

## Overview
The Liquidity Pool model features three token types:
  - Token: The base unit.
  - StakedToken: A unit backed by a specific amount of Token, with an exchange ratio determined by the price.
  - LpToken: A unit representing a share of the liquidity pool. Liquidity providers receive LpToken when they add liquidity to the pool and can redeem them for a proportional amount of Token and StakedToken.

The model includes two main actors:
  - Swapper: An actor who exchanges StakedToken for Token.
  - Liquidity Provider: An actor who provides liquidity to the pool in exchange for LpToken.

The Liquidity Pool supports the following operations:
  - `LiquidityPool::init`: Initializes the liquidity pool with configuration parameters such as price, fee range, and liquidity target.
  - `LiquidityPool::add_liquidity`: Adds Token to the pool, minting new LpToken.
  - `LiquidityPool::remove_liquidity`: Redeems LpToken for a proportional amount of Token and StakedToken.
  - `LiquidityPool::swap`: Swaps StakedToken for Token, applying a fee based on the pool's configuration.

## TODO
  - Finish implementation - *LiquidityPool::remove_liquidity*.
  - Write the missing tests.
