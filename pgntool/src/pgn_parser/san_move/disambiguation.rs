use crate::pgn_parser::san_move::file::File;
use crate::pgn_parser::san_move::rank::Rank;
use crate::pgn_parser::san_move::square::Square;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub enum Disambiguation {
    FileLetter(File),
    RankNumber(Rank),
    SquareCoord(Square),
    None,
}

impl From<File> for Disambiguation {
    fn from(file: File) -> Self {
        Disambiguation::FileLetter(file)
    }
}

impl From<Rank> for Disambiguation {
    fn from(rank: Rank) -> Self {
        Disambiguation::RankNumber(rank)
    }
}

impl From<Square> for Disambiguation {
    fn from(square: Square) -> Self {
        Disambiguation::SquareCoord(square)
    }
}

impl GrammarNode for Disambiguation {
    fn check_start(s: &str) -> bool {
        // Square is covered by File::check_start().
        File::check_start(s) || Rank::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // Try to read the Square first, consuming two characters.
        dbg!("FOOBAR");
        if Square::check_start(dbg!(s)) {
            if let Ok((square, remaining)) = Square::parse(s) {
                return Ok((Self::from(square), remaining));
            }
        }

        // If the Square doesn't work, then it might be just a File.
        if File::check_start(dbg!(s)) {
            if let Ok((file, remaining)) = File::parse(s) {
                return Ok((Self::from(file), remaining));
            }
        }

        // Otherwise, it's just a Rank.
        if Rank::check_start(dbg!(s)) {
            if let Ok((rank, remaining)) = Rank::parse(s) {
                return Ok((Self::from(rank), remaining));
            }
        }

        panic!("Someone forgot to call check_start()");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_rank_with_tail {
        ($rank:literal, $tail:literal, $parsed:expr) => {
            assert_eq!(
                (Disambiguation::RankNumber(Rank::from($rank)), $tail),
                $parsed.unwrap()
            )
        };
    }

    macro_rules! assert_file_with_tail {
        ($file:literal, $tail:literal, $parsed:expr) => {
            assert_eq!(
                (Disambiguation::FileLetter(File::from($file)), $tail),
                $parsed.unwrap()
            )
        };
    }

    macro_rules! assert_square_with_tail {
        ($file:literal, $rank:literal, $tail:literal, $parsed:expr) => {
            assert_eq!(
                (
                    Disambiguation::SquareCoord(Square {
                        rank: $rank.into(),
                        file: $file.into(),
                    }),
                    $tail
                ),
                $parsed.unwrap()
            )
        };
    }

    #[test]
    fn test_start() {
        assert!(Disambiguation::check_start("a"));
        assert!(Disambiguation::check_start("8"));
        assert!(Disambiguation::check_start("a8"));

        assert!(!Disambiguation::check_start("t"));
        assert!(!Disambiguation::check_start("9"));
    }

    #[test]
    fn test_parse() {
        assert_file_with_tail!('a', "TAIL", Disambiguation::parse("aTAIL"));
        assert_file_with_tail!('d', " SPACE", Disambiguation::parse("d SPACE"));

        assert_rank_with_tail!('8', "TAIL", Disambiguation::parse("8TAIL"));
        assert_rank_with_tail!('1', " SPACE", Disambiguation::parse("1 SPACE"));

        assert_square_with_tail!('a', '8', "TAIL", Disambiguation::parse("a8TAIL"));
        assert_square_with_tail!('c', '6', " SPACE", Disambiguation::parse("c6 SPACE"));
    }
}
