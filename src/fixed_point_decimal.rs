use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

const DECIMALS: u32 = 6;
const FACTOR: u64 = 10_u64.pow(DECIMALS);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointDecimal {
    value: u64,
}

#[derive(Debug, PartialEq)]
pub enum FixedPointError {
    Overflow,
    Underflow,
    DivisionByZero,
}

impl fmt::Display for FixedPointError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FixedPointError::Overflow => write!(f, "Overflow occurred during operation!"),
            FixedPointError::Underflow => write!(f, "Underflow occurred during operation!"),
            FixedPointError::DivisionByZero => write!(f, "Division by zero!"),
        }
    }
}

impl TryFrom<u64> for FixedPointDecimal {
    type Error = FixedPointError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if let Some(fixed_point_value) = value.checked_mul(FACTOR) {
            Ok(FixedPointDecimal {
                value: fixed_point_value,
            })
        } else {
            Err(FixedPointError::Overflow)
        }
    }
}

impl TryFrom<f64> for FixedPointDecimal {
    type Error = FixedPointError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        let scaled_value = value * FACTOR as f64;
        if scaled_value > u64::MAX as f64 || scaled_value < 0.0 {
            return Err(FixedPointError::Overflow);
        }

        let fixed_point_value = scaled_value.round() as u64;
        Ok(FixedPointDecimal {
            value: fixed_point_value,
        })
    }
}

impl std::ops::Add for FixedPointDecimal {
    type Output = Result<Self, FixedPointError>;

    fn add(self, other: FixedPointDecimal) -> Self::Output {
        if let Some(result) = self.value.checked_add(other.value) {
            Ok(FixedPointDecimal { value: result })
        } else {
            Err(FixedPointError::Overflow)
        }
    }
}

impl std::ops::AddAssign for FixedPointDecimal {
    fn add_assign(&mut self, other: FixedPointDecimal) {
        self.value = self
            .value
            .checked_add(other.value)
            .expect("Overflow during addition");
    }
}

impl std::ops::Sub for FixedPointDecimal {
    type Output = Result<Self, FixedPointError>;

    fn sub(self, other: FixedPointDecimal) -> Self::Output {
        if let Some(result) = self.value.checked_sub(other.value) {
            Ok(FixedPointDecimal { value: result })
        } else {
            Err(FixedPointError::Underflow)
        }
    }
}

impl std::ops::SubAssign for FixedPointDecimal {
    fn sub_assign(&mut self, other: FixedPointDecimal) {
        self.value = self
            .value
            .checked_sub(other.value)
            .expect("Underflow during subtraction");
    }
}

impl std::ops::Mul for FixedPointDecimal {
    type Output = Result<Self, FixedPointError>;

    fn mul(self, other: FixedPointDecimal) -> Self::Output {
        let result = (self.value as u128)
            .checked_mul(other.value as u128)
            .ok_or(FixedPointError::Overflow)?;

        let scaled_result = result
            .checked_div(FACTOR as u128)
            .ok_or(FixedPointError::Overflow)?;

        if scaled_result > u64::MAX as u128 {
            return Err(FixedPointError::Overflow);
        }

        Ok(FixedPointDecimal {
            value: scaled_result as u64,
        })
    }
}

impl std::ops::Div for FixedPointDecimal {
    type Output = Result<Self, FixedPointError>;

    fn div(self, other: FixedPointDecimal) -> Self::Output {
        if other.value == 0 {
            return Err(FixedPointError::DivisionByZero);
        }

        let scaled_numerator = (self.value as u128)
            .checked_mul(FACTOR as u128)
            .ok_or(FixedPointError::Overflow)?;

        let result = scaled_numerator
            .checked_div(other.value as u128)
            .ok_or(FixedPointError::Overflow)?;

        if result > u64::MAX as u128 {
            return Err(FixedPointError::Overflow);
        }

        Ok(FixedPointDecimal {
            value: result as u64,
        })
    }
}

impl PartialEq<u64> for FixedPointDecimal {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other * FACTOR
    }
}

impl PartialEq<FixedPointDecimal> for u64 {
    fn eq(&self, other: &FixedPointDecimal) -> bool {
        *self * FACTOR == other.value
    }
}

impl PartialOrd for FixedPointDecimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.cmp(&other.value))
    }

    fn lt(&self, other: &Self) -> bool {
        self.value < other.value
    }
}

