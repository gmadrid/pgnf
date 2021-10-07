use file::File;
use piece::Piece;
use rank::Rank;
use square::Square;
use toolpack::trytools::if_some;

use crate::pgn_error::PgnError;
use crate::pgn_error::PgnError::UnexpectedInput;
use crate::pgn_parser::san_move::capture::Capture;
use crate::pgn_parser::san_move::check::Check;
use crate::pgn_parser::san_move::piecespec::PieceSpec;
use crate::pgn_parser::GrammarNode;
use crate::pgn_parser::san_move::promotion::Promotion;

mod capture;
mod check;
mod disambiguation;
mod file;
mod piece;
mod piecespec;
mod rank;
mod square;
mod promotion;

#[derive(Debug, Eq, PartialEq)]
pub struct SanMove {
    move_type: SanMoveType,
    check: Check,
}

#[derive(Debug, Eq, PartialEq)]
pub enum SanMoveType {
    Move(SanMoveDetail),
    LongCastle,
    ShortCastle,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SanMoveDetail {
    piece: Piece,
    destination: Square,
    from_file: Option<File>,
    from_rank: Option<Rank>,
    capture: bool,
    promote: Option<Piece>,
}

impl SanMove {
    fn parse_castle(s: &str) -> crate::Result<(SanMoveType, Check, &str)> {
        // Check for Long Castle first because short castle is a prefix of long castle.
        if let Some(s) = s.strip_prefix("O-O-O") {
            let (check, s) = Check::parse(s).unwrap_or((Check::None, s));
            Ok((SanMoveType::LongCastle, check, s))
        } else if let Some(s) = s.strip_prefix("O-O") {
            let (check, s) = Check::parse(s).unwrap_or((Check::None, s));
            Ok((SanMoveType::ShortCastle, check, s))
        } else {
            Err(PgnError::UnexpectedInput("Castle", s.to_string()))
        }
    }
}

/*
  8.2.3: Movetext SAN (Standard Algebraic Notation)

  See reference document. The description is too long to include here.

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
    SANMOVE ::= <PIECESPEC><CAPTURE><DESTINATION><PROMOTION><CHECK>
    PIECESPEC ::= <PIECE><DISAMBIGUATION>
              ::= <empty>   // implied pawn
    CAPTURE ::= 'x'
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
    PIECE := [PNBRQK]?
    FILE ::= [a-h]
    RANK ::= [1-8]
    SQUARE ::= <RANK><FILE>
    DESTINATION ::= SQUARE
    PROMOTION ::= '=' <PIECE>
              ::= <empty>
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
        if let Ok((castle, check, remaining)) = SanMove::parse_castle(s) {
            return Ok((
                SanMove {
                    move_type: castle,
                    check,
                },
                remaining,
            ));
        }

        let (piecespec, s) = PieceSpec::parse(s).unwrap_or((PieceSpec::pawn(), s));

        let (capture, s) = if_some(Capture::check_start(s))
            .and_then(|_| Capture::parse(s).ok())
            .map(|(_, remaining)| (true, remaining))
            .unwrap_or((false, s));

        let (destination, s) = if Square::check_start(s) {
            Square::parse(s)?
        } else {
            return Err(UnexpectedInput("Destination square", s.to_string()));
        };

        let (promotion, s) = if Promotion::check_start(s) {
            Promotion::parse(s).map(|(p, s)| (Some(Piece::from(p)), s))?
        } else {
            (None, s)
        };

        let (check, s) = if_some(Check::check_start(s))
            .and_then(|_| Check::parse(s).ok())
            .unwrap_or((Check::None, s));

        let from_file = piecespec.disambiguation.file();
        let from_rank = piecespec.disambiguation.rank();

        let detail = SanMoveDetail {
            piece: piecespec.piece,
            destination,
            from_file,
            from_rank,
            capture,
            promote: promotion.into(),
        };

        Ok((
            SanMove {
                move_type: SanMoveType::Move(detail),
                check,
            },
            s,
        ))
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
                        rank: Rank::try_from($rank).unwrap(),
                        file: File::try_from($file).unwrap(),
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
    use std::convert::TryFrom;
    use toolpack::tupl::first;

    #[test]
    fn test_size() {
        assert_eq!(10, std::mem::size_of::<SanMove>());
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
                        check: Check::None,
                        move_type: SanMoveType::Move(SanMoveDetail {
                            piece: Piece::parse($piece).map(|(s, _)| s).unwrap(),
                            destination: Square::parse($square).map(|(s, _)| s).unwrap(),
                            from_file: None,
                            from_rank: None,
                            capture: false,
                            promote: None,
                        })
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
        ($piece:literal, $square:literal, $pawn_file:expr, $tail:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        move_type: SanMoveType::Move(SanMoveDetail {
                            piece: Piece::parse($piece).map(|(p, _)| p).unwrap(),
                            destination: Square::parse($square).map(|p| *first(&p)).unwrap(),
                            from_file: $pawn_file,
                            from_rank: None,
                            capture: true,
                            promote: None,
                        }),
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
        assert_capture!(
            "P",
            "e5",
            Some(File::try_from('d').unwrap()),
            "TAIL",
            "dxe5TAIL"
        );
        assert_capture!("Q", "f6", None, " SPACE", "Qxf6 SPACE");
    }

    macro_rules! assert_check {
        ($piece:literal, $square:literal, $check:expr, $tail:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        move_type: SanMoveType::Move(SanMoveDetail {
                            piece: Piece::parse($piece).map(|(p, _)| p).unwrap(),
                            destination: Square::parse($square).map(|p| *first(&p)).unwrap(),
                            from_file: None,
                            from_rank: None,
                            capture: false,
                            promote: None,
                        }),
                        check: $check
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

    macro_rules! assert_disambiguate {
        ($piece:literal, $file:expr, $rank:expr, $square:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        move_type: SanMoveType::Move(SanMoveDetail {
                            piece: Piece::parse($piece).map(|(p, _)| p).unwrap(),
                            destination: Square::parse($square).map(|p| *first(&p)).unwrap(),
                            from_file: $file,
                            from_rank: $rank,
                            capture: false,
                            promote: None,
                        }),
                        check: Check::None
                    },
                    ""
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_disambiguate() {
        assert_disambiguate!("R", File::try_from('h').ok(), None, "e8", "Rhe8");
        assert_disambiguate!("N", None, Rank::try_from('3').ok(), "e1", "N3e1");
        assert_disambiguate!(
            "B",
            File::try_from('a').ok(),
            Rank::try_from('3').ok(),
            "c5",
            "Ba3c5"
        );
    }

    #[test]
    fn test_castle() {
        assert_eq!(
            (
                SanMove {
                    move_type: SanMoveType::LongCastle,
                    check: Check::None
                },
                "TAIL"
            ),
            SanMove::parse("O-O-OTAIL").unwrap()
        );
        assert_eq!(
            (
                SanMove {
                    move_type: SanMoveType::ShortCastle,
                    check: Check::None
                },
                " SPACE"
            ),
            SanMove::parse("O-O SPACE").unwrap()
        );

        assert_eq!(
            (
                SanMove {
                    move_type: SanMoveType::LongCastle,
                    check: Check::Check
                },
                ""
            ),
            SanMove::parse("O-O-O+").unwrap()
        );
        assert_eq!(
            (
                SanMove {
                    move_type: SanMoveType::ShortCastle,
                    check: Check::Mate
                },
                ""
            ),
            SanMove::parse("O-O#").unwrap()
        );
    }

    macro_rules! assert_promotion {
        ($piece:literal, $square: literal, $capture:expr, $file:expr, $promo_piece:literal, $to_parse:literal) => {
            assert_eq!(
                (
                    SanMove {
                        move_type: SanMoveType::Move(SanMoveDetail {
                            piece: Piece::parse($piece).map(|(p, _)| p).unwrap(),
                            destination: Square::parse($square).map(|(p, _)| p).unwrap(),
                            from_file: $file,
                            from_rank: None,
                            capture: $capture,
                            promote: Piece::parse($promo_piece).map(|(p, _) | p).ok(),
                        }),
                        check: Check::None,
                    },
                    ""
                ),
                SanMove::parse($to_parse).unwrap()
            )
        };
    }

    #[test]
    fn test_promotion() {
        assert_promotion!("P", "g8", false, None, "Q", "g8=Q");
        assert_promotion!(
            "P",
            "e1",
            true,
            File::try_from('d').ok(),
            "N",
            "dxe1=N"
        );
    }
}
