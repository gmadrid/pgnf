use crate::pgn_parser::GrammarNode;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct File(pub u8);

impl Debug for File {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]",
               ('a' as u8 + self.0 - 1) as char)
    }
}

impl From<char> for File {
    fn from(ch: char) -> Self {
        File(ch as u8 - 'a' as u8 + 1)
    }
}

impl GrammarNode for File {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ch >= 'a' && ch <= 'h')
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
}
