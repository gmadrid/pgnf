use file::File;
use piece::Piece;
use rank::Rank;
use square::Square;

use crate::pgn_parser::san_move::capture::Capture;
use crate::pgn_parser::san_move::check::Check;
use crate::pgn_parser::san_move::piecespec::PieceSpec;
use crate::pgn_parser::GrammarNode;
use crate::pgn_error::PgnError;

mod capture;
mod check;
mod disambiguation;
mod file;
mod piece;
mod piecespec;
mod rank;
mod square;

#[derive(Debug, Eq, PartialEq)]
pub enum SanMove {
    Move(SanMoveDetail),
    LongCastle(Check),
    ShortCastle(Check),
}

#[derive(Debug, Eq, PartialEq)]
pub struct SanMoveDetail {
    piece: Piece,
    destination: Square,
    from_file: Option<File>,
    from_rank: Option<Rank>,
    check: Check,
}

fn first<T, U>(param: (T, U)) -> T {
    param.0
}

impl SanMove {
    fn parse_castle(s: &str) -> crate::Result<(Self, &str)> {
        // Check for Long Castle first because short castle is a prefix of long castle.
        if let Some(s) = s.strip_prefix("O-O-O") {
            let (check, s) = Check::parse(s).unwrap_or((Check::None, s));
            Ok((SanMove::LongCastle(check), s))
        } else if let Some(s) = s.strip_prefix("O-O") {
            let (check, s) = Check::parse(s).unwrap_or((Check::None, s));
            Ok((SanMove::ShortCastle(check), s))
        } else {
            Err(PgnError::UnexpectedChar('X', 'X'))
        }
    }
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
    Ba3c5

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
        PieceSpec::check_start(s)
            || Capture::check_start(s)
            || Square::check_start(s)
            || s.starts_with('O')
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        if let Ok(castle_pair) = SanMove::parse_castle(s) {
            return Ok(castle_pair);
        }

        let (piecespec, s) = PieceSpec::parse(s).unwrap_or((PieceSpec::pawn(), s));

        let capture_pair = if Capture::check_start(s) {
            Capture::parse(s).ok()
        } else {
            None
        };

        let s = capture_pair
            .as_ref()
            .map(|(_, cap_remaining)| *cap_remaining)
            .unwrap_or(s);

        let (destination, s) = if Square::check_start(s) {
            Square::parse(s)?
        } else {
            return Err(PgnError::InvalidCheckChar('X'));
        };

        let check_pair = if Check::check_start(s) {
            Check::parse(s).ok()
        } else {
            None
        };

        let s = check_pair
            .as_ref()
            .map(|(_, check_remaining)| *check_remaining)
            .unwrap_or(s);

        let check = check_pair.map(first).unwrap_or(Check::None);

        let from_file = piecespec.disambiguation.file();
        let from_rank = piecespec.disambiguation.rank();

        let detail = SanMoveDetail {
            piece: piecespec.piece,
            destination,
            from_file,
            from_rank,
            check,
        };

        Ok((SanMove::Move(detail), s))
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
        assert_eq!(9, std::mem::size_of::<SanMove>());
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
                    SanMove::Move(SanMoveDetail {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(|(s, _)| s).unwrap(),
                        from_file: None,
                        from_rank: None,
                        check: Check::None,
                    }),
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
                    SanMove::Move(SanMoveDetail {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(first).unwrap(),
                        from_file: File::parse($pawn_file).map(first).ok(),
                        from_rank: None,
                        check: Check::None,
                    }),
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
                    SanMove::Move(SanMoveDetail {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(first).unwrap(),
                        from_file: None,
                        from_rank: None,
                        check: $check,
                    }),
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

    macro_rules! assert_disambiguate {
        ($piece:literal, $file:expr, $rank:expr, $square:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove::Move(SanMoveDetail {
                        piece: Piece::parse($piece).map(first).unwrap(),
                        destination: Square::parse($square).map(first).unwrap(),
                        from_file: $file,
                        from_rank: $rank,
                        check: Check::None,
                    }),
                    ""
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_disambiguate() {
        assert_disambiguate!("R", File::from_letter("h"), None, "e8", "Rhe8");
        assert_disambiguate!("N", None, Rank::from_number("3"), "e1", "N3e1");
        assert_disambiguate!(
            "B",
            File::from_letter("a"),
            Rank::from_number("3"),
            "c5",
            "Ba3c5"
        );
    }

    #[test]
    fn test_castle() {
        assert_eq!(
            (SanMove::LongCastle(Check::None), "TAIL"),
            SanMove::parse("O-O-OTAIL").unwrap()
        );
        assert_eq!(
            (SanMove::ShortCastle(Check::None), " SPACE"),
            SanMove::parse("O-O SPACE").unwrap()
        );

        assert_eq!(
            (SanMove::LongCastle(Check::Check), ""),
            SanMove::parse("O-O-O+").unwrap()
        );
        assert_eq!(
            (SanMove::ShortCastle(Check::Mate), ""),
            SanMove::parse("O-O#").unwrap()
        );
    }
}
