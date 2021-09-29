use crate::pgn_parser::GrammarNode;

pub struct Element {}

/*
  <element> ::= <move-number-indication>
            ::= <SAN-move>
            ::= <numeric-annotation-glyph>

*/
impl GrammarNode for Element {
    fn check_start(s: &str) -> bool {
        todo!()
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        todo!()
    }
}
