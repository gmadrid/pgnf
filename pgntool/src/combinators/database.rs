/*
    <PGN-database> ::= <PGN-game> <PGN-database>
                       <empty>
    <PGN-game> ::= <tag-section> <movetext-section>
 */

use chumsky::prelude::*;
use crate::combinators::TagSection;

pub struct PgnDatabase;

pub struct PgnGame {
    tag_section: TagSection,
    //movetext_section: MovetextSection,
}

// fn pgn_database_matcher() -> impl Parser<char, PgnDatabase, Error = Simple<char>> {
//     todo!()
// }
//
// fn pgn_game_matcher() -> impl Parser<char, PgnGame, Error = Simple<char>> {
//     todo!()
// }