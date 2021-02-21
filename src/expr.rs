use crate::num::Number;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number { number: Number },
    Add { op1: Box<Expr>, op2: Box<Expr> },
    Sub { op1: Box<Expr>, op2: Box<Expr> },
    Mul { op1: Box<Expr>, op2: Box<Expr> },
    Div { op1: Box<Expr>, op2: Box<Expr> },
}

impl std::string::ToString for Expr {
    fn to_string(&self) -> String {
        match self {
            Expr::Number { number } => format!("{}", number.to_string()),
            Expr::Add { op1, op2 } => format!("{}+{}", op1.to_string(), op2.to_string()),
            Expr::Sub { op1, op2 } => format!("{}-{}", op1.to_string(), op2.to_string()),
            Expr::Mul { op1, op2 } => format!(
                "{}*{}",
                match **op1 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({})", op1.to_string()),
                    _ => op1.to_string(),
                },
                match **op2 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({})", op2.to_string()),
                    _ => op2.to_string(),
                },
            ),
            Expr::Div { op1, op2 } => format!(
                "{}/{}",
                match **op1 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({})", op1.to_string()),
                    _ => op1.to_string(),
                },
                match **op2 {
                    Expr::Add { .. } | Expr::Sub { .. } => format!("({})", op2.to_string()),
                    _ => op2.to_string(),
                },
            ),
        }
    }
}
