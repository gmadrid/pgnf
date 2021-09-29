use crate::pgn_parser::tag_pair::TagPair;
use crate::pgn_parser::GrammarNode;

#[derive(Debug)]
pub struct TagSection {
    pairs: Vec<TagPair>,
}

impl TagSection {
    pub fn empty() -> Self {
        TagSection { pairs: vec![] }
    }
}

/*
 <tag-section> ::= <tag-pair> <tag-section>
                   <empty>
*/
impl GrammarNode for TagSection {
    fn check_start(s: &str) -> bool {
        TagPair::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let mut s = s;

        let mut pairs: Vec<TagPair> = Default::default();
        while TagPair::check_start(s) {
            let (pair, remainder) = TagPair::parse(s)?;
            s = remainder.trim_start();
            pairs.push(pair);
        }

        Ok((TagSection { pairs }, s))
    }
}
