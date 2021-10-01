use crate::pgn_parser::GrammarNode;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Rank(pub u8);

impl Rank {
    pub fn from_number(s: &str) -> Option<Rank> {
        if let Some(ch) = s.chars().next() {
            if !('1'..='8').contains(&ch) {
                None
            } else {
                Some(Rank((ch as u8) - b'1' + 1))
            }
        } else {
            None
        }
    }
}

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl From<char> for Rank {
    fn from(ch: char) -> Self {
        Rank(ch as u8 - b'1' + 1)
    }
}

impl GrammarNode for Rank {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ('1'..='8').contains(&ch))
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // unwrap: unwrap okay, because check_start found a character.
        Ok((Rank::from(s.chars().next().unwrap()), &s[1..]))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::assert_with_tail;

    #[test]
    fn test_start() {
        assert!(Rank::check_start("1"));
        assert!(Rank::check_start("8"));
        assert!(!Rank::check_start("0"));
        assert!(!Rank::check_start("9"));
    }

    #[test]
    fn test_parse() {
        assert_with_tail!(Rank::from('1'), "TAIL", Rank::parse("1TAIL"));
        assert_with_tail!(Rank::from('8'), " SPACE", Rank::parse("8 SPACE"));
    }
}
