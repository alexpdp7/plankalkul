use std::collections::HashMap;

use num::bigint::BigInt;
use num::rational::BigRational;
use num::Integer;
use num::Signed;

pub struct Decimal {
    integer: String,
    decimal: String,
    decimal_periodic: String,
    complete: bool,
}

pub fn to_decimal(n: BigRational, max_decimals: usize) -> Decimal {
    let (decimal, decimal_periodic, complete) =
        fract_to_decimal_and_periodic(n.fract().abs(), max_decimals);
    Decimal {
        integer: n.to_integer().to_string(),
        decimal,
        decimal_periodic,
        complete,
    }
}

fn fract_to_decimal_and_periodic(
    mut f: BigRational,
    max_decimals: usize,
) -> (String, String, bool) {
    let mut decimal = "".to_string();
    let mut seen = HashMap::new();
    loop {
        if decimal.len() == max_decimals {
            return (decimal, "".to_string(), false);
        }
        f *= BigInt::from(10);

        if seen.contains_key(&f) {
            let pos = *seen.get(&f).unwrap();
            let (d, p) = decimal.split_at(pos - 1);
            return (d.to_string(), p.to_string(), true);
        }

        let (div, rem) = f.numer().div_rem(f.denom());
        decimal = format!("{decimal}{div}");
        if rem == BigInt::from(0) {
            return (decimal, "".to_string(), true);
        }

        seen.insert(f.clone(), decimal.len());

        f = BigRational::new(rem, f.denom().clone());
    }
}

impl std::fmt::Debug for Decimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{}{}{}{}{}",
            self.integer,
            self.decimal,
            if !self.decimal_periodic.is_empty() {
                "|"
            } else {
                ""
            },
            self.decimal_periodic,
            if !self.decimal_periodic.is_empty() {
                "|"
            } else {
                ""
            },
            if self.complete { "" } else { "..." },
        )
    }
}

#[cfg(test)]
use std::str::FromStr;

#[test]
fn test_to_decimal() {
    fn dec(s: &str) -> String {
        format!("{:?}", to_decimal(BigRational::from_str(s).unwrap(), 100))
    }
    assert_eq!("0.45", dec("45/100"));
    assert_eq!("23.0", dec("23"));
    assert_eq!("-23.0", dec("-23"));
    assert_eq!("23.45", dec("2345/100"));
    assert_eq!("-23.45", dec("-2345/100"));
    assert_eq!("0.23|45|", dec("2322/9900"));
    assert_eq!("0.125", dec("1/8"));
    assert_eq!("0.|3|", dec("1/3"));
    assert_eq!("0.0000000004690085841454564425004666060876731713418737579658979890655216750096407176766165335402294038...", dec("1/2132157137"))
}
