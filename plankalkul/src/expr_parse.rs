use nom::{
    branch::alt,
    character::complete::{char, one_of},
    multi::fold_many0,
    sequence::delimited,
    IResult,
};

use crate::expr::Expr;
use crate::num_parse::number;

pub fn expr(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, op1) = expr1(input)?;
    let mut folder = fold_many0(
        expr_second_arm,
        || op1.clone(),
        |e1, (op, op2)| match op {
            '+' => Box::new(Expr::Add { op1: e1, op2 }),
            '-' => Box::new(Expr::Sub { op1: e1, op2 }),
            _ => panic!(),
        },
    );
    folder(input)
}

fn expr_second_arm(input: &str) -> IResult<&str, (char, Box<Expr>)> {
    let (input, op) = one_of("+-")(input)?;
    let (input, op2) = expr1(input)?;
    Ok((input, (op, op2)))
}

fn expr1(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, op1) = expr2(input)?;
    let mut folder = fold_many0(
        expr1_second_arm,
        || op1.clone(),
        |e1, (op, op2)| match op {
            '*' => Box::new(Expr::Mul { op1: e1, op2 }),
            '/' => Box::new(Expr::Div { op1: e1, op2 }),
            _ => panic!(),
        },
    );

    folder(input)
}

fn expr1_second_arm(input: &str) -> IResult<&str, (char, Box<Expr>)> {
    let (input, op) = one_of("*/")(input)?;
    let (input, op2) = expr2(input)?;
    Ok((input, (op, op2)))
}

fn expr2(input: &str) -> IResult<&str, Box<Expr>> {
    alt((expr2_number, expr2_paren, expr2_negate))(input)
}

fn expr2_number(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, number) = number(input)?;
    Ok((input, Box::new(Expr::Number { number })))
}

fn expr2_paren(input: &str) -> IResult<&str, Box<Expr>> {
    delimited(char('('), expr, char(')'))(input)
}

fn expr2_negate(input: &str) -> IResult<&str, Box<Expr>> {
    let (input, _) = char('-')(input)?;
    let (input, expr) = expr(input)?;
    Ok((input, Box::new(Expr::Negate { op: expr })))
}

#[test]
fn test() {
    test_expr("1");
    test_expr_is("1", "1.0");
    test_expr_is("11/10", "1.1");
    test_expr("-1");
    test_expr_is("-1", "-1.0");
    test_expr_is("-11/10", "-1.1");
    test_expr("1+1");
    test_expr("1-1");
    test_expr("1*1");
    test_expr("1/1");
    test_expr("1+1+1");
    test_expr("1*1*1");
    test_expr("1+1*1");
    test_expr("(1+1)*1");
    test_expr_is("1+1*1", "1+(1*1)");
    test_expr("1-2+3");
    test_expr("1-(2+3)");
    test_expr_is("1-2+3", "(1-2)+3");
    test_expr("-(1+1)");
    test_expr("10/5/2");
    test_expr("10/(5/2)");
    test_expr_is("10/5/2", "(10/5)/2");
}

#[cfg(test)]
fn test_expr(s: &str) {
    let (rest, expr) = expr(s).unwrap();
    assert_eq!("", rest);
    assert_eq!(s, expr.to_string());
}

#[cfg(test)]
fn test_expr_is(exp: &str, s: &str) {
    let (rest, expr) = expr(s).unwrap();
    assert_eq!("", rest);
    assert_eq!(exp, expr.to_string());
}
