use crate::pgn_parser::GrammarNode;

#[derive(Debug)]
pub struct MovetextSection {}

impl GrammarNode for MovetextSection {
    fn check_start(s: &str) -> bool {
        // TODO: write this
        false
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        todo!()
    }
}
