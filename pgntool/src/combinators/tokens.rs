use chumsky::error::Simple;
use chumsky::prelude::*;
use std::str::FromStr;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Token {
    LAngle,
    RAngle,

    LBrack,
    RBrack,

    LParen,
    RParen,

    LCurly,
    RCurly,

    Period,
    Star,

    Ident(String),
    Integer(i32),
    NAG(i32),
    Str(String),
}

impl From<Token> for String {
    fn from(token: Token) -> Self {
        match token {
            Token::Ident(s) => s,
            Token::Str(s) => s,
            _ => panic!("String value requested from invalid token"),
        }
    }
}

impl From<Token> for i32 {
    fn from(token: Token) -> Self {
        match token {
            Token::Integer(num) => num,
            Token::NAG(num) => num,
            _ => panic!("Integer value requested from invalid token"),
        }
    }
}

fn char_matcher(ch: char, f: impl Fn() -> Token) -> impl Parser<char, Token, Error = Simple<char>> {
    just::<char, Simple<char>>(ch).map(move |_| f())
}

macro_rules! matched_char {
    ($name: ident, $c: expr, $t: expr) => {
        pub fn $name() -> impl Parser<char, Token, Error = Simple<char>> {
            char_matcher($c, || $t)
        }
    };
}

matched_char!(langle_matcher, '<', Token::LAngle);
matched_char!(rangle_matcher, '>', Token::RAngle);
matched_char!(lbrack_matcher, '[', Token::LBrack);
matched_char!(rbrack_matcher, ']', Token::RBrack);
matched_char!(lparen_matcher, '(', Token::LParen);
matched_char!(rparen_matcher, ')', Token::RParen);
matched_char!(lcurly_matcher, '{', Token::LCurly);
matched_char!(rcurly_matcher, '}', Token::RCurly);
matched_char!(period_matcher, '.', Token::Period);
matched_char!(star_matcher, '*', Token::Star);

fn is_symbol_character(ch: &char) -> bool {
    ch.is_ascii_alphanumeric()
        || *ch == '_'
        || *ch == '+'
        || *ch == '#'
        || *ch == '='
        || *ch == ':'
        || *ch == '-'
}

pub fn symbol_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_alphanumeric())
        .then(filter(is_symbol_character).repeated())
        .map(|(ch, vec)| format!("{}{}", ch, vec.into_iter().collect::<String>()))
        .map(Token::Ident)
}

pub fn nag_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    just('$')
        .ignore_then(filter(|c: &char| c.is_ascii_digit()).repeated().at_least(1))
        .collect()
        .try_map(|s: String, span| {
            i32::from_str(&s).map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .map(Token::NAG)
}

pub fn integer_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .collect()
        .try_map(|s: String, span| {
            i32::from_str(&s).map_err(|e| Simple::custom(span, format!("{}", e)))
        })
        .map(Token::Integer)
}

pub fn string_matcher() -> impl Parser<char, Token, Error = Simple<char>> {
    // TODO: Restrict '\' escaping to \n and \\.
    // TODO: don't allow non-printables in a String.
    // TODO: limit to 255 chars
    filter(|c| (*c != '"' && *c != '\\'))
        .or(just('\\').ignore_then(any()))
        .repeated()
        .delimited_by('"', '"')
        .collect()
        .map(|s| Token::Str(s))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_string() {
        let matcher = string_matcher();

        assert_eq!(
            matcher.parse(r#""Hello""#),
            Ok(Token::Str("Hello".to_string()))
        );
        assert_eq!(
            matcher.parse(r#""Hel\"lo""#),
            Ok(Token::Str(r#"Hel"lo"#.to_string()))
        );
        assert_eq!(
            matcher.parse(r#""Hell\\o""#),
            Ok(Token::Str(r#"Hell\o"#.to_string()))
        );
        assert_eq!(matcher.parse(r#""""#), Ok(Token::Str("".to_string())));
    }

    #[test]
    fn test_integer() {
        let matcher = integer_matcher();

        assert_eq!(matcher.parse("123"), Ok(Token::Integer(123)));
    }

    #[test]
    fn test_langle() {
        let matcher = langle_matcher();

        assert_eq!(matcher.parse("<").unwrap(), Token::LAngle);
        assert!(matcher.parse("]").is_err());
    }

    #[test]
    fn test_rangle() {
        let matcher = rangle_matcher();

        assert_eq!(matcher.parse(">").unwrap(), Token::RAngle);
        assert!(matcher.parse("[").is_err());
    }

    #[test]
    fn test_lbrack() {
        let matcher = lbrack_matcher();

        assert_eq!(matcher.parse("[").unwrap(), Token::LBrack);
        assert!(matcher.parse("]").is_err());
    }

    #[test]
    fn test_rbrack() {
        let matcher = rbrack_matcher();

        assert_eq!(matcher.parse("]").unwrap(), Token::RBrack);
        assert!(matcher.parse("[").is_err());
    }

    #[test]
    fn test_lparen() {
        let matcher = lparen_matcher();

        assert_eq!(matcher.parse("(").unwrap(), Token::LParen);
        assert!(matcher.parse("@").is_err());
    }

    #[test]
    fn test_rparen() {
        let matcher = rparen_matcher();

        assert_eq!(matcher.parse(")").unwrap(), Token::RParen);
        assert!(matcher.parse("@").is_err());
    }

    #[test]
    fn test_lcurly() {
        let matcher = lcurly_matcher();

        assert_eq!(matcher.parse("{").unwrap(), Token::LCurly);
        assert!(matcher.parse("@").is_err());
    }

    #[test]
    fn test_rcurly() {
        let matcher = rcurly_matcher();

        assert_eq!(matcher.parse("}").unwrap(), Token::RCurly);
        assert!(matcher.parse("@").is_err());
    }

    #[test]
    fn test_nag() {
        let matcher = nag_matcher();

        assert_eq!(matcher.parse("$32"), Ok(Token::NAG(32)))
    }

    #[test]
    fn test_period() {
        let matcher = period_matcher();

        assert_eq!(matcher.parse(".").unwrap(), Token::Period);
        assert!(matcher.parse("[").is_err());
    }

    #[test]
    fn test_star() {
        let matcher = star_matcher();

        assert_eq!(matcher.parse("*").unwrap(), Token::Star);
        assert!(matcher.parse("[").is_err());
    }

    #[test]
    fn test_symbol() {
        let matcher = symbol_matcher();

        assert_eq!(matcher.parse("one"), Ok(Token::Ident("one".to_string())))
    }
}
