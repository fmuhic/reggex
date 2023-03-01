use crate::parser::parse_expression;
use crate::token::Token;

type FinateStateMachine = Vec<Box<dyn Matcher>>;

const STATE_SIZE: usize = 130;
const LINE_START: usize = 128;
const LINE_END: usize = 129;

#[allow(dead_code)]
#[derive(PartialEq)]
enum MatchResult {
    FAILED,
    CURRENT,
    NEXT
}

trait Matcher {
    fn match_char(&self, ch: char) -> MatchResult;
}

struct StateMatcher {
    state: [u8; STATE_SIZE]
}

impl Matcher for StateMatcher {
    fn match_char(&self, ch: char) -> MatchResult {
        let state_index = match ch {
            '^' => LINE_START,
            '$' => LINE_END,
            _ => ch as usize

        };

        if self.state[state_index] == 1 {
            MatchResult::NEXT
        }
        else {
            MatchResult::FAILED
        }
    }
}

impl StateMatcher {
    pub fn new(token: &Token) -> StateMatcher {
        match token {
            Token::SingleMatch(ch) => {
                let state_value = match ch {
                    '^' => LINE_START,
                    '$' => LINE_END,
                    _ => *ch as usize
                };
                let mut state = [0; STATE_SIZE];
                state[state_value] = 1;
                StateMatcher{state}
            }
            Token::RangeMatch(range) => {
                #[allow(unused_mut)]
                let mut state: [u8; 130] = core::array::from_fn(|i| {
                    if range.contains(&(i as u8)) { 1 } else { 0 }
                });
                StateMatcher{state}
            }
            _ => {
                unreachable!("Token processor not implemented");
            }
        }
    }
}

pub struct Reggex {
    fsm: FinateStateMachine
}

impl Reggex {
    pub fn new(exp: &str) -> Reggex {
        let mut fsm = FinateStateMachine::new();
        let tokens = parse_expression(exp);

        for t in tokens {
            fsm.push(Box::new(StateMatcher::new(&t)));
        }

        Reggex { fsm }
    }

    pub fn matches(&self, exp: &str) -> bool {
        let mut current_state = self.get_default_state();

        for letter in exp.chars() {
            if current_state >= self.fsm.len() {
                return false;
            }
            match self.fsm[current_state].match_char(letter) {
                MatchResult::FAILED => return false,
                MatchResult::NEXT => current_state += 1,
                MatchResult::CURRENT => {}
            }
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
        if matches!(self.fsm[default_state].match_char('^'), MatchResult::NEXT) {
            default_state + 1
        } else {
            default_state
        }
    }

    fn match_line_end(&self, current_state: usize) -> bool {
        current_state == self.fsm.len() - 1 &&
        matches!(self.fsm[current_state].match_char('$'), MatchResult::NEXT)
    }
}
