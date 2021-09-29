use crate::pgn_parser::element::Element;
use crate::pgn_parser::recursive_variation::RecursiveVariation;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, PartialEq, Eq)]
pub struct ElementSequence {}

/*
  <element-sequence> ::= <element> <element-sequence>
                         <recursive-variation> <element-sequence>
                         <empty>
*/
impl GrammarNode for ElementSequence {
    fn check_start(s: &str) -> bool {
        Element::check_start(s) || RecursiveVariation::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        todo!()
    }
}
