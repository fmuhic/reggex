use itertools::{MultiPeek, Itertools};
use std::str::Chars;

use super::matcher::{ Matcher, MatchResult };
use super::simple_matcher::SimpleMatcher;
use super::start_line_matcher::StartLineMatcher;
use super::end_line_matcher::EndLineMatcher;
use crate::token::{Token, TokenType};

pub struct ComplexMatcher {
    states: Vec<Box<dyn Matcher>>,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for ComplexMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        let mut result = MatchResult::Success(match_count);

        for state in self.states.iter().rev() {
            let mut current_iter = iter.clone();
            result = match state.match_exp(&mut current_iter, match_count) {
                MatchResult::Success(count) => match &self.next {
                    Some(next_matcher) => next_matcher.match_exp(&mut current_iter, count),
                    None => MatchResult::Success(count)
                }
                MatchResult::Failed(count) => MatchResult::Failed(count)
            };

            if let MatchResult::Success(_) = result {
                return result
            }
        }

        result
    }
}

impl ComplexMatcher {
    pub fn from_list(tokens: &Vec<Token>, next_matcher: Option<Box<dyn Matcher>>) -> ComplexMatcher {
        let mut states = Vec::new();
        let mut next_link = None;

        for t in tokens.iter().rev() {
            if let TokenType::Aleternation = t.kind {
                if let Some(matcher) = next_link {
                    states.push(matcher);
                    next_link = None;
                }
                continue
            }

            let matcher: Box<dyn Matcher> = match &t.kind {
                TokenType::Complex(token_list) => {
                    Box::new(ComplexMatcher::from_list(token_list, next_link))
                }
                TokenType::StartLine => {
                    Box::new(StartLineMatcher::new(next_link))
                }
                TokenType::EndLine => {
                    Box::new(EndLineMatcher::new(next_link))
                }
                _ => Box::new(SimpleMatcher::from_token(t, next_link))
            };
            next_link = Some(matcher);
        }

        if let Some(matcher) = next_link {
            states.push(matcher)
        }

        ComplexMatcher { states, next: next_matcher }
    }
}
