use crate::{PgnError, Result};
use std::str::FromStr;

trait GrammarNode {
    // TODO: document these
    fn check_start(s: &str) -> bool;
    fn parse(s: &str) -> Result<(Self, &str)>
    where
        Self: Sized;
}

mod game_termination;
mod pgn_database;
pub use pgn_database::PgnDatabase;
mod movetext_section;
mod pgn_game;
mod symbol;
mod tag_pair;
mod tag_section;

pub fn parse_pgn(s: impl AsRef<str>) -> Result<PgnDatabase> {
    let s = s.as_ref().trim_start();
    let (database, _) = PgnDatabase::parse(s)?;
    Ok(database)
}
