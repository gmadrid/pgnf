use crate::pgn_parser::san_move::piece::Piece;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;

pub struct Promotion(Piece);

impl From<Promotion> for Piece {
    fn from(promotion: Promotion) -> Self {
        promotion.0
    }
}

impl GrammarNode for Promotion {
    fn check_start(s: &str) -> bool {
        s.starts_with('=')
    }

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // Skip the '='. (It's there because the caller should have called check_start().
        let s = &s[1..];

        if !Piece::check_start(s) {
            return Err(PgnError::UnexpectedInput("Promotion", s.to_string()));
        }

        let (piece, s) = Piece::parse(s)?;

        Ok((Promotion(piece), s))
    }
}
