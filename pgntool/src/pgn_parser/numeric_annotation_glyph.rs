use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub struct NumericAnnotationGlyph {}

impl GrammarNode for NumericAnnotationGlyph {
    fn check_start(s: &str) -> bool {

        todo!("NAG: '{}'", s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        todo!()
    }
}
