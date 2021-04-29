#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}
