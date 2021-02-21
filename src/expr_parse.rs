use nom::{
    branch::alt,
    character::complete::{char, one_of},
    combinator::opt,
    sequence::delimited,
    IResult,
};

use crate::expr::Expr;
use crate::num_parse::number;

pub fn expr(input: &str) -> IResult<&str, Expr> {
    let (input, op1) = expr1(input)?;
    let (input, second_arm) = opt(expr_second_arm)(input)?;
    Ok((
        input,
        match second_arm {
            Some((op, op2)) => match op {
                '+' => Expr::Add {
                    op1: Box::new(op1),
                    op2: Box::new(op2),
                },
                '-' => Expr::Sub {
                    op1: Box::new(op1),
                    op2: Box::new(op2),
                },
                _ => panic!(),
            },
            None => op1,
        },
    ))
}

fn expr_second_arm(input: &str) -> IResult<&str, (char, Expr)> {
    let (input, op) = one_of("+-")(input)?;
    let (input, op2) = expr1(input)?;
    Ok((input, (op, op2)))
}

fn expr1(input: &str) -> IResult<&str, Expr> {
    let (input, op1) = expr2(input)?;
    let (input, second_arm) = opt(expr1_second_arm)(input)?;
    Ok((
        input,
        match second_arm {
            Some((op, op2)) => match op {
                '*' => Expr::Mul {
                    op1: Box::new(op1),
                    op2: Box::new(op2),
                },
                '/' => Expr::Div {
                    op1: Box::new(op1),
                    op2: Box::new(op2),
                },
                _ => panic!(),
            },
            None => op1,
        },
    ))
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
