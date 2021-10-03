use crate::pgn_parser::GrammarNode;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct File(pub u8);

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", (b'a' + self.0 - 1) as char)
    }
}

impl From<char> for File {
    fn from(ch: char) -> Self {
        // TODO: should I panic here? .
        File(ch as u8 - b'a' + 1)
    }
}

impl GrammarNode for File {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ('a'..='h').contains(&ch))
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // unwrap: unwrap okay, because check_start found a character.
        Ok((File::from(s.chars().next().unwrap()), &s[1..]))
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
        assert_with_tail!(File::from('a'), "TAIL", File::parse("aTAIL"));
        assert_with_tail!(File::from('h'), " SPACE", File::parse("h SPACE"));
    }

    #[test]
    fn test_from_letter() {
        assert_eq!(Some(File(1)), File::from_letter("a"));

        assert_eq!(None, File::from_letter(""));
        assert_eq!(None, File::from_letter("i"));
    }
}
