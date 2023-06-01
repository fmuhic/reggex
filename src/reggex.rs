use itertools::multipeek;

use crate::parser::parse_expression;
use crate::matchers::matcher::{ Matcher, MatchResult };
use crate::matchers::complex_matcher::ComplexMatcher;
use crate::token::TokenMatch;

pub struct Reggex {
    matcher: ComplexMatcher
}

impl Reggex {
    pub fn new(exp: &str) -> Reggex {
        let tokens = parse_expression(exp);
        // println!("tokens: {:#?}", tokens);
        let matcher = ComplexMatcher::from_list(&tokens, TokenMatch::Regular, 1, None);
        Reggex { matcher }
    }

    pub fn matches(&self, exp: &str) -> bool {
        let iter = &mut multipeek(exp.chars());
        match self.matcher.match_exp(iter, 0) {
            MatchResult::Success(match_len) => {
                match_len == exp.len() as i32
            }
            _ => false
        }
    }
}
