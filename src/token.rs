use std::ops::Range;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenMatch {
    Regular,
    MultiMatch
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Aleternation,
    NoneOrMany,
    OneOrMany,
    StartLine,
    EndLine,
    SingleMatch(char),
    MultiMatch(String),
    RangeMatch(Range<u8>),
    Complex(Vec<Token>)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenType,
    pub t_match: TokenMatch,
    pub min_match: i32
}

impl Token {
    pub fn new(token_type: TokenType, token_match: TokenMatch) -> Token {
        Token { kind: token_type, t_match: token_match, min_match: 1 }
    }

    pub fn new_regular(token_type: TokenType) -> Token {
        Token { kind: token_type, t_match: TokenMatch::Regular, min_match: 1 }
    }
}
