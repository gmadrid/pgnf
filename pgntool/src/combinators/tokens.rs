use chumsky::error::Simple;
use chumsky::prelude::*;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Ident(String);

impl From<Ident> for String {
    fn from(ident: Ident) -> Self {
        ident.0
    }
}

fn is_ident_character(ch: &char) -> bool {
    ch.is_ascii_alphanumeric()
        || *ch == '_'
        || *ch == '+'
        || *ch == '#'
        || *ch == '='
        || *ch == ':'
        || *ch == '-'
}

pub fn ident_matcher() -> impl Parser<char, Ident, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_alphanumeric())
        .then(filter(is_ident_character).repeated())
        .map(|(ch, vec)| format!("{}{}", ch, vec.into_iter().collect::<String>()))
        .map(Ident)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct PgnStr(String);

impl From<PgnStr> for String {
    fn from(pgn_str: PgnStr) -> Self {
        pgn_str.0
    }
}

pub fn string_matcher() -> impl Parser<char, PgnStr, Error = Simple<char>> {
    // TODO: Restrict '\' escaping to \n and \\.
    // TODO: don't allow non-printables in a String.
    // TODO: limit to 255 chars
    filter(|c| (*c != '"' && *c != '\\'))
        .or(just('\\').ignore_then(any()))
        .repeated()
        .delimited_by('"', '"')
        .collect()
        .map(PgnStr)
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct Nag(i32);

impl From<Nag> for i32 {
    fn from(nag: Nag) -> Self {
        nag.0
    }
}

pub fn nag_matcher() -> impl Parser<char, Nag, Error = Simple<char>> {
    just('$')
        // TODO: replace this with text::int
        .ignore_then(filter(|c: &char| c.is_ascii_digit()).repeated().at_least(1))
        .collect()
        .try_map(|s: String, span| {
            i32::from_str(&s).map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .map(Nag)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let matcher = string_matcher();

        assert_eq!(matcher.parse(r#""Hello""#), Ok(PgnStr("Hello".to_string())));
        assert_eq!(
            matcher.parse(r#""Hel\"lo""#),
            Ok(PgnStr(r#"Hel"lo"#.to_string()))
        );
        assert_eq!(
            matcher.parse(r#""Hell\\o""#),
            Ok(PgnStr(r#"Hell\o"#.to_string()))
        );
        assert_eq!(matcher.parse(r#""""#), Ok(PgnStr("".to_string())));
    }

    #[test]
    fn test_nag() {
        let matcher = nag_matcher();

        assert_eq!(matcher.parse("$32"), Ok(Nag(32)))
    }

    #[test]
    fn test_symbol() {
        let matcher = ident_matcher();

        assert_eq!(matcher.parse("one"), Ok(Ident("one".to_string())))
    }
}
