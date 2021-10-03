use crate::pgn_error::PgnError::UnexpectedInput;
use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub struct Capture;

/*
   CAPTURE ::= 'x'
           ::= <empty>
*/
impl GrammarNode for Capture {
    fn check_start(s: &str) -> bool {
        s.starts_with('x')
    }

    fn parse(s: &str) -> crate::Result<(Capture, &str)>
    where
        Self: Sized,
    {
        if !s.starts_with('x') {
            return Err(UnexpectedInput("Capture", s.to_string()));
        }

        // Skip the 'x'.
        Ok((Capture, &s[1..]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_capture {
        ($tail:literal, $parsed:expr) => {
            assert_eq!((Capture, $tail), $parsed.unwrap())
        };
    }

    #[test]
    fn test_start() {
        assert!(Capture::check_start("x"));
        assert!(!Capture::check_start(" x"));
    }

    #[test]
    fn test_parse() {
        assert_capture!("TAIL", Capture::parse("xTAIL"));
        assert_capture!(" SPACE", Capture::parse("x SPACE"));
    }
}
