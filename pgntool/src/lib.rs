mod combinators;

mod pgn_error;
pub use pgn_error::PgnError;

// mod pgn_parser;
// pub use pgn_parser::{parse_pgn, PgnDatabase};

#[derive(Debug)]
pub struct PgnDatabase;

pub fn parse_pgn(_: &str) -> Result<PgnDatabase> {
    Ok(PgnDatabase)
}

type Result<T> = std::result::Result<T, PgnError>;
