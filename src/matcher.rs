use crate::constant::STATE_SIZE;
use crate::token::MatchType;

use itertools::MultiPeek;
use std::str::Chars;

struct OneOrManyMatcher {
    state: [u8; STATE_SIZE]
}

struct NoneOrManyMatcher {
    state: [u8; STATE_SIZE]
}

pub struct RegularMatcher {
    state: [u8; STATE_SIZE],
    level: MatchType
}

pub struct StartLineMatcher {
    level: MatchType
}

pub struct EndLineMatcher {
    level: MatchType
}

pub trait Matcher {
    fn match_char(&self, iter: &mut MultiPeek<Chars>, prev_match: MatchResult) -> MatchResult;
}

impl Matcher for RegularMatcher {
    fn match_char(&self, iter: &mut MultiPeek<Chars>, _: MatchResult) -> MatchResult {
        match self.level {
            MatchType::Regular => self.simple_match(iter),
            MatchType::OneOrMany => self.one_or_many_match(iter),
            MatchType::NoneOrMany => self.none_or_many_match(iter)
        }
    }
}

impl RegularMatcher {
    fn simple_match(&self, iter: &mut MultiPeek<Chars>) -> MatchResult {
        if self.match_state(iter.peek()) {
            iter.next();
            MatchResult::Success
        } else {
            MatchResult::Failed
        }
    }

    fn one_or_many_match(&self, iter: &mut MultiPeek<Chars>) -> MatchResult {
        let match_count = 0;
        let next = iter.peek();

        while self.match_state(next) {
            match_count += 1;
            next = iter.peek();
        }

        if match_count == 0 {
            MatchResult::Failed
        }
        else {
            for _ in 0..match_count {
                iter.next();
            }
            MatchResult::Success
        }
    }

    fn none_or_many_match(&self, iter: &mut MultiPeek<Chars>) -> MatchResult {
        let match_count = 0;
        let next = iter.peek();

        while self.match_state(next) {
            match_count += 1;
            next = iter.peek();
        }

        for _ in 0..match_count {
            iter.next();
        }

        MatchResult::Success
    }

    fn match_state(&self, chr: Option<&char>) -> bool {
        match chr {
            Some(c) => {
                self.state[*c as usize] == 1
            },
            None => false
        }
    }
}

impl Matcher for StartLineMatcher {
    fn match_char(&self, _: &mut MultiPeek<Chars>, prev_match: MatchResult) -> MatchResult {
        match self.level {
            MatchType::Regular => match prev_match {
                MatchResult::Start => MatchResult::Success,
                _ => MatchResult::Failed
            },
            _ => MatchResult::Failed
        }
    }
}

impl Matcher for EndLineMatcher {
    fn match_char(&self, iter: &mut MultiPeek<Chars>, prev_match: MatchResult) -> MatchResult {
        match self.level {
            MatchType::Regular => match iter.peek() {
                None => MatchResult::Success,
                _ => MatchResult::Failed
            },
            _ => MatchResult::Failed
        }
    }
}
