use itertools::multipeek;
use itertools::MultiPeek;
use std::str::Chars;

use crate::parser::parse_expression;
use crate::matchers::matcher::{ Matcher, MatchResult };
use crate::matchers::complex_matcher::ComplexMatcher;

pub struct Reggex {
    matcher: ComplexMatcher
}

impl Reggex {
    pub fn new(exp: &str) -> Reggex {
        let tokens = parse_expression(exp);
        println!("tokens {:?}", tokens);
        let matcher = ComplexMatcher::from_list(&tokens, None);
        Reggex { matcher }
    }

    pub fn matches(&self, exp: &str) -> bool {
        let iter = &mut multipeek(exp.chars());
        match self.matcher.match_exp(iter, 0) {
            MatchResult::Success(_) => match_endline(iter),
            _ => false
        }
    }

}

fn match_endline(iter: &mut MultiPeek<Chars>) -> bool {
    match iter.peek() {
        Some(_) => false,
        None => true
    }
}
