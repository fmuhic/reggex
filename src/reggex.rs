use itertools::multipeek;
use itertools::MultiPeek;
use std::str::Chars;

use crate::constant::STATE_SIZE;
use crate::parser::parse_expression;
use crate::token::Token;
type State = [u8; 130];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatchResult {
    Start,
    End,
    Success,
    Failed,
    Repeat
}

pub trait Matcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>) -> MatchResult;
}

pub struct SimpleMatcher {
    state: State,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for SimpleMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>) -> MatchResult {
        match simple_match(&self.state, iter) {
            MatchResult::Success => match &self.next {
                Some(next_matcher) => next_matcher.match_exp(iter),
                None => MatchResult::Success
            },
            _ => MatchResult::Failed
        }
    }
}


impl SimpleMatcher {
    fn from_token(token: &Token, next_matcher: Option<Box<dyn Matcher>>) -> SimpleMatcher {
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

pub struct ComplexMatcher {
    state: Option<Box<dyn Matcher>>,
    next: Option<Box<dyn Matcher>>
}

impl Matcher for ComplexMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>) -> MatchResult {
        match &self.state {
            Some(matcher) => match matcher.match_exp(iter) {
                MatchResult::Success => match &self.next {
                    Some(next_matcher) => next_matcher.match_exp(iter),
                    None => MatchResult::Success
                },
                _ => MatchResult::Failed
            },
            None => MatchResult::Success
        }
    }
}

impl ComplexMatcher {
    fn from_list(tokens: &Vec<Token>, next_matcher: Option<Box<dyn Matcher>>) -> ComplexMatcher {
        let mut next_link = None;
        for t in tokens.iter().rev() {
            let matcher: Box<dyn Matcher> = match t {
                Token::Complex(token_list, match_type) => {
                    Box::new(ComplexMatcher::from_list(token_list, next_link))
                },
                _ => Box::new(SimpleMatcher::from_token(t, next_link))
            };
            next_link = Some(matcher);
        }
        ComplexMatcher { state: next_link, next: next_matcher }
    }
}


fn simple_match(state: &State, iter: &mut MultiPeek<Chars>) -> MatchResult {
    if match_state(state, iter.peek()) {
        iter.next();
        MatchResult::Success
    } else {
        iter.reset_peek();
        MatchResult::Failed
    }
}

fn match_state(state: &State, chr: Option<&char>) -> bool {
    match chr {
        Some(c) => {
            println!("Matching char '{}'", c);
            state[*c as usize] == 1
        },
        None => false
    }
}

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
        match self.matcher.match_exp(iter) {
            MatchResult::Success => match_endline(iter),
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
