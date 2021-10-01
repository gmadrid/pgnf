use crate::pgn_parser::GrammarNode;
use crate::PgnError;

#[derive(Debug, Eq, PartialEq)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn,
}

impl GrammarNode for Piece {
    fn check_start(s: &str) -> bool {
        let chs: &[char] = &['P', 'N', 'B', 'R', 'Q', 'K'];
        s.starts_with(chs)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let piece = match s.chars().next() {
            Some('P') => Piece::Pawn,
            Some('N') => Piece::Knight,
            Some('B') => Piece::Bishop,
            Some('R') => Piece::Rook,
            Some('Q') => Piece::Queen,
            Some('K') => Piece::King,
            _ => return Err(PgnError::UnmatchedInput("Piece", s.to_string())),
        };
        Ok((piece, &s[1..]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_with_tail;
    use Piece::*;

    #[test]
    fn test_start() {
        assert!(Piece::check_start("K"));
        assert!(Piece::check_start("Q"));
        assert!(Piece::check_start("N"));
        assert!(Piece::check_start("B"));
        assert!(Piece::check_start("R"));
        assert!(Piece::check_start("P"));
        assert!(!Piece::check_start("S"));
        assert!(!Piece::check_start("Z"));
    }

    #[test]
    fn test_parse() {
        assert_with_tail!(King, "TAIL", Piece::parse("KTAIL"));
        assert_with_tail!(Queen, "TAIL", Piece::parse("QTAIL"));
        assert_with_tail!(Rook, "TAIL", Piece::parse("RTAIL"));
        assert_with_tail!(Bishop, "TAIL", Piece::parse("BTAIL"));
        assert_with_tail!(Knight, "TAIL", Piece::parse("NTAIL"));
        assert_with_tail!(Pawn, " SPACE", Piece::parse("P SPACE"));
    }
}
