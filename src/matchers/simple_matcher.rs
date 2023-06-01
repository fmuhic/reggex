use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::State;
use super::matcher::{ Matcher, MatchResult };
use crate::constant::STATE_SIZE;
use crate::token::{Token, TokenType, TokenMatch};

pub struct SimpleMatcher {
    state: State,
    match_type: TokenMatch,
    min_match_amount: i32,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for SimpleMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        let mut running_count = match_count;
        match self.match_type {
            TokenMatch::Regular => {
                for _ in 0..self.min_match_amount {
                    match simple_match(&self.state, iter) {
                        true => {
                            running_count += 1
                        }
                        false => return MatchResult::Failed(running_count)
                    }
                }
            }
            TokenMatch::MultiMatch => {
                let mut attempt = 0;
                loop {
                    match simple_match(&self.state, iter) {
                        true => {
                            running_count += 1
                        }
                        false => {
                            if attempt >= self.min_match_amount {
                                break
                            } else {
                                return MatchResult::Failed(running_count)
                            }
                        }
                    }
                    attempt += 1;
                }
            }
        }

        match &self.next {
            Some(next_matcher) => return next_matcher.match_exp(iter, running_count),
            None => return MatchResult::Success(running_count)
        }
    }
}


impl SimpleMatcher {
    pub fn from_token(token: &Token, next_matcher: Option<Box<dyn Matcher>>) -> SimpleMatcher {
        match &token.kind {
            TokenType::SingleMatch(ch) => {
                let mut state = [0; STATE_SIZE];
                state[*ch as usize] = 1;
                SimpleMatcher {
                    state,
                    next: next_matcher,
                    match_type: token.t_match,
                    min_match_amount: token.min_match
                }
            }
            TokenType::RangeMatch(range) => {
                #[allow(unused_mut)]
                let mut state: [u8; 130] = core::array::from_fn(|i| {
                    if range.contains(&(i as u8)) { 1 } else { 0 }
                });
                SimpleMatcher {
                    state,
                    next: next_matcher,
                    match_type: token.t_match,
                    min_match_amount: token.min_match
                }
            }
            t => {
                unreachable!("Token processor not implemented for token {:?}", t);
            }
        }
    }
}


pub fn simple_match(state: &State, iter: &mut MultiPeek<Chars>) -> bool {
    match &iter.peek() {
        Some(&c) => {
            if state[c as usize] == 1 {
                iter.next();
                true
            } else {
                iter.reset_peek();
                false
            }
        }
        None => {
            iter.reset_peek();
            false
        }
    }
}
