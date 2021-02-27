use std::collections::HashMap;

use num::bigint::BigInt;
use num::rational::BigRational;
use num::Integer;
use num::Signed;

pub struct Decimal {
    integer: String,
    decimal: String,
    decimal_periodic: String,
}

pub fn to_decimal(n: BigRational) -> Decimal {
    let mut f = n.fract().abs();
    let mut decimal = "".to_string();
    let mut seen = HashMap::new();
    loop {
        f = f * BigInt::from(10);
        let (div, rem) = f.numer().div_rem(f.denom());
        decimal = format!("{}{}", decimal, div);
        if rem == BigInt::from(0) {
            return Decimal {
                integer: n.to_integer().to_string(),
                decimal: decimal,
                decimal_periodic: "".to_string(),
            };
        }

        if seen.contains_key(&rem) {
            let pos = *seen.get(&rem).unwrap();
            let (d, p) = decimal.split_at(pos);
            return Decimal {
                integer: n.to_integer().to_string(),
                decimal: d.to_string(),
                decimal_periodic: p.to_string(),
            };
        }

        seen.insert(rem.clone(), decimal.len());

        f = BigRational::new(rem, f.denom().clone());
    }
}

impl std::fmt::Debug for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}|{}|",
            self.integer, self.decimal, self.decimal_periodic
        )
    }
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn test_to_decimal() {
    fn dec(s: &str) -> String {
        format!("{:?}", to_decimal(BigRational::from_str(s).unwrap()))
    }
    assert_eq!("0.45||", dec("45/100"));
    assert_eq!("23.0||", dec("23"));
    assert_eq!("-23.0||", dec("-23"));
    assert_eq!("23.45||", dec("2345/100"));
    assert_eq!("-23.45||", dec("-2345/100"));
    assert_eq!("0.23|45|", dec("2322/9900"));
}