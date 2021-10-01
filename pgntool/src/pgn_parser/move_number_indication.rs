use crate::pgn_parser::GrammarNode;
use itertools::put_back;

#[derive(Debug, Eq, PartialEq)]
pub struct MoveNumberIndication {
    number: u16,
}

/*
  8.2.2: Movetext move number indications

  A move number indication is composed of one or more adjacent digits (an integer token) followed
  by zero or more periods. The integer portion of the indication gives the move number of the
  immediately following white move (if present) and also the immediately following black move
  (if present).
*/
impl GrammarNode for MoveNumberIndication {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ch.is_ascii_digit())
    }

    fn parse(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        let mut iter = put_back(s.chars().enumerate());

        let (num_part, first_non_num): (&str, _) =
            if let Some((idx, ch)) = iter.find(|(idx, ch)| !ch.is_ascii_digit()) {
                iter.put_back((idx, ch));
                (&s[..idx], idx)
            } else {
                (s, s.len())
            };

        // TODO: deal with spaces between number and periods.
        // TODO: deal with spaces between periods?
        // TODO: clean up input?

        let first_non_period = if let Some((idx, _)) = iter.find(|(_, ch)| !(*ch == '.')) {
            idx
        } else {
            s.len()
        };

        Ok((
            MoveNumberIndication {
                number: num_part.parse()?,
            },
            &s[first_non_period..],
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pgn_parser::element::Element::Move;

    macro_rules! mni_assert {
        ($num:literal, $tail:literal, $s:expr) => {
            assert_eq!(
                (MoveNumberIndication { number: $num }, $tail),
                MoveNumberIndication::parse($s).unwrap()
            )
        };
    }

    #[test]
    fn test_basic() {
        mni_assert!(3, "ONEDIGIT", "3.ONEDIGIT");
        mni_assert!(34, "TWODIGIT", "34.TWODIGIT");
        mni_assert!(48, "MANYDOTS", "48....MANYDOTS");
        mni_assert!(64, "NODOT", "64NODOT");
    }

    //    #[test]
    fn test_spaces() {
        // TODO: make this work.
        mni_assert!(56, " WITHSPACE", "56.... WITHSPACE");
        mni_assert!(65, " NODOTWITHSPACE", "64 NODOTWITHSPACE");
    }
}
