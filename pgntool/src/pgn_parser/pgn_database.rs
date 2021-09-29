use crate::pgn_parser::pgn_game::PgnGame;
use crate::pgn_parser::GrammarNode;

#[derive(Debug)]
pub struct PgnDatabase {
    pgn_games: Vec<PgnGame>,
}

/*
 <PGN-database> ::= <PGN-game> <PGN-database>
                    <empty>
*/
impl GrammarNode for PgnDatabase {
    fn check_start(s: &str) -> bool {
        PgnGame::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let mut pgn_games: Vec<PgnGame> = Default::default();
        let mut s = s;
        while PgnGame::check_start(s) {
            let (game, remainder) = PgnGame::parse(s)?;
            s = remainder.trim_start();
            pgn_games.push(game)
        }
        Ok((PgnDatabase { pgn_games }, s))
    }
}
