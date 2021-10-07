use crate::pgn_error::PgnError::UnmatchedChar;
use crate::pgn_parser::GrammarNode;
use crate::PgnError;
use std::convert::TryFrom;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Rank(pub u8);

impl Debug for Rank {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

impl TryFrom<char> for Rank {
    type Error = PgnError;

    fn try_from(ch: char) -> Result<Self, Self::Error> {
        if ('1'..='8').contains(&ch) {
            Ok(Rank(ch as u8 - b'1' + 1))
        } else {
            Err(UnmatchedChar("Rank", ch))
        }
    }
}

impl GrammarNode for Rank {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ('1'..='8').contains(&ch))
    }

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        // unwrap: unwrap okay, because check_start found a valid character.
        Ok((Rank::try_from(s.chars().next().unwrap()).unwrap(), &s[1..]))
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
        assert_with_tail!(Rank::try_from('1').unwrap(), "TAIL", Rank::parse("1TAIL"));
        assert_with_tail!(
            Rank::try_from('8').unwrap(),
            " SPACE",
            Rank::parse("8 SPACE")
        );
    }
}
