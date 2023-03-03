use crate::parser::parse_expression;
use crate::token::{Token, MatchType};

use itertools::multipeek;

type FinateStateMachine = Vec<Box<dyn Matcher>>;

const STATE_SIZE: usize = 130;
const LINE_START: usize = 128;
const LINE_END: usize = 129;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
enum MatchResult {
    Success,
    Failed,
    Repeat
}

trait Matcher {
    fn match_char(&self, ch: &char, prev_match: MatchResult) -> MatchResult;
}

struct StateMatcher {
    state: [u8; STATE_SIZE],
    match_type: MatchType
}

impl Matcher for StateMatcher {
    fn match_char(&self, ch: &char, prev_match: MatchResult) -> MatchResult {
        match self.match_type {
            MatchType::Regular => {
                if self.match_current_state(ch) {
                    MatchResult::Success
                } else {
                    MatchResult::Failed
                }
            }
            MatchType::NoneOrMany => {
                if self.match_current_state(ch) {
                    MatchResult::Repeat
                } else {
                    MatchResult::Success
                }
            }
            MatchType::OneOrMany => {
                match prev_match {
                    MatchResult::Success => if self.match_current_state(ch) {
                        MatchResult::Repeat
                    } else {
                        MatchResult::Failed
                    }
                    _ => if self.match_current_state(ch) {
                        MatchResult::Repeat
                    } else {
                        MatchResult::Success
                    }
                }
            }
        }
    }
}

impl StateMatcher {
    pub fn new(token: &Token) -> StateMatcher {
        match token {
            Token::SingleMatch(ch, match_type) => {
                let state_value = match ch {
                    '^' => LINE_START,
                    '$' => LINE_END,
                    _ => *ch as usize
                };
                let mut state = [0; STATE_SIZE];
                state[state_value] = 1;
                StateMatcher{state, match_type: match_type.clone()}
            }
            Token::RangeMatch(range, match_type) => {
                #[allow(unused_mut)]
                let mut state: [u8; 130] = core::array::from_fn(|i| {
                    if range.contains(&(i as u8)) { 1 } else { 0 }
                });
                StateMatcher{state, match_type: match_type.clone()}
            }
            _ => {
                unreachable!("Token processor not implemented");
            }
        }
    }

    fn match_current_state(&self, ch: &char) -> bool {
        let state_index = match ch {
            '^' => LINE_START,
            '$' => LINE_END,
            _ => *ch as usize

        };

        self.state[state_index] == 1
    } 
}

pub struct Reggex {
    fsm: FinateStateMachine
}

impl Reggex {
    pub fn new(exp: &str) -> Reggex {
        let mut fsm = FinateStateMachine::new();
        let tokens = parse_expression(exp);
        println!("Tokens parsed: {:?}", tokens);

        for t in tokens {
            fsm.push(Box::new(StateMatcher::new(&t)));
        }

        Reggex { fsm }
    }

    pub fn matches(&self, exp: &str) -> bool {
        let mut current_state = self.get_default_state();

        let mut prev_match = MatchResult::Success;

        let mut iter = multipeek(exp.chars());
        while iter.peek() != None {
            iter.reset_peek();
            if current_state >= self.fsm.len() {
                return false;
            }
            let next_match = self.fsm[current_state].match_char(iter.peek().unwrap(), prev_match);
            iter.reset_peek();

            println!("matching {}, prev: {:?}, next: {:?}, state = {}", iter.peek().unwrap_or(&'x'), prev_match, next_match, current_state);
            if prev_match == MatchResult::Repeat && next_match == MatchResult::Success {
            } else {
                iter.next();
            }

            iter.reset_peek();
            match next_match  {
                MatchResult::Failed => return false,
                MatchResult::Success => {
                    current_state += 1;
                },
                MatchResult::Repeat => {}
            }
            prev_match = next_match

        }

        return if current_state < self.fsm.len() {
            self.match_line_end(current_state)
        }
        else {
            true
        }
    }

    fn get_default_state(&self) -> usize {
        let default_state = 0;
        let prev_match = MatchResult::Success;
        if matches!(self.fsm[default_state].match_char(&'^', prev_match), MatchResult::Success) {
            default_state + 1
        } else {
            default_state
        }
    }

    fn match_line_end(&self, current_state: usize) -> bool {
        let prev_match = MatchResult::Success;
        current_state == self.fsm.len() - 1 &&
        matches!(self.fsm[current_state].match_char(&'$', prev_match), MatchResult::Success)
    }
}
