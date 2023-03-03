use crate::token::Token;
use crate::token::MatchType;

use itertools::MultiPeek;
use std::str::Chars;
use itertools::multipeek;

pub fn parse_expression(exp: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = multipeek(exp.chars());
    while iter.peek() != None {
        parse_token(&mut iter).map(|token| tokens.push(token));
    }

    return tokens;
}

fn parse_token(iter: &mut MultiPeek<Chars>) -> Option<Token> {
    match advance(iter, 1) {
        Some(c) => {
            let match_type = find_match_type(iter);
            match c {
                ' ' | '\t' | '\n'=> None,
                'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' | '^' | '$' => Some(Token::SingleMatch(c, match_type)),
                '.' => Some(Token::RangeMatch('!' as u8 .. '~' as u8, match_type)),
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

fn find_match_type(iter: &mut MultiPeek<Chars>) -> MatchType {
    match iter.peek() {
        Some(c) => {
            match c {
                '*' => MatchType::NoneOrMany,
                '+' => MatchType::OneOrMany,
                _ => MatchType::Regular
            }
        }
        None => MatchType::Regular
    }
}
