#[macro_use]
extern crate lalrpop_util;

pub mod ast;
#[allow(unused_imports)]
use ast::*;

// cargo run build scriptsで.lalrpopファイルを読み込みパーサーを生成する
lalrpop_mod!(pub ast_parser);

fn main() {
    println!("Hello, world!");
}

#[test]
fn calculator4() {
    let expr = ast_parser::ExprParser::new().parse("22 * 44 + 66").unwrap();
    assert_eq!(
        Box::new(Expr::Op(
            Box::new(Expr::Op(
                Box::new(Expr::Number(22)),
                Opcode::Mul,
                Box::new(Expr::Number(44)),
            )),
            Opcode::Add,
            Box::new(Expr::Number(66)),
        )),
        expr
    );
}
