use disambiguation::Disambiguation::SquareCoord;
use file::File;
use piece::Piece;
use piece::Piece::{Bishop, Knight, Pawn, Queen, Rook};
use rank::Rank;
use square::Square;

use crate::pgn_parser::san_move::capture::Capture;
use crate::pgn_parser::san_move::check::Check;
use crate::pgn_parser::san_move::piecespec::PieceSpec;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;

mod capture;
mod check;
mod disambiguation;
mod file;
mod piece;
mod piecespec;
mod rank;
mod square;

#[derive(Debug, Eq, PartialEq)]
pub struct SanMove {
    piece: Piece,
    destination: Square,
    from_file: Option<File>,
    from_rank: Option<Rank>,
    check: Check,
}

fn first<T, U>(param: (T, U)) -> T {
    param.0
}

/*
  8.2.3: Movetext SAN (Standard Algebraic Notation)

  See reference document. Descrippion is too long to put here.

  Simple cases:
    Qg4
    e5

  Captures
    dxe5
    Qxf6

  Castles:
    O-O
    O-O-O

  Checks:
    Qf6+
    Nd7#

  Disambiguate:
    Rhe8
    N3e1

  Promotion:
    g8=Q
    dxe1=N

  Complex:
    Qa6xb7#
    fxg1=Q+

  A candidate grammar for a SAN move:
    SANMOVE ::= <PIECESPEC><CAPTURE><DESTINATION><CHECK>
    PIECESPEC ::= <PIECE><DISAMBIGUATION>
              ::= <empty>   // implied pawn
    CAPTURE ::= <PAWNFILE> 'x'
            ::= <empty>
    PAWNFILE ::= FILE
             ::= <empty>
    // Ironically, this is an ambiguous, context-sensitive parse.
    // It *must* be followed by either 'x' or SQUARE.
    // (Pawn captures never disambiguate, so PAWNFILE should always be empty.)
    // You can use this "follow set" to check for a valid DISAMBIGUATION.
    DISAMBIGUATION ::= RANK
                   ::= FILE
                   ::= SQUARE
                   ::= <empty>
    PIECE := [PNBRQK]
    FILE ::= [a-h]
    RANK ::= [1-8]
    SQUARE ::= <RANK><FILE>
    DESTINATION ::= SQUARE
    CHECK ::= [+#]?
*/
impl GrammarNode for SanMove {
    fn check_start(s: &str) -> bool {
        PieceSpec::check_start(s) || Capture::check_start(s) || Square::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (piecespec, s) = PieceSpec::parse(s).unwrap_or((PieceSpec::pawn(), s));
        dbg!(&piecespec, s);

        let capture_pair = if Capture::check_start(s) {
            Capture::parse(s).ok()
        } else {
            None
        };
        (&capture_pair, s);

        let s = capture_pair
            .as_ref()
            .map(|(_, cap_remaining)| *cap_remaining)
            .unwrap_or(s);

        let (destination, s) = if Square::check_start(s) {
            Square::parse(s)?
        } else {
            Err(PgnError::InvalidCheckChar('X'))?
        };

        let check_pair = if Check::check_start(s) {
            Check::parse(s).ok()
        } else {
            None
        };

        dbg!(&check_pair);
        let s = check_pair
            .as_ref()
            .map(|(_, check_remaining)| *check_remaining)
            .unwrap_or(s);

        let check = check_pair.map(first).unwrap_or(Check::None);

        let from_file = piecespec.disambiguation.file();

        let sanmove = SanMove {
            piece: piecespec.piece,
            destination,
            from_file,
            from_rank: None,
            check,
        };

        Ok((sanmove, s))
    }
}

pub fn if_some<T>(pred: bool, val: T) -> Option<T> {
    if pred {
        Some(val)
    } else {
        None
    }
}

pub fn if_some_with<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
    }
}

#[cfg(test)]
mod test_macros {
    #[macro_export]
    macro_rules! assert_with_tail {
        ($file:literal, $rank:literal, $tail:literal, $parsed:expr) => {
            assert_eq!(
                (
                    Square {
                        rank: Rank::from($rank),
                        file: File::from($file),
                    },
                    $tail
                ),
                $parsed.unwrap()
            )
        };
        ($value:expr, $tail:literal, $parsed:expr) => {
            assert_eq!(($value, $tail), $parsed.unwrap())
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pgn_parser::san_move::check::Check;

    #[test]
    fn test_size() {
        assert_eq!(8, std::mem::size_of::<SanMove>());
        assert_eq!(1, std::mem::size_of::<Piece>());
        assert_eq!(1, std::mem::size_of::<Rank>());
        assert_eq!(1, std::mem::size_of::<File>());
        assert_eq!(2, std::mem::size_of::<Square>());
        assert_eq!(1, std::mem::size_of::<Check>());
    }

    macro_rules! assert_simple_match {
        ($piece:expr, $square:literal, $tail:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(|(s, _)| s).unwrap(),
                        from_file: None,
                        from_rank: None,
                        check: Check::None,
                    },
                    $tail
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_simple() {
        assert_simple_match!("Q", "g4", "TAIL", "Qg4TAIL");
        assert_simple_match!("P", "e5", " SPACE", "e5 SPACE");
    }

    macro_rules! assert_capture {
        ($piece:literal, $square:literal, $pawn_file:literal, $tail:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(first).unwrap(),
                        from_file: File::parse($pawn_file).map(first).ok(),
                        from_rank: None,
                        check: Check::None,
                    },
                    $tail
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_capture() {
        assert_capture!("P", "e5", "d", "TAIL", "dxe5TAIL");
        assert_simple_match!("Q", "f6", " SPACE", "Qxf6 SPACE");
    }

    macro_rules! assert_check {
        ($piece:literal, $square:literal, $check:expr, $tail:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(first).unwrap(),
                        from_file: None,
                        from_rank: None,
                        check: $check,
                    },
                    $tail
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_checks() {
        assert_check!("Q", "f6", Check::Check, "TAIL", "Qf6+TAIL");
        assert_check!("N", "d7", Check::Mate, " SPACE", "Nd7# SPACE");
    }
}
