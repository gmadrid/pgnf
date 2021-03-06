use crate::pgn_error::PgnError;
use crate::pgn_parser::element_sequence::ElementSequence;
use crate::pgn_parser::GrammarNode;

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

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        if !s.starts_with('(') {
            return Err(PgnError::UnexpectedInput(
                "Recursive variation",
                s.to_string(),
            ));
        }

        // skip the '(' and spaces.
        let s = s[1..].trim();

        let (sequence, s) = if ElementSequence::check_start(s) {
            ElementSequence::parse(s)?
        } else {
            (ElementSequence::default(), s)
        };

        let s = s.trim();

        if !s.starts_with(')') {
            return Err(PgnError::UnexpectedInput(
                "Recursive variation",
                s.to_string(),
            ));
        }

        // skip the ')'
        Ok((RecursiveVariation { sequence }, &s[1..]))
    }
}
