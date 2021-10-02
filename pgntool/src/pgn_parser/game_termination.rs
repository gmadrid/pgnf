use crate::pgn_error::PgnError;
use crate::pgn_parser::GrammarNode;
use crate::Result;

/*
   <game-termination> ::= 1-0
                      ::= 0-1
                      ::= 1/2-1/2
                      ::= *
*/

#[derive(Debug, Eq, PartialEq)]
pub enum GameTermination {
    WhiteWins,
    BlackWins,
    Tie,
    Unfinished,
}

pub fn if_some_with<T>(pred: bool, f: impl FnOnce() -> T) -> Option<T> {
    if pred {
        Some(f())
    } else {
        None
    }
}

fn match_prefix<'a, T>(s: &'a str, pat: &str, t: T) -> Option<(T, &'a str)> {
    if_some_with(s.starts_with(pat), || (t, &s[pat.len()..]))
}

impl GrammarNode for GameTermination {
    fn check_start(s: &str) -> bool {
        let start_chars: &[_] = &['1', '0', '*'];
        s.starts_with(start_chars)
    }

    fn parse(s: &str) -> Result<(GameTermination, &str)> {
        match_prefix(s, "1-0", GameTermination::WhiteWins)
            .or_else(|| match_prefix(s, "0-1", GameTermination::BlackWins))
            .or_else(|| match_prefix(s, "1/2-1/2", GameTermination::Tie))
            .or_else(|| match_prefix(s, "*", GameTermination::Unfinished))
            .ok_or_else(|| PgnError::UnmatchedInput("GameTermination", s.to_string()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_start() {
        assert!(GameTermination::check_start("1-0TAIL"));
        assert!(GameTermination::check_start("1/2-1/2TAIL"));
        assert!(GameTermination::check_start("0-1TAIL"));
        assert!(GameTermination::check_start("*TAIL"));

        assert!(GameTermination::check_start("1111"));

        assert!(!GameTermination::check_start("NOPE"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            (GameTermination::WhiteWins, "TAIL"),
            GameTermination::parse("1-0TAIL").unwrap()
        );
        assert_eq!(
            (GameTermination::BlackWins, "TAIL"),
            GameTermination::parse("0-1TAIL").unwrap()
        );
        assert_eq!(
            (GameTermination::Tie, "TAIL"),
            GameTermination::parse("1/2-1/2TAIL").unwrap()
        );
        assert_eq!(
            (GameTermination::Unfinished, "TAIL"),
            GameTermination::parse("*TAIL").unwrap()
        );

        assert!(GameTermination::parse("INVALID").is_err());
    }
}
