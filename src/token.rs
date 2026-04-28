#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    LParen,
    RParen,
    Multiply,
    Divide,
    Plus,
    Minus,
    Negate,
    EOF,
}
