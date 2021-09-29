use crate::pgn_parser::GrammarNode;

pub struct MoveNumberIndication {}

impl GrammarNode for MoveNumberIndication {
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
