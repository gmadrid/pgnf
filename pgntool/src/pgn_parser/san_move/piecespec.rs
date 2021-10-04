use crate::pgn_parser::san_move::disambiguation::Disambiguation;
use crate::pgn_parser::san_move::piece::Piece;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub struct PieceSpec {
    pub piece: Piece,
    pub disambiguation: Disambiguation,
}

impl PieceSpec {
    pub fn pawn() -> Self {
        PieceSpec {
            piece: Piece::Pawn,
            disambiguation: Disambiguation::None,
        }
    }
}

impl GrammarNode for PieceSpec {
    fn check_start(s: &str) -> bool {
        Piece::check_start(s) || Disambiguation::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (piece, s) = if Piece::check_start(s) {
            Piece::parse(s)?
        } else {
            (Piece::Pawn, s)
        };

        let (disambiguation, s) = if Disambiguation::check_start(s) {
            Disambiguation::parse(s).unwrap_or((Disambiguation::None, s))
        } else {
            (Disambiguation::None, s)
        };

        Ok((
            PieceSpec {
                piece,
                disambiguation,
            },
            s,
        ))
    }
}

#[cfg(test)]
mod test {
    macro_rules! assert_ps_with_tail {
        ($piece:literal, $tail:literal, $parsed:expr) => {};
    }

    /*
     Value to test with:

       a6     None    | Err | a6
       Qa6    Queen 'a' | Err | a6
       axb6
       Qxc8
       Qac8
       Naxc8
       N7xb5

       // TODO: write these.
    */

    #[test]
    fn test_start() {}
}
