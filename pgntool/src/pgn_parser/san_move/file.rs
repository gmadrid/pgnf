use crate::pgn_error::PgnError::UnmatchedChar;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct File(pub u8);

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", (b'a' + self.0 - 1) as char)
    }
}

impl TryFrom<char> for File {
    type Error = PgnError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if (('a'..='h').contains(&ch)) {
            Ok(File(ch as u8 - b'a' + 1))
        } else {
            Err(UnmatchedChar("File", ch))
        }
    }
}

impl GrammarNode for File {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ('a'..='h').contains(&ch))
    }

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // unwrap: unwrap okay, because check_start found a valid character.
        Ok((File::try_from(s.chars().next().unwrap()).unwrap(), &s[1..]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_with_tail;

    #[test]
    fn test_start() {
        assert!(File::check_start("a"));
        assert!(File::check_start("h"));
        assert!(!File::check_start(" "));
        assert!(!File::check_start("i"));
    }

    #[test]
    fn test_parse() {
        assert_with_tail!(File::try_from('a').unwrap(), "TAIL", File::parse("aTAIL"));
        assert_with_tail!(
            File::try_from('h').unwrap(),
            " SPACE",
            File::parse("h SPACE")
        );
    }
}
