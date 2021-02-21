use std::str::FromStr;

use num::rational::BigRational;

#[derive(Clone, Debug, PartialEq)]
pub struct Number {
    number: BigRational,
}

pub fn build_number(integer: &str, decimal: &str) -> Number {
    Number {
        number: num::rational::Ratio::from_str(&format!(
            "{}{}/1{}",
            integer,
            decimal,
            std::iter::repeat("0")
                .take(decimal.len())
                .collect::<String>()
        ))
        .unwrap(),
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.number)
    }
}

impl std::ops::Add for Number {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            number: self.number + other.number,
        }
    }
}

impl std::ops::Sub for Number {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            number: self.number - other.number,
        }
    }
}

impl std::ops::Mul for Number {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            number: self.number * other.number,
        }
    }
}

impl std::ops::Div for Number {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            number: self.number / other.number,
        }
    }
}

impl std::ops::Neg for Number {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            number: -self.number,
        }
    }
}
