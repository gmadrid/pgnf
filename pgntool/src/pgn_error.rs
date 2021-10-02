use thiserror::Error;
use std::num::ParseIntError;

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

    #[error("Unexpected character found while parsing Check: {0}")]
    InvalidCheckChar(char),

    #[error("{0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("{0}")]
    ToolPack(#[from] toolpack::TPError),
}
