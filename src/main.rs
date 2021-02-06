extern crate nom;
use nom::character::complete::{char, digit1};
use nom::{opt, IResult};
use num::rational::BigRational;
use std::str::FromStr;

fn decimal_part(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('.')(input)?;
    let (input, decimals) = digit1(input)?;
    Ok((input, decimals))
}

fn number(input: &str) -> IResult<&str, BigRational> {
    let (input, integer) = digit1(input)?;
    let (input, decimal) = opt!(input, decimal_part)?;
    match decimal {
        Some(decimal) => Ok((
            input,
            num::rational::Ratio::from_str(&format!(
                "{}{}/1{}",
                integer,
                decimal,
                std::iter::repeat("0")
                    .take(decimal.len())
                    .collect::<String>()
            ))
            .unwrap(),
        )),
        None => Ok((input, num::rational::Ratio::from_str(integer).unwrap())),
    }
}

fn main() {
}

#[test]
fn test_decimal_part() {
    assert_eq!(("", "123"), decimal_part(".123").unwrap());
}

#[test]
fn test_integer_number() {
    assert_eq!(
        ("", num::rational::Ratio::from_str("123").unwrap()),
        number("123").unwrap()
    );
}

#[test]
fn test_decimal_number() {
    assert_eq!(
        ("", num::rational::Ratio::from_str("123456/1000").unwrap()),
        number("123.456").unwrap()
    );
}
