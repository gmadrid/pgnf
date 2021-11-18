mod database;
pub use database::*;

mod element;
pub use element::*;

mod movetext;
pub use movetext::*;

mod tags;
pub use tags::*;

mod tokens;
pub use tokens::*;

use crate::PgnError;
use chumsky::prelude::*;

pub fn parse_pgn(s: &str) -> crate::Result<PgnDatabase> {
    pgn_database_matcher()
        .parse(s)
        .map_err(PgnError::SimpleError)
}
