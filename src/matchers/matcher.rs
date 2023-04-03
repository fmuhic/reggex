use std::str::Chars;
use itertools::MultiPeek;
pub type State = [u8; 130];

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatchResult {
    Success(i32),
    Failed(i32)
}

pub trait Matcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult;
}

pub fn simple_match(state: &State, iter: &mut MultiPeek<Chars>, count: i32) -> bool {
    match &iter.peek() {
        Some(&c) => {
            println!("Matching char '{}', current count is {}", c, count);
            iter.next();
            state[c as usize] == 1
        }
        None => {
            iter.reset_peek();
            false
        }
    }
}
