use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::State;
use super::matcher::{ Matcher, MatchResult, simple_match };
use crate::constant::STATE_SIZE;
use crate::token::Token;

pub struct SimpleMatcher {
    state: State,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for SimpleMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        match simple_match(&self.state, iter, match_count) {
            true => match &self.next {
                Some(next_matcher) => next_matcher.match_exp(iter, match_count + 1),
                None => MatchResult::Success(match_count + 1)
            },
            false => MatchResult::Failed(match_count)
        }
    }
}


impl SimpleMatcher {
    pub fn from_token(token: &Token, next_matcher: Option<Box<dyn Matcher>>) -> SimpleMatcher {
        match token {
            Token::SingleMatch(ch, match_type) => {
                let mut state = [0; STATE_SIZE];
                state[*ch as usize] = 1;
                SimpleMatcher { state, next: next_matcher }
            }
            Token::RangeMatch(range, match_type) => {
                #[allow(unused_mut)]
                let mut state: [u8; 130] = core::array::from_fn(|i| {
                    if range.contains(&(i as u8)) { 1 } else { 0 }
                });
                SimpleMatcher { state, next: next_matcher }
            }
            _ => {
                unreachable!("Token processor not implemented");
            }
        }
    }
}

