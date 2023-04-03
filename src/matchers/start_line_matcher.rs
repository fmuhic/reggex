use itertools::MultiPeek;
use std::str::Chars;

use super::matcher::{ Matcher, MatchResult };

pub struct StartLineMatcher {
    next: Option<Box<dyn Matcher>>
}

impl Matcher for StartLineMatcher {
    fn match_exp(&self, iter: &mut MultiPeek<Chars>, match_count: i32) -> MatchResult {
        if match_count == 0 {
            match &self.next {
                Some(next_matcher) => {
                    next_matcher.match_exp(iter, match_count)
                }
                None => MatchResult::Success(match_count)
            }
        } else {
            MatchResult::Failed(match_count)
        }

    }
}

impl StartLineMatcher {
    pub fn new(next_matcher: Option<Box<dyn Matcher>>) -> StartLineMatcher {
        StartLineMatcher { next: next_matcher }
    }
}
