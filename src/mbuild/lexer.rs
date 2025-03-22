use std::fmt;

pub enum TokenType {
    KwSet,
    KwTo,
}

pub struct Token {
    token_type: TokenType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Lexer {
        Lexer { src: s, pos: 0 }
    }

    pub fn next_token(&self) -> Option<Token> {
        todo!()
    }
}
