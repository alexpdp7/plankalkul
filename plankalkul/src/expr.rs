use crate::num::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number { number: Number },
    Negate { op: Box<Expr> },
    Add { op1: Box<Expr>, op2: Box<Expr> },
    Sub { op1: Box<Expr>, op2: Box<Expr> },
    Mul { op1: Box<Expr>, op2: Box<Expr> },
    Div { op1: Box<Expr>, op2: Box<Expr> },
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number { number } => write!(f, "{number}"),
            Expr::Negate { op } => write!(f, "-({op})"),
            Expr::Add { op1, op2 } => write!(f, "{op1}+{op2}"),
            Expr::Sub { op1, op2 } => write!(
                f,
                "{}-{}",
                op1,
                match **op2 {
                    Expr::Add { .. } => format!("({op2})"),
                    _ => op2.to_string(),
                }
            ),
            Expr::Mul { op1, op2 } => write!(
                f,
                "{}*{}",
                match **op1 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({op1})"),
                    _ => op1.to_string(),
                },
                match **op2 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({op2})"),
                    _ => op2.to_string(),
                },
            ),
            Expr::Div { op1, op2 } => write!(
                f,
                "{}/{}",
                match **op1 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({op1})"),
                    _ => op1.to_string(),
                },
                match **op2 {
                    Expr::Number { .. } => op2.to_string(),
                    _ => format!("({op2})"),
                },
            ),
        }
    }
}
