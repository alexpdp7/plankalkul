use plankalkul::expr_parse;
use std::env::args;

fn main() {
    let arg = args().nth(1).unwrap();
    let (rest, expr) = expr_parse::expr(arg.as_str()).unwrap();
    assert_eq!("", rest);
    println!(
        "{} -> {} , {} , {:?}",
        arg,
        expr,
        expr.as_number(),
        expr.as_number().to_decimal()
    );
}
