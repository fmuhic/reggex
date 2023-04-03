use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::{ Matcher, MatchResult };

pub struct EndLineMatcher {
    next: Option<Box<dyn Matcher>>
}

impl Matcher for EndLineMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        let result = match iter.peek() {
            Some(_) => MatchResult::Failed(match_count),
            None => {
                match &self.next {
                    Some(_) => MatchResult::Failed(match_count),
                    None => MatchResult::Success(match_count)
                }
            }
        };
        iter.reset_peek();
        result
    }
}

impl EndLineMatcher {
    pub fn new(next_matcher: Option<Box<dyn Matcher>>) -> EndLineMatcher {
        EndLineMatcher { next: next_matcher }
    }
}
