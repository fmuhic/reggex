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
        Some(c) => match c {
            ' ' | '\t' | '\n'=> None,
            '^' => Some(Token::StartLine(find_match_type(iter))),
            '$' => Some(Token::EndLine(find_match_type(iter))),
            'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => Some(Token::SingleMatch(c, find_match_type(iter))),
            '.' => Some(Token::RangeMatch('!' as u8 .. '~' as u8, find_match_type(iter))),
            '|' => Some(Token::Aleternation),
            '(' => Some(Token::Complex(parse_subgroup(iter), find_match_type(iter))),
            _ => None
        }
        None => None
    }
}

fn advance(iter: &mut MultiPeek<Chars>, amount: usize) -> Option<char> {
    let mut next_char = None;
    for _ in 0..amount {
        next_char = iter.next();
    }
    next_char
}

fn find_match_type(iter: &mut MultiPeek<Chars>) -> MatchType {
    let match_type = match iter.peek() {
        Some(c) => match c {
            '*' => MatchType::NoneOrMany,
            '+' => MatchType::OneOrMany,
            _ => MatchType::Regular
        }
        None => MatchType::Regular
    };
    iter.reset_peek();
    match_type
}

fn parse_subgroup(iter: &mut MultiPeek<Chars>) -> Vec<Token> {
    let mut subgroup = Vec::new();
    let mut parentheses_count = 0;

    let mut next = iter.peek().unwrap();
    while !(*next == ')' && parentheses_count == 0) {
        if *next == '(' {
            parentheses_count += 1;
        } else if *next == ')' {
            parentheses_count -= 1;
        }
        subgroup.push(*next);
        next = iter.peek().unwrap();
    }

    advance(iter, subgroup.len() + 1);

    let exp: String = subgroup.into_iter().collect();
    parse_expression(&exp)
}
