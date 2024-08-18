use std::cmp::Ordering;
use std::convert::TryFrom;
use std::fmt;

const DECIMALS: u32 = 6;
const FACTOR: u64 = 10_u64.pow(DECIMALS);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixedPointDecimal {
    value: u64,
}

impl From<u64> for FixedPointDecimal {
    fn from(value: u64) -> Self {
        FixedPointDecimal {
            value: value * FACTOR,
        }
    }
}

impl TryFrom<f64> for FixedPointDecimal {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value <= 0.0 {
            Err("Value must be greater than zero")
        } else {
            let scaled_value = (value * FACTOR as f64).round() as u64;
            Ok(FixedPointDecimal {
                value: scaled_value,
            })
        }
    }
}

impl std::ops::Add for FixedPointDecimal {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        FixedPointDecimal {
            value: self.value.wrapping_add(other.value),
        }
    }
}

impl std::ops::AddAssign for FixedPointDecimal {
    fn add_assign(&mut self, other: Self) {
        self.value = self.value.wrapping_add(other.value);
    }
}

impl std::ops::Sub for FixedPointDecimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        FixedPointDecimal {
            value: self.value.saturating_sub(other.value),
        }
    }
}

impl std::ops::SubAssign for FixedPointDecimal {
    fn sub_assign(&mut self, other: Self) {
        self.value = self.value.saturating_sub(other.value);
    }
}

impl std::ops::Mul for FixedPointDecimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let result = (self.value as u128 * other.value as u128) / FACTOR as u128;
        FixedPointDecimal {
            value: result as u64,
        }
    }
}

impl std::ops::Div for FixedPointDecimal {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let result = (self.value as u128 * FACTOR as u128) / other.value as u128;
        FixedPointDecimal {
            value: result as u64,
        }
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

impl Default for FixedPointDecimal {
    fn default() -> Self {
        FixedPointDecimal { value: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_u64() {
        let value = 12345;
        let fixed_point = FixedPointDecimal::from(value);
        assert_eq!(fixed_point.value, value * FACTOR);
    }

    #[test]
    fn test_try_from_f64_success() {
        let value = 123.456789;
        let fixed_point = FixedPointDecimal::try_from(value).expect("Conversion failed");
        assert_eq!(fixed_point.value, (value * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_try_from_f64_failure() {
        let value = -123.456789;
        let result = FixedPointDecimal::try_from(value);
        assert_eq!(result, Err("Value must be greater than zero"));
    }

    #[test]
    fn test_addition() {
        let a = FixedPointDecimal::try_from(12.345678).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(7.654321).expect("Conversion failed");
        let result = a + b;
        assert_eq!(result.value, (19.999999 * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_add_assign() {
        let mut a = FixedPointDecimal::try_from(12.345678).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(7.654321).expect("Conversion failed");
        a += b;
        assert_eq!(a.value, (19.999999 * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_subtraction_clamped_to_zero() {
        let a = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        let result = a - b;
        assert_eq!(result.value, 0);
    }

    #[test]
    fn test_subtraction_successful_clamp() {
        let a = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        let result = a - b;
        assert_eq!(result.value, (5.0 * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_multiplication() {
        let a = FixedPointDecimal::try_from(1.234567).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(2.345678).expect("Conversion failed");
        let result = a * b;
        assert_eq!(result.value, (2.895896 * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_division() {
        let a = FixedPointDecimal::try_from(2.345678).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(1.234567).expect("Conversion failed");
        let result = a / b;
        assert_eq!(result.value, (1.900000015 * FACTOR as f64).round() as u64);
    }

    #[test]
    fn test_eq_with_u64() {
        let fixed_point = FixedPointDecimal::from(100u64);
        assert_eq!(fixed_point, 100u64);
        assert_eq!(100u64, fixed_point);

        let fixed_point = FixedPointDecimal::from(123456u64);
        assert_eq!(fixed_point, 123456u64);
        assert_eq!(123456u64, fixed_point);

        let fixed_point = FixedPointDecimal::from(999u64);
        assert_ne!(fixed_point, 998u64);
        assert_ne!(998u64, fixed_point);
    }

    #[test]
    fn test_eq_with_fixed_point() {
        let fixed_point_a = FixedPointDecimal::from(123u64);
        let fixed_point_b = FixedPointDecimal::from(123u64);
        assert_eq!(fixed_point_a, fixed_point_b);

        let fixed_point_c = FixedPointDecimal::from(124u64);
        assert_ne!(fixed_point_a, fixed_point_c);
    }

    #[test]
    fn test_less_than_operator() {
        let a = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        assert!(a < b);
        assert!(!(b < a));
    }

    #[test]
    fn test_greater_than_operator() {
        let a = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        assert!(a > b);
        assert!(!(b > a));
    }

    #[test]
    fn test_equal_operator() {
        let a = FixedPointDecimal::try_from(7.5).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(7.5).expect("Conversion failed");
        assert!(a == b);
    }

    #[test]
    fn test_less_than_or_equal_operator() {
        let a = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        assert!(a <= b);

        let c = FixedPointDecimal::try_from(4.0).expect("Conversion failed");
        assert!(c <= b);
    }

    #[test]
    fn test_greater_than_or_equal_operator() {
        let a = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        let b = FixedPointDecimal::try_from(5.0).expect("Conversion failed");
        assert!(a >= b);

        let c = FixedPointDecimal::try_from(10.0).expect("Conversion failed");
        assert!(a >= c);
    }

    #[test]
    fn test_display() {
        let value = FixedPointDecimal::try_from(123.456789).expect("Conversion failed");
        assert_eq!(format!("{}", value), "123.456789");
    }

    #[test]
    fn test_default() {
        let default = FixedPointDecimal::default();
        assert_eq!(default.value, 0);
    }
}
