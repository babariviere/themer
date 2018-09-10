#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Ident(String),
    Str(String),
    Hex(String),
    Path(String),
    Number(String),
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
}
