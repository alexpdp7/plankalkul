use nom::{
    branch::alt,
    character::complete::{char, one_of},
    multi::fold_many0,
    sequence::delimited,
    IResult,
};

use crate::expr::Expr;
use crate::num_parse::number;

pub fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, op1) = expr1(input)?;
    fold_many0(expr_second_arm, op1, |e1, (op, op2)| match op {
        '+' => Expr::Add {
            op1: Box::new(e1),
            op2: Box::new(op2),
        },
        '-' => Expr::Sub {
            op1: Box::new(e1),
            op2: Box::new(op2),
        },
        _ => panic!(),
    })(input)
}

fn expr_second_arm(input: &str) -> IResult<&str, (char, Expr)> {
    let (input, op) = one_of("+-")(input)?;
    let (input, op2) = expr1(input)?;
    Ok((input, (op, op2)))
}

fn expr1(input: &str) -> IResult<&str, Expr> {
    let (input, op1) = expr2(input)?;
    fold_many0(expr1_second_arm, op1, |e1, (op, op2)| match op {
        '*' => Expr::Mul {
            op1: Box::new(e1),
            op2: Box::new(op2),
        },
        '/' => Expr::Div {
            op1: Box::new(e1),
            op2: Box::new(op2),
        },
        _ => panic!(),
    })(input)
}

fn expr1_second_arm(input: &str) -> IResult<&str, (char, Expr)> {
    let (input, op) = one_of("*/")(input)?;
    let (input, op2) = expr1(input)?;
    Ok((input, (op, op2)))
}

fn expr2(input: &str) -> IResult<&str, Expr> {
    alt((expr2_number, expr2_paren))(input)
}

fn expr2_number(input: &str) -> IResult<&str, Expr> {
    let (input, number) = number(input)?;
    Ok((input, Expr::Number { number }))
}

fn expr2_paren(input: &str) -> IResult<&str, Expr> {
    delimited(char('('), expr, char(')'))(input)
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
