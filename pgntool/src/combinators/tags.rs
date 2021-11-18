use crate::combinators::{ident_matcher, string_matcher, Ident, PgnStr};
use chumsky::prelude::*;

/*
    <tag-section> ::= <tag-pair> <tag-section>
                      <empty>
    <tag-pair> ::= [ <tag-name> <tag-value> ]
    <tag-name> ::= <identifier>
    <tag-value> ::= <string>
*/

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TagSection {
    pairs: Vec<TagPair>,
}

impl TagSection {
    pub fn pairs(&self) -> &Vec<TagPair> {
        &self.pairs
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TagPair {
    symbol: String,
    value: String,
}

pub fn tag_section_matcher() -> impl Parser<char, TagSection, Error = Simple<char>> {
    tag_pair_matcher()
        .padded()
        .repeated()
        .map(|pairs| TagSection { pairs })
        .labelled("TAG SECTION")
}

fn tag_pair_matcher() -> impl Parser<char, TagPair, Error = Simple<char>> {
    tag_name_matcher()
        .padded()
        .then(tag_value_matcher())
        .padded()
        .delimited_by('[', ']')
        .map(|(symbol, value)| TagPair {
            symbol: symbol.into(),
            value: value.into(),
        })
        .labelled("TAG PAIR")
}

fn tag_name_matcher() -> impl Parser<char, Ident, Error = Simple<char>> {
    ident_matcher().labelled("TAG NAME")
}

fn tag_value_matcher() -> impl Parser<char, PgnStr, Error = Simple<char>> {
    string_matcher().labelled("TAG VALUE")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_tag_section() {
        let matcher = tag_section_matcher();
        assert_eq!(matcher.parse("").unwrap(), TagSection { pairs: vec![] });
    }

    #[test]
    fn test_tag_section() {
        let matcher = tag_section_matcher();

        assert_eq!(
            matcher
                .parse(
                    r#"
        [Name "Bobby"]
        [ Place "Georgia" ]
        [Show "Breaking Bad"   ]
        "#
                    .trim()
                )
                .unwrap(),
            TagSection {
                pairs: vec![
                    TagPair {
                        symbol: "Name".to_string(),
                        value: "Bobby".to_string()
                    },
                    TagPair {
                        symbol: "Place".to_string(),
                        value: "Georgia".to_string()
                    },
                    TagPair {
                        symbol: "Show".to_string(),
                        value: "Breaking Bad".to_string()
                    },
                ]
            }
        );
    }

    #[test]
    fn test_tag_pairs() {
        let matcher = tag_pair_matcher();

        assert_eq!(
            matcher.parse(r#"[Name "Bobby"]"#).unwrap(),
            TagPair {
                symbol: "Name".to_string(),
                value: "Bobby".to_string()
            }
        );
        assert_eq!(
            matcher.parse(r#"[ Dogfood "Alpo" ]"#).unwrap(),
            TagPair {
                symbol: "Dogfood".to_string(),
                value: "Alpo".to_string()
            }
        );
    }
}
