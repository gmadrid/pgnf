use crate::pgn_parser::element_sequence::ElementSequence;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;

#[derive(Debug, PartialEq, Eq)]
pub struct RecursiveVariation {
    sequence: ElementSequence,
}

/*
  <recursive-variation> ::= ( <element-sequence> )
*/
impl GrammarNode for RecursiveVariation {
    fn check_start(s: &str) -> bool {
        s.starts_with('(')
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        if !s.starts_with('(') {
            return Err(PgnError::UnmatchedInput(
                "Recursive variation",
                s.to_string(),
            ));
        }

        let s = &s[1..];

        let (sequence, s) = if ElementSequence::check_start(s) {
            ElementSequence::parse(s)?
        } else {
            (ElementSequence::default(), s)
        };

        if !s.starts_with(')') {
            return Err(PgnError::UnmatchedInput(
                "Recursive variation",
                s.to_string(),
            ));
        }

        Ok((RecursiveVariation { sequence }, &s[1..]))
    }
}