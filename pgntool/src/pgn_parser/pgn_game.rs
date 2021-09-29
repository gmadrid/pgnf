use crate::pgn_parser::movetext_section::MovetextSection;
use crate::pgn_parser::tag_section::TagSection;
use crate::pgn_parser::GrammarNode;

#[derive(Debug)]
pub struct PgnGame {
    tag_section: TagSection,
    movetext_section: MovetextSection,
}

/*
  <PGN-game> ::= <tag-section> <movetext-section>
*/
impl GrammarNode for PgnGame {
    fn check_start(s: &str) -> bool {
        TagSection::check_start(s) || MovetextSection::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (tag_section, s) = if TagSection::check_start(s) {
            TagSection::parse(s)?
        } else {
            (TagSection::empty(), s)
        };

        let s = s.trim_start();

        //let (movetext_section, s) = MovetextSection::parse(s)?;

        Ok((
            PgnGame {
                tag_section,
                movetext_section: MovetextSection{},  // TODO: write this.
            },
            s,
        ))
    }
}
