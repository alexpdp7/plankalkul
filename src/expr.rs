use crate::num::Number;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Number { number: Number },
    Add { op1: Box<Expr>, op2: Box<Expr> },
    Sub { op1: Box<Expr>, op2: Box<Expr> },
    Mul { op1: Box<Expr>, op2: Box<Expr> },
    Div { op1: Box<Expr>, op2: Box<Expr> },
}
