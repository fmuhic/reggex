use std::ops::Range;

#[derive(Debug)]
pub enum Token {
    SingleMatch(char),
    MultiMatch(String),
    RangeMatch(Range<char>),
    Complex(String)
}
