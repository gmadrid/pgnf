use crate::pgn_parser::san_move::file::File;
use crate::pgn_parser::san_move::rank::Rank;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Square {
    pub rank: Rank,
    pub file: File,
}

impl Debug for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}",
                          ('a' as u8 + self.file.0 - 1) as char,
               self.rank.0)
    }
}

impl GrammarNode for Square {
    fn check_start(s: &str) -> bool {
        File::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (file, s) = if File::check_start(s) {
            File::parse(s)?
        } else {
            return Err(PgnError::InvalidCheckChar('Z'));
        };

        let (rank, s) = if Rank::check_start(s) {
            Rank::parse(s)?
        } else {
            return Err(PgnError::InvalidCheckChar('Q'));
        };

        Ok((Square { rank, file }, s))
    }
}

#[cfg(test)]
mod square {
    use super::*;
    use crate::assert_with_tail;

    #[test]
    fn test_start() {
        assert!(Square::check_start("a8"));
        assert!(Square::check_start("h2"));
        assert!(!Square::check_start("z8"));
        assert!(!Square::check_start("8"));
    }

    #[test]
    fn test_parse() {
        assert_with_tail!('a', '8', "TAIL", Square::parse("a8TAIL"));
        assert_with_tail!('h', '1', " SPACE", Square::parse("h1 SPACE"));
        assert_with_tail!('c', '6', "", Square::parse("c6"));

        assert!(Square::parse("aTAIL").is_err());
    }
}
