use crate::pgn_parser::GrammarNode;

pub struct NumericAnnotationGlyph {}

impl GrammarNode for NumericAnnotationGlyph {
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
