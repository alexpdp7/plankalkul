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

impl std::string::ToString for Number {
    fn to_string(&self) -> String {
        format!("{}", self.number)
    }
}
