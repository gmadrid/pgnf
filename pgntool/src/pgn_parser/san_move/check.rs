use crate::pgn_parser::GrammarNode;
use crate::pgn_error::PgnError;

#[derive(Debug, Eq, PartialEq)]
pub enum Check {
    Check,
    Mate,
    None,
}

impl GrammarNode for Check {
    fn check_start(s: &str) -> bool {
        let chars: &[char] = &['+', '#'];
        s.starts_with(chars)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        match s.chars().next() {
            Some('+') => Ok((Check::Check, &s[1..])),
            Some('#') => Ok((Check::Mate, &s[1..])),
            Some(ch) => Err(PgnError::InvalidCheckChar(ch)),
            None => panic!("did you call check_start()?"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_with_tail;

    #[test]
    fn test_start() {
        assert!(Check::check_start("+"));
        assert!(Check::check_start("#"));
        assert!(!Check::check_start("$"));
        assert!(!Check::check_start("Z"));
        assert!(!Check::check_start(""));
    }

    #[test]
    fn test_parse() {
        assert_with_tail!(Check::Check, "TAIL", Check::parse("+TAIL"));
        assert_with_tail!(Check::Mate, " SPACE", Check::parse("# SPACE"));
    }
}
