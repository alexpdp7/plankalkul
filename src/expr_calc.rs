use crate::expr;
use crate::num;


impl expr::Expr {
    pub fn as_number(&self) -> num::Number {
        match self {
            expr::Expr::Number{number} => number.clone(),
            expr::Expr::Add{op1, op2} => op1.as_number() + op2.as_number(),
            expr::Expr::Sub{op1, op2} => op1.as_number() - op2.as_number(),
            expr::Expr::Mul{op1, op2} => op1.as_number() * op2.as_number(),
            expr::Expr::Div{op1, op2} => op1.as_number() / op2.as_number(),
        }
    }
}
