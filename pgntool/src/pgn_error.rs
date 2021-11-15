use std::num::ParseIntError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PgnError {
    #[error("Bad number format while parsing {0}: {1}")]
    ParseIntError(&'static str, ParseIntError),

    // TODO: ARE THE FOLLOWING NECESSARY IN THE NEW REGIME //
    #[error("The input ended unexpectedly while parsing {0}")]
    UnexpectedEOF(&'static str),

    #[error("Unexpected input while parsing {0}: {1}")]
    UnexpectedInput(&'static str, String),

    #[error("Unexpected character while parsing {0}: {1}")]
    UnmatchedChar(&'static str, char),

    // NOT a user-visible error.
    // This is used when a parse is rejected because of the character immediately after
    // the parsed input. This is required because parts of the grammar are ambiguous.
    #[error("Unmatched follow set while parsing")]
    UnmatchedFollowSet,

    // NOT a user-visible error.
    // Used to indicate an element list which has been short-circuited by a disambiguated parse.
    #[error("Element loop terminated")]
    ElementSequenceTerminated,
}
