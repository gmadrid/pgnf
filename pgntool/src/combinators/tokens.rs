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

fn is_printable(ch: &char) -> bool {
    (0x20u32..=0x7e).contains(&From::from(*ch))
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

pub fn escaped_char_matcher() -> impl Parser<char, char, Error = Simple<char>> {
    just('\\').or(just('"'))
}

pub fn string_matcher() -> impl Parser<char, PgnStr, Error = Simple<char>> {
    filter(|c| (*c != '"' && *c != '\\' && is_printable(c)))
        .or(just('\\').ignore_then(escaped_char_matcher()))
        .repeated()
        .delimited_by('"', '"')
        .collect()
        .try_map(|s: String, span| {
            if s.len() > 255 {
                Err(Simple::custom(span, "Strings must have length <= 255."))
            } else {
                Ok(PgnStr(s.to_string()))
            }
        })
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
        .ignore_then(chumsky::text::int(10))
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
    fn test_bad_strings() {
        let matcher = string_matcher();

        // non-printables are not allowed
        assert!(matcher.parse("\"foo\tbar\"").is_err());

        // Only '\' and '"' are allowed to be escaped.
        assert!(matcher.parse(r#""\"""#).is_ok());
        assert!(matcher.parse(r#""\\""#).is_ok());
        assert!(dbg!(matcher.parse(r#""\n""#)).is_err());
    }

    #[test]
    fn test_long_string() {
        let matcher = string_matcher();

        let test_255 = "\"123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456710012345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345672001234567890123456789012345678901234567890123456789012255\"";
        assert_eq!(257, test_255.len()); // +2 for the quotes

        let test_256 = "\"1234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567100123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890123456720012345678901234567890123456789012345678901234567890123256\"";
        assert_eq!(258, test_256.len()); // +2 for the quotes

        assert!(matcher.parse(test_255).is_ok());
        assert!(matcher.parse(test_256).is_err());
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
