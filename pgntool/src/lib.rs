//use toolpack::{spew_at_level, verbose}; // TODO: figure out how to eliminate use spew_at_level

mod pgn_error;
pub use pgn_error::PgnError;

mod pgn_parser;
pub use pgn_parser::{parse_pgn, PgnDatabase};

type Result<T> = std::result::Result<T, PgnError>;
