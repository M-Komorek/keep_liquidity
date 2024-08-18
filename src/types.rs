use core::fmt;

use crate::fixed_point_decimal::FixedPointDecimal;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TokenAmount(pub FixedPointDecimal);

impl From<u64> for TokenAmount {
    fn from(value: u64) -> Self {
        TokenAmount(FixedPointDecimal::from(value))
    }
}

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StakedTokenAmount(pub FixedPointDecimal);

impl From<u64> for StakedTokenAmount {
    fn from(value: u64) -> Self {
        StakedTokenAmount(FixedPointDecimal::from(value))
    }
}

impl fmt::Display for StakedTokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LpTokenAmount(pub FixedPointDecimal);

impl From<u64> for LpTokenAmount {
    fn from(value: u64) -> Self {
        LpTokenAmount(FixedPointDecimal::from(value))
    }
}

impl fmt::Display for LpTokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default)]
pub struct Price(pub FixedPointDecimal);

impl From<u64> for Price {
    fn from(value: u64) -> Self {
        Price(FixedPointDecimal::from(value))
    }
}

#[derive(Debug, Default)]
pub struct Percentage(pub FixedPointDecimal);

impl From<u64> for Percentage {
    fn from(value: u64) -> Self {
        Percentage(FixedPointDecimal::from(value))
    }
}
