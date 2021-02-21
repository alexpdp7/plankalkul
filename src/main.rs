use plankalkul::expr_parse;
use std::env::args;

fn main() {
    println!("{:?}", expr_parse::expr(args().nth(1).unwrap().as_str()));
}
