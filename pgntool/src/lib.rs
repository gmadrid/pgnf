use thiserror::Error;
use toolpack::{spew_at_level, verbose}; // TODO: figure out how to eliminate use spew_at_level

mod pgn_parser;
pub use pgn_parser::{parse_pgn, PgnDatabase};

#[derive(Debug, Error)]
pub enum PgnError {
    // TODO: make this a more useful error.
    //#[error("Unexpected Input: {0}")]
    //UnexpectedInput(String),
    #[error("Unmatched input for {0}: {1}")]
    UnmatchedInput(&'static str, String),

    #[error("Unexpected end of input received while parsing {0}")]
    UnexpectedEndOfInput(&'static str),

    #[error("Expected {0}, but found {1}.")]
    UnexpectedChar(char, char),

    #[error("{0}")]
    ToolPack(#[from] toolpack::TPError),
}

type Result<T> = std::result::Result<T, PgnError>;
