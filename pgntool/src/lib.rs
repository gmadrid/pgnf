mod combinators;
pub use combinators::parse_pgn;

mod pgn_error;
pub use pgn_error::PgnError;

type Result<T> = std::result::Result<T, PgnError>;
