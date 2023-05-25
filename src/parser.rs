use crate::token::{
    Token,
    TokenType,
    TokenMatch
};

use itertools::MultiPeek;
use std::str::Chars;
use itertools::multipeek;

pub fn parse_expression(exp: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut iter = multipeek(exp.chars());
    while iter.peek() != None {
        parse_token(&mut iter).map(|t| {
            match tokens.last_mut() {
                Some(last) => {
                    match t.kind {
                        TokenType::NoneOrMany => {
                            last.t_match = TokenMatch::MultiMatch;
                            last.min_match -= 1;
                        },
                        TokenType::OneOrMany => {
                            last.t_match = TokenMatch::MultiMatch;
                        },
                        _ => {
                            if last.kind == t.kind {
                                last.min_match += 1;
                            } else {
                                tokens.push(t);
                            }
                        }
                    }

                }
                None => tokens.push(t)
            }
        });
    }

    for t in &tokens {
        println!("Token {:?}", t)
    }
    tokens
}

fn parse_token(iter: &mut MultiPeek<Chars>) -> Option<Token> {
    match advance(iter, 1) {
        Some(c) => match c {
            ' ' | '\t' | '\n'=> None,
            '^' => Some(Token::new_regular(TokenType::StartLine)),
            '$' => Some(Token::new_regular(TokenType::EndLine)),
            '*' => Some(Token::new_regular(TokenType::NoneOrMany)),
            '+' => Some(Token::new_regular(TokenType::OneOrMany)),
            'a' ..= 'z' | 'A' ..= 'Z' | '0' ..= '9' => Some(Token::new_regular(TokenType::SingleMatch(c))),
            '.' => Some(Token::new_regular(TokenType::RangeMatch('!' as u8 .. '~' as u8))),
            '|' => Some(Token::new_regular(TokenType::Aleternation)),
            '(' => Some(Token::new_regular(TokenType::Complex(parse_subgroup(iter)))),
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
