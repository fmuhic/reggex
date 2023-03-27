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
            match c {
                ' ' | '\t' | '\n'=> None,
                '^' => Some(Token::StartToken(find_match_type(iter))),
                '$' => Some(Token::EndToken(find_match_type(iter))),
                'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => Some(Token::SingleMatch(c, find_match_type(iter))),
                '.' => Some(Token::RangeMatch('!' as u8 .. '~' as u8, find_match_type(iter))),
                '(' => Some(Token::Complex(parse_subgroup(iter), find_match_type(iter))),
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
    let match_type = match iter.peek() {
        Some(c) => {
            match c {
                '*' => MatchType::NoneOrMany,
                '+' => MatchType::OneOrMany,
                _ => MatchType::Regular
            }
        }
        None => MatchType::Regular
    };
    iter.reset_peek();
    match_type
}

fn parse_subgroup(iter: &mut MultiPeek<Chars>) -> Vec<Token> {
    let mut group: Vec<char> = Vec::new();
    let mut next = iter.peek();
    while next != None {
        let next_char = next.unwrap();
        if *next_char == ')' {
            break;
        }
        group.push(*next_char);
        println!("Next is  {:?}", next);
        next = iter.peek();
    }

    for _ in 0..group.len() + 1 {
        iter.next();
    }
        println!("Next one is {:?}", iter.peek());
    let expression: String = group.into_iter().collect();

    parse_expression(&expression)
}
