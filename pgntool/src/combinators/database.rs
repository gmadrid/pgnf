/*
   <PGN-database> ::= <PGN-game> <PGN-database>
                      <empty>
   <PGN-game> ::= <tag-section> <movetext-section>
*/

use crate::combinators::{
    movetext_section_matcher, tag_section_matcher, MovetextSection, TagSection,
};
use chumsky::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PgnDatabase {
    games: Vec<PgnGame>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PgnGame {
    tag_section: TagSection,
    movetext_section: MovetextSection,
}

pub fn pgn_database_matcher() -> impl Parser<char, PgnDatabase, Error = Simple<char>> {
    pgn_game_matcher()
        .padded()
        .repeated()
        .collect()
        .map(|gs| PgnDatabase { games: gs })
}

fn pgn_game_matcher() -> impl Parser<char, PgnGame, Error = Simple<char>> {
    tag_section_matcher()
        .padded()
        .then(movetext_section_matcher())
        .try_map(|(ts, mts), span| {
            if ts.pairs().is_empty() && mts.is_empty() {
                Err(Simple::custom(span, "Empty game matcher"))
            } else {
                Ok(PgnGame {
                    tag_section: ts,
                    movetext_section: mts,
                })
            }
        })
}

#[cfg(test)]
mod test {
    use super::*;

    const GAME_TEXT: &str = r#"[EndDate "2021.09.21"]
[Termination "VitaminG won by resignation"]

1. e4 c6 2. d4 d5 3. Nc3 dxe4 4. Nxe4 Nf6"#;

    #[test]
    fn test_database_matcher() {
        let matcher = pgn_database_matcher();

        matcher.parse(GAME_TEXT).unwrap();
    }

    #[test]
    fn test_game_matcher() {
        let matcher = pgn_game_matcher();

        matcher.parse(GAME_TEXT).unwrap();
    }

    #[test]
    fn test_empty_game() {
        let matcher = pgn_game_matcher();

        assert!(matcher.parse("").is_err());
    }
}
