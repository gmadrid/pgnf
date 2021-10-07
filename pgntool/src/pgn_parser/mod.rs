use crate::{PgnError, Result};

trait GrammarNode {
    // Returns true if the first character of the string is a valid first letter for the
    // GrammarNode.
    //
    // - should not trim white space from the start of the string. (That is the caller's
    //   responsibility.)
    fn check_start(s: &str) -> bool;

    // Returns a parsed node and the 'tail' (the part of the string remaining after parsing
    // the node).
    //
    // - should assume that check_start() has been called and returned 'true' on the string.
    // - should not trim white space from the start of the string. (That is the caller's
    //   responsibility.)
    // - may trim white space from inside the parsed string.
    // - should not trim trailing white space.

    // Returns a parsed node and the 'tail'. (See parse_wrapped for more details.)
    //
    // Also, check validity of the parse by checking the follow set.
    //
    // Implementations should _not_ override this.
    fn parse(s: &str) -> Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (node, follow) = Self::parse_wrapped(s)?;

        if Self::valid_follow(follow) {
            Ok((node, follow))
        } else {
            Err(PgnError::UnmatchedFollowSet)
        }
    }

    // TODO: document this.
    fn valid_follow(_: &str) -> bool {
        true
    }

    // Returns a parsed node and the 'tail' (the part of the string remaining after parsing
    // the node).
    //
    // - should assume that check_start() has been called and returned 'true' on the string.
    // - should not trim white space from the start of the string. (That is the caller's
    //   responsibility.)
    // - may trim white space from inside the parsed string.
    // - should not trim trailing white space.
    fn parse_wrapped(s: &str) -> Result<(Self, &str)>
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
