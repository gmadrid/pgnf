use crate::pgn_error::PgnError;
use crate::pgn_parser::move_number_indication::MoveNumberIndication;
use crate::pgn_parser::numeric_annotation_glyph::NumericAnnotationGlyph;
use crate::pgn_parser::san_move::SanMove;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, PartialEq, Eq)]
pub enum Element {
    MoveNumber(MoveNumberIndication),
    Move(SanMove),
    Annotation(NumericAnnotationGlyph),
}

/*
  <element> ::= <move-number-indication>
            ::= <SAN-move>
            ::= <numeric-annotation-glyph>

*/
impl GrammarNode for Element {
    fn check_start(s: &str) -> bool {
        MoveNumberIndication::check_start(s)
            || SanMove::check_start(s)
            || NumericAnnotationGlyph::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (element, s) = if MoveNumberIndication::check_start(s) {
            let (mni, remaining) = MoveNumberIndication::parse(s)?;
            (Element::MoveNumber(mni), remaining)
        } else if SanMove::check_start(s) {
            let (sm, remaining) = SanMove::parse(s)?;
            (Element::Move(sm), remaining)
        } else if NumericAnnotationGlyph::check_start(s) {
            let (nag, remaining) = NumericAnnotationGlyph::parse(s)?;
            (Element::Annotation(nag), remaining)
        } else {
            return Err(PgnError::UnexpectedInput("Element", s.to_string()));
        };
        Ok((element, s))
    }
}
