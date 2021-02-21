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
