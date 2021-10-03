use crate::pgn_error::PgnError;
use crate::pgn_parser::symbol::Symbol;
use crate::pgn_parser::GrammarNode;
use crate::Result;

#[derive(Debug, Eq, PartialEq)]
pub struct TagPair {
    name: String,
    value: String,
}

fn parse_char(s: &str, ch: char) -> Result<&str> {
    if let Some(next_ch) = s.chars().next() {
        if ch == next_ch {
            Ok(&s[1..])
        } else {
            Err(PgnError::UnmatchedChar("parse_char", next_ch))
        }
    } else {
        Err(PgnError::UnexpectedEOF("parse_char"))
    }
}

/*
 A string token is a sequence of zero or more printing characters delimited by a pair of quote
 characters (ASCII decimal value 34, hexadecimal value 0x22). An empty string is represented by
 two adjacent quotes. (Note: an apostrophe is not a quote.) A quote inside a string is represented
 by the backslash immediately followed by a quote. A backslash inside a string is represented by
 two adjacent backslashes. Strings are commonly used as tag pair values (see below). Non-printing
 characters like newline and tab are not permitted inside of strings. A string token is terminated
 by its closing quote. Currently, a string is limited to a maximum of 255 characters of data.
*/
fn parse_pgn_string(s: &str) -> Result<(String, &str)> {
    let s: &str = parse_char(s, '"')?;

    let mut output = String::new();
    let mut escaping = false;
    let mut remaining = s;
    for ch in s.chars() {
        match escaping {
            false => {
                if ch == '"' {
                    break;
                }
                remaining = &remaining[1..];

                if ch == '\\' {
                    escaping = true;
                    continue;
                }

                output.push(ch);
            }
            true => {
                output.push(ch);
                remaining = &remaining[1..];
                escaping = false;
            }
        }
    }

    let s = parse_char(remaining, '"')?;

    Ok((output, s))
}

fn parse_tag_name(s: &str) -> Result<(String, &str)> {
    let (symbol, s) = Symbol::parse(s)?;
    Ok((symbol.into(), s))
}

fn parse_tag_value(s: &str) -> Result<(String, &str)> {
    parse_pgn_string(s)
}

/*
 <tag-pair> ::= [ <tag-name> <tag-value> ]
*/
impl GrammarNode for TagPair {
    fn check_start(s: &str) -> bool {
        s.starts_with('[')
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let s = parse_char(s, '[')?;

        let (name, s) = parse_tag_name(s)?;
        let s = s.trim_start();
        let (value, s) = parse_tag_value(s)?;

        let s = parse_char(s, ']')?;

        Ok((TagPair { name, value }, s))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tag_pair() {
        let (pair, tail) = TagPair::parse(r#"[Tag-Name "Tag Value"]TAIL"#).unwrap();
        assert_eq!("TAIL", tail);
        assert_eq!("Tag-Name", pair.name);
        assert_eq!("Tag Value", pair.value);

        let (pair, tail) = TagPair::parse(r#"[Escaped "Has a \\ and a \"."]TAIL"#).unwrap();
        assert_eq!("TAIL", tail);
        assert_eq!("Escaped", pair.name);
        assert_eq!(r#"Has a \ and a "."#, pair.value);
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            ("foobar".to_string(), ""),
            parse_pgn_string("\"foobar\"").unwrap()
        );
        assert_eq!(
            ("quux".to_string(), "TAIL"),
            parse_pgn_string("\"quux\"TAIL").unwrap()
        );
        assert_eq!(
            ("foo\"bar".to_string(), "TAIL"),
            parse_pgn_string("\"foo\\\"bar\"TAIL").unwrap()
        );
        assert_eq!(
            ("back\\slash".to_string(), "TAIL"),
            parse_pgn_string("\"back\\\\slash\"TAIL").unwrap()
        )
    }
}
