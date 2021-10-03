use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgnError {
    #[error("Bad number format while parsing {0}: {1}")]
    ParseIntError(&'static str, ParseIntError),

    #[error("The input ended unexpectedly while parsing {0}")]
    UnexpectedEOF(&'static str),

    #[error("Unexpected input while parsing {0}: {1}")]
    UnexpectedInput(&'static str, String),

    // This shouldn't be user-visible.
    // This is used when a parse is rejected because of the character immediately after
    // the parsed input. This is required because parts of the grammar are ambiguous.
    #[error("Unmatched follow set while parsing {0}")]
    UnmatchedFollowSet(&'static str),

    #[error("Unexpected character while parsing {0}: {1}")]
    UnmatchedChar(&'static str, char),
}
