use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::{ Matcher, MatchResult };
use super::simple_matcher::SimpleMatcher;
use super::start_line_matcher::StartLineMatcher;
use super::end_line_matcher::EndLineMatcher;
use crate::token::Token;

pub struct ComplexMatcher {
    state: Option<Box<dyn Matcher>>,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for ComplexMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        match &self.state {
            Some(matcher) => match matcher.match_exp(iter, match_count) {
                MatchResult::Success(count) => match &self.next {
                    Some(next_matcher) => next_matcher.match_exp(iter, count),
                    None => MatchResult::Success(count)
                },
                MatchResult::Failed(count) => MatchResult::Failed(count)
            },
            None => MatchResult::Success(match_count)
        }
    }
}

impl ComplexMatcher {
    pub fn from_list(tokens: &Vec<Token>, next_matcher: Option<Box<dyn Matcher>>) -> ComplexMatcher {
        let mut next_link = None;
        for t in tokens.iter().rev() {
            let matcher: Box<dyn Matcher> = match t {
                Token::Complex(token_list, match_type) => {
                    Box::new(ComplexMatcher::from_list(token_list, next_link))
                }
                Token::StartToken(match_type) => {
                    Box::new(StartLineMatcher::new(next_link))
                }
                Token::EndToken(match_type) => {
                    Box::new(EndLineMatcher::new(next_link))
                }
                _ => Box::new(SimpleMatcher::from_token(t, next_link))
            };
            next_link = Some(matcher);
        }
        ComplexMatcher { state: next_link, next: next_matcher }
    }
}
