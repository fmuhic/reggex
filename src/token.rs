use std::ops::Range;

#[derive(Debug, Clone, Copy)]
pub enum MatchType {
    Regular,
    NoneOrMany,
    OneOrMany
}

#[derive(Debug)]
pub enum Token {
    Aleternation,
    StartLine(MatchType),
    EndLine(MatchType),
    SingleMatch(char, MatchType),
    MultiMatch(String, MatchType),
    RangeMatch(Range<u8>, MatchType),
    Complex(Vec<Token>, MatchType)
}
