use core::fmt;

use crate::fixed_point_decimal::FixedPointDecimal;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct TokenAmount(pub FixedPointDecimal);

impl fmt::Display for TokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct StakedTokenAmount(pub FixedPointDecimal);

impl fmt::Display for StakedTokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LpTokenAmount(pub FixedPointDecimal);

impl fmt::Display for LpTokenAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
