#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub ast_parser);
pub mod ast;

fn main() {
    println!("Hello, world!");
}

#[test]
fn calculator4() {
    let expr = ast_parser::ExprParser::new().parse("22 * 44 + 66").unwrap();
    assert_eq!(&format!("{:?}", expr), "((22 * 44) + 66)");
}
