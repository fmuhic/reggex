use crate::token::Token;

use itertools::MultiPeek;
use std::str::Chars;
use itertools::multipeek;

pub fn parse_expression(exp: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = multipeek(exp.chars());
    while iter.peek() != None {
        match parse_token(&mut iter) {
            Some(token) => {
                tokens.push(token);
            },
            None => {
                println!("Unknown token");
            }
        }
    }

    return tokens;
}

fn parse_token(iter: &mut MultiPeek<Chars>) -> Option<Token> {
    match advance(iter, 1) {
        Some(c) => {
            match c {
                ' ' | '\t' | '\n'=> None,
                'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '^' | '$' => Some(Token::SingleMatch(c)),
                '.' => Some(Token::RangeMatch('!' .. '~')),
                _ => None
            }
        }
        None => None
    }
}

fn advance(iter: &mut MultiPeek<Chars>, amount: u32) -> Option<char> {
    let mut next_char = None;
    for _ in 0..amount {
        next_char = iter.next();
    }
    next_char
}
