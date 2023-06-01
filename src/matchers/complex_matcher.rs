use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::{ Matcher, MatchResult };
use super::simple_matcher::SimpleMatcher;
use super::start_line_matcher::StartLineMatcher;
use super::end_line_matcher::EndLineMatcher;
use crate::token::{Token, TokenType, TokenMatch};
use crate::parser::advance;

pub struct ComplexMatcher {
    states: Vec<Box<dyn Matcher>>,
    match_type: TokenMatch,
    min_match_amount: i32,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for ComplexMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        let mut running_count = match_count;
        match self.match_type {
            TokenMatch::Regular => {
                for _ in 0..self.min_match_amount {
                    match self.match_regular(iter, running_count) {
                        MatchResult::Success(count) => { running_count = count }
                        MatchResult::Failed(count) => return MatchResult::Failed(count)
                    }
                }

                return match &self.next {
                    Some(next_matcher) => next_matcher.match_exp(iter, running_count),
                    None => MatchResult::Success(running_count)
                }
            }
            TokenMatch::MultiMatch => {
                let mut attempt = 0;
                loop {
                    match self.match_regular(iter, running_count) {
                        MatchResult::Success(count) => {
                            running_count = count
                        }
                        MatchResult::Failed(count) => {
                            if attempt >= self.min_match_amount {
                                 return match &self.next {
                                     Some(next_matcher) => next_matcher.match_exp(iter, running_count),
                                     None => MatchResult::Success(running_count)
                                 }
                            } else {
                                return MatchResult::Failed(count)
                            }
                        }
                    }
                    attempt += 1;
                }
            }
        };
    }
}

impl ComplexMatcher {
    pub fn from_list(tokens: &Vec<Token>, match_type: TokenMatch, min_match_amount: i32, next_matcher: Option<Box<dyn Matcher>>) -> ComplexMatcher {
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
                    Box::new(ComplexMatcher::from_list(token_list, t.t_match, t.min_match, next_link))
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

        ComplexMatcher { states, match_type, min_match_amount, next: next_matcher }
    }

    fn match_regular(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        let mut result = MatchResult::Success(match_count);
        for state in self.states.iter().rev() {
            let mut current_iter = iter.clone();
            result = state.match_exp(&mut current_iter, match_count);

            if let MatchResult::Success(new_count) = result {
                advance(iter, (new_count - match_count) as usize);
                return result
            }
        }
        result
    }
}
