use crate::fixed_point_decimal::FixedPointDecimal;

#[derive(Debug, Default)]
pub struct Price(pub FixedPointDecimal);

#[derive(Debug, Default)]
pub struct Percentage(pub FixedPointDecimal);
