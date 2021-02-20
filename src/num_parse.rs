extern crate nom;
use nom::character::complete::{char, digit1};
use nom::{char, opt, IResult};

use crate::num::{build_number, Number};

pub fn number(input: &str) -> IResult<&str, Number> {
    let (input, sign) = opt!(input, char!('-'))?;
    let (input, integer) = digit1(input)?;
    let (input, decimal) = opt!(input, decimal_part)?;
    let integer = match sign.is_some() {
        true => format!("-{}", integer),
        false => integer.to_string(),
    };
    match decimal {
        Some(decimal) => Ok((input, build_number(integer.as_str(), decimal))),
        None => Ok((input, build_number(integer.as_str(), "0"))),
    }
}

fn decimal_part(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('.')(input)?;
    let (input, decimals) = digit1(input)?;
    Ok((input, decimals))
}

#[test]
fn test_decimal_part() {
    assert_eq!(("", "123"), decimal_part(".123").unwrap());
}

#[test]
fn test_integer_number() {
    assert_eq!(("", build_number("123", "0")), number("123").unwrap(),);
}

#[test]
fn test_decimal_number() {
    assert_eq!(("", build_number("123", "456")), number("123.456").unwrap());
}

#[test]
fn test_negative_decimal_number() {
    assert_eq!(
        ("", build_number("-123", "456")),
        number("-123.456").unwrap()
    );
}
