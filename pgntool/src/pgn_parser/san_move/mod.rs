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
        (&piecespec, s);

        let capture_pair = if Capture::check_start(s) {
            Capture::parse(s).ok()
        } else {
            None
        };
        (&capture_pair, s);

        let s = capture_pair
            .map(|(_, cap_remaining)| cap_remaining)
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

        let s = check_pair
            .map(|(_, check_remaining)| check_remaining)
            .unwrap_or(s);

        let sanmove = SanMove {
            piece: piecespec.piece,
            destination,
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
        assert_eq!(3, std::mem::size_of::<SanMove>());
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
                        piece: $piece,
                        destination: Square::parse($square).map(|(s, _)| s).unwrap(),
                    },
                    $tail
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_simple() {
        assert_simple_match!(Piece::Queen, "g4", "TAIL", "Qg4TAIL");
        assert_simple_match!(Piece::Pawn, "e5", " SPACE", "e5 SPACE");
    }
}
