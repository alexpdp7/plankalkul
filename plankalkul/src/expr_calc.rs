use crate::expr;
use crate::num;

impl expr::Expr {
    pub fn as_number(&self) -> Result<num::Number, String> {
        match self {
            expr::Expr::Number { number } => Ok(number.clone()),
            expr::Expr::Negate { op } => match op.as_number() {
                Ok(op) => Ok(-op),
                Err(s) => Err(format!("Cannot negate {op}: {s}")),
            },
            expr::Expr::Add { op1, op2 } => match (op1.as_number(), op2.as_number()) {
                (Ok(op1), Ok(op2)) => Ok(op1 + op2),
                _ => Err(show_error_binary_expr(self, op1, op2)),
            },
            expr::Expr::Sub { op1, op2 } => match (op1.as_number(), op2.as_number()) {
                (Ok(op1), Ok(op2)) => Ok(op1 - op2),
                _ => Err(show_error_binary_expr(self, op1, op2)),
            },
            expr::Expr::Mul { op1, op2 } => match (op1.as_number(), op2.as_number()) {
                (Ok(op1), Ok(op2)) => Ok(op1 * op2),
                _ => Err(show_error_binary_expr(self, op1, op2)),
            },
            expr::Expr::Div { op1, op2 } => match (op1.as_number(), op2.as_number()) {
                (Ok(op1), Ok(op2)) => {
                    if op2 == num::build_number("0", "0") {
                        Err(format!("division by zero in {self}"))
                    } else {
                        Ok(op1 / op2)
                    }
                }
                _ => Err(show_error_binary_expr(self, op1, op2)),
            },
        }
    }
}

fn show_error_binary_expr(expr: &expr::Expr, op1: &expr::Expr, op2: &expr::Expr) -> String {
    format!(
        "error in {}: {}, {}",
        expr,
        show_error_expr(op1),
        show_error_expr(op2)
    )
}

fn show_error_expr(op: &expr::Expr) -> String {
    match op.as_number() {
        Ok(op) => format!("{op}"),
        Err(err) => format!("({err})"),
    }
}

#[cfg(test)]
use crate::expr_parse;

#[test]
fn test_err_div_by_zero() {
    assert_eq!(
        expr_parse::expr("1/0").unwrap().1.as_number().err(),
        Some("division by zero in 1/0".to_string())
    );
}

#[test]
fn test_err_operate_with_div_by_zero() {
    assert_eq!(
        expr_parse::expr("1+1/0").unwrap().1.as_number().err(),
        Some("error in 1+1/0: 1, (division by zero in 1/0)".to_string())
    );
}
