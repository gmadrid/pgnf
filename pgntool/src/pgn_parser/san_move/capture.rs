use crate::pgn_parser::san_move::file::File;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;

#[derive(Debug, Eq, PartialEq)]
struct Capture {
    pawn_file: Option<File>,
}

/*
   CAPTURE ::= <PAWNFILE> 'x'
           ::= <empty>
*/
impl GrammarNode for Capture {
    fn check_start(s: &str) -> bool {
        s.starts_with('x') || File::check_start(s)
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let (pawn_file, s) = if File::check_start(s) {
            File::parse(s)
                .map(|(f, s)| (Some(f), s))
                .unwrap_or((None, s))
        } else {
            (None, s)
        };

        if !s.starts_with('x') {
            return Err(PgnError::UnmatchedInput("FIX THIS", s.to_string()));
        }

        // Skip the 'x'.
        Ok((Capture { pawn_file }, &s[1..]))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! assert_capture_with_tail {
        ($tail:literal, $parsed:expr) => {
            assert_eq!((Capture { pawn_file: None }, $tail), $parsed.unwrap())
        };

        ($file:literal, $tail:literal, $parsed:expr) => {
            assert_eq!(
                (
                    Capture {
                        pawn_file: Some(File::from($file))
                    },
                    $tail
                ),
                $parsed.unwrap()
            )
        };
    }

    #[test]
    fn test_start() {
        assert!(Capture::check_start("x"));
        assert!(Capture::check_start("dx"));

        assert!(!Capture::check_start("Dx"));
        assert!(!Capture::check_start("8x"));
        assert!(!Capture::check_start(" x"));
    }

    #[test]
    fn test_parse() {
        assert_capture_with_tail!("TAIL", Capture::parse("xTAIL"));
        assert_capture_with_tail!(" SPACE", Capture::parse("x SPACE"));

        assert_capture_with_tail!('d', "e5TAIL", Capture::parse("dxe5TAIL"));
        assert_capture_with_tail!('e', "f6 SPACE", Capture::parse("exf6 SPACE"))
    }
}
