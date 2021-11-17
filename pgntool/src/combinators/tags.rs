use crate::combinators::{string_matcher, symbol_matcher, Token};
use chumsky::prelude::*;

/*
<tag-section> ::= <tag-pair> <tag-section>
                  <empty>
<tag-pair> ::= [ <tag-name> <tag-value> ]
<tag-name> ::= <identifier>
<tag-value> ::= <string>
*/

pub struct TagSection {
    pairs: Vec<TagPair>,
}

pub struct TagPair {
    symbol: String,
    value: String,
}

pub fn tag_section_matcher() -> impl Parser<char, TagSection, Error = Simple<char>> {
    tag_pair_matcher()
        .repeated()
        .map(|pairs| TagSection { pairs })
        .labelled("TAG SECTION")
}

pub fn tag_pair_matcher() -> impl Parser<char, TagPair, Error = Simple<char>> {
    tag_name_matcher()
        .then(tag_value_matcher())
        .delimited_by('[', ']')
        .map(|(symbol, value)| TagPair {
            symbol: symbol.into(),
            value: value.into(),
        })
        .labelled("TAG PAIR")
}

pub fn tag_name_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    symbol_matcher().labelled("TAG NAME")
}

pub fn tag_value_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    string_matcher().labelled("TAG VALUE")
}

#[cfg(test)]
mod test {
    use super::*;
}
