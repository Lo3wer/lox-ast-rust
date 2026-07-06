use crate::token::Token;

#[derive(Debug, Clone)]
pub struct LexError {
    pub line: usize,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}