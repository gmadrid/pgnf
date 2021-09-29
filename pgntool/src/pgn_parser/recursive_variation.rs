use crate::pgn_parser::GrammarNode;

pub struct RecursiveVariation {}

impl GrammarNode for RecursiveVariation {
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
