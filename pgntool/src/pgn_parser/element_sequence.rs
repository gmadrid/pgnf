use crate::pgn_parser::element::Element;
use crate::pgn_parser::recursive_variation::RecursiveVariation;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ElementSequence {
    sequence: Vec<SequenceMember>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SequenceMember {
    Move(Element),
    Variation(RecursiveVariation),
}

/*
  <element-sequence> ::= <element> <element-sequence>
                         <recursive-variation> <element-sequence>
                         <empty>
*/
impl GrammarNode for ElementSequence {
    fn check_start(s: &str) -> bool {
        Element::check_start(s) || RecursiveVariation::check_start(s)
    }

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let mut sequence: Vec<SequenceMember> = vec![];
        let mut s = s;

        loop {
            if Element::check_start(s) {
                let element_result = Element::parse(s);
                if matches!(element_result, Err(PgnError::UnmatchedFollowSet)) {
                    break;
                }
                let (element, remainder) = element_result?;
                sequence.push(SequenceMember::Move(element));
                s = remainder.trim_start();
            } else if RecursiveVariation::check_start(s) {
                let (variation, remainder) = RecursiveVariation::parse(s)?;
                sequence.push(SequenceMember::Variation(variation));
                s = remainder.trim_start();
            } else {
                break;
            }
        }

        Ok((ElementSequence { sequence }, s))
    }
}
