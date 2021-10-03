use crate::pgn_parser::element_sequence::ElementSequence;
use crate::pgn_parser::game_termination::GameTermination;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub struct MovetextSection {
    element_sequence: ElementSequence,
    game_termination: GameTermination,
}

/*
  <movetext-section> ::= <element-sequence> <game-termination>
*/
impl GrammarNode for MovetextSection {
    fn check_start(s: &str) -> bool {
        ElementSequence::check_start(s) || GameTermination::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (element_sequence, s) = ElementSequence::parse(s)?;

        let s = s.trim_start();

        let (game_termination, s) = GameTermination::parse(s)?;

        Ok((
            MovetextSection {
                element_sequence,
                game_termination,
            },
            s,
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //#[test]
    // TODO: make this work.
    fn test_with_termination() {
        MovetextSection::parse("e4 c6 0-1").unwrap();
    }
}