impl fmt::Display for FixedPointDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let formatted = format!(
            "{:.1$}",
            self.value as f64 / FACTOR as f64,
            DECIMALS as usize
        );
        write!(f, "{}", formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u64_success() {
        let result = FixedPointDecimal::try_from(123u64).unwrap();
        assert_eq!(result.value, 123 * FACTOR);
    }

    #[test]
    fn test_try_from_f64_success() {
        let result = FixedPointDecimal::try_from(123.456789).unwrap();
        assert_eq!(result.value, 123456789);
    }

    #[test]
    fn test_try_from_f64_invalid_input() {
        let result = FixedPointDecimal::try_from(-123.456789);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::Overflow);
    }

    #[test]
    fn test_try_from_f64_overflow() {
        let large_value = (u64::MAX as f64 / FACTOR as f64) + 1.0;
        let result = FixedPointDecimal::try_from(large_value);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::Overflow);
    }

    #[test]
    fn test_addition_success() {
        let num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(23.456789).unwrap();
        let result = (num1 + num2).unwrap();
        assert_eq!(result.value, 35802467);
    }

    #[test]
    fn test_addition_overflow() {
        let num1 = FixedPointDecimal { value: u64::MAX };
        let num2 = FixedPointDecimal { value: 1 };
        let result = num1 + num2;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::Overflow);
    }

    #[test]
    fn test_add_assign_success() {
        let mut num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(23.456789).unwrap();
        num1 += num2;
        assert_eq!(num1.value, 35802467);
    }

    #[test]
    fn test_subtraction_success() {
        let num1 = FixedPointDecimal::try_from(23.456789).unwrap();
        let num2 = FixedPointDecimal::try_from(12.345678).unwrap();
        let result = (num1 - num2).unwrap();
        assert_eq!(result.value, 11111111);
    }

    #[test]
    fn test_subtraction_underflow() {
        let num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(23.456789).unwrap();
        let result = num1 - num2;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::Underflow);
    }

    #[test]
    fn test_sub_assign_success() {
        let mut num1 = FixedPointDecimal::try_from(23.456789).unwrap();
        let num2 = FixedPointDecimal::try_from(12.345678).unwrap();
        num1 -= num2;
        assert_eq!(num1.value, 11111111);
    }

    #[test]
    fn test_multiplication_success() {
        let num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(2.0).unwrap();
        let result = (num1 * num2).unwrap();
        assert_eq!(result.value, 24691356); // 12.345678 * 2.0 = 24.691356
    }

    #[test]
    fn test_multiplication_overflow() {
        let num1 = FixedPointDecimal {
            value: u64::MAX / 2,
        };
        let num2 = FixedPointDecimal::try_from(3.0).unwrap();
        let result = num1 * num2;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::Overflow);
    }

    #[test]
    fn test_division_success() {
        let num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(2.0).unwrap();
        let result = (num1 / num2).unwrap();
        assert_eq!(result.value, 6172839); // 12.345678 / 2.0 = 6.172839
    }

    #[test]
    fn test_division_by_zero() {
        let num1 = FixedPointDecimal::try_from(12.345678).unwrap();
        let num2 = FixedPointDecimal::try_from(0.0).unwrap();
        let result = num1 / num2;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), FixedPointError::DivisionByZero);
    }

    #[test]
    fn test_eq_with_u64() {
        let fixed_point = FixedPointDecimal::try_from(100u64).unwrap();
        assert_eq!(fixed_point, 100u64);
        assert_eq!(100u64, fixed_point);

        let fixed_point = FixedPointDecimal::try_from(999u64).unwrap();
        assert_ne!(fixed_point, 998u64);
        assert_ne!(998u64, fixed_point);
    }

    #[test]
    fn test_eq_with_fixed_point() {
        let fixed_point_a = FixedPointDecimal::try_from(123u64).unwrap();
        let fixed_point_b = FixedPointDecimal::try_from(123u64).unwrap();
        assert_eq!(fixed_point_a, fixed_point_b);

        let fixed_point_c = FixedPointDecimal::try_from(124u64).unwrap();
        assert_ne!(fixed_point_a, fixed_point_c);
    }

    #[test]
    fn test_less_than_operator() {
        let a = FixedPointDecimal::try_from(5.0).unwrap();
        let b = FixedPointDecimal::try_from(10.0).unwrap();
        assert!(a < b);
        assert!(!(b < a));
    }

    #[test]
    fn test_greater_than_operator() {
        let a = FixedPointDecimal::try_from(10.0).unwrap();
        let b = FixedPointDecimal::try_from(5.0).unwrap();
        assert!(a > b);
        assert!(!(b > a));
    }

    #[test]
    fn test_equal_operator() {
        let a = FixedPointDecimal::try_from(7.5).unwrap();
        let b = FixedPointDecimal::try_from(7.5).unwrap();
        assert!(a == b);
    }

    #[test]
    fn test_less_than_or_equal_operator() {
        let a = FixedPointDecimal::try_from(5.0).unwrap();
        let b = FixedPointDecimal::try_from(5.0).unwrap();
        assert!(a <= b);

        let c = FixedPointDecimal::try_from(4.0).unwrap();
        assert!(c <= b);
    }

    #[test]
    fn test_greater_than_or_equal_operator() {
        let a = FixedPointDecimal::try_from(10.0).unwrap();
        let b = FixedPointDecimal::try_from(5.0).unwrap();
        assert!(a >= b);

        let c = FixedPointDecimal::try_from(10.0).unwrap();
        assert!(a >= c);
    }

    #[test]
    fn test_display() {
        let value = FixedPointDecimal::try_from(123.456789).unwrap();
        assert_eq!(format!("{}", value), "123.456789");
    }

    #[test]
    fn test_default() {
        let default = FixedPointDecimal::default();
        assert_eq!(default.value, 0);
    }
}
