use crate::{PgnError, Result};
use std::str::FromStr;

trait GrammarNode {
    // TODO: document these
    fn check_start(s: &str) -> bool;
    fn parse(s: &str) -> Result<(Self, &str)>
    where
        Self: Sized;
}

mod element;
mod element_sequence;
mod game_termination;
mod move_number_indication;
mod movetext_section;
mod numeric_annotation_glyph;
mod pgn_database;
mod pgn_game;
mod recursive_variation;
mod san_move;
mod symbol;
mod tag_pair;
mod tag_section;

pub use pgn_database::PgnDatabase;

pub fn parse_pgn(s: impl AsRef<str>) -> Result<PgnDatabase> {
    let s = s.as_ref().trim_start();
    let (database, _) = PgnDatabase::parse(s)?;
    Ok(database)
}
