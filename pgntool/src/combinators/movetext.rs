/*
   <movetext-section> ::= <element-sequence> <game-termination>
   <element-sequence> ::= <element> <element-sequence>
                          '(' <element-sequence> ')' <element-sequence>
                          <empty>
*/

use crate::combinators::{element_p_matcher, ElementP, GameTermination};
use chumsky::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MovetextSection {
    sequence: ElementSequence,
    termination: GameTermination,
}

impl MovetextSection {
    pub fn is_empty(&self) -> bool {
        self.sequence.members.is_empty()
    }
}

pub fn movetext_section_matcher() -> impl Parser<char, MovetextSection, Error = Simple<char>> {
    element_sequence_matcher().try_map(|mut sequence, _span| {
        // TODO: need to check for GameTermination which is not at the end of the list.
        if let Some(SequenceMember::Elem(ElementP::Termination(term))) = sequence.members.last() {
            let term_clone = *term;
            sequence.members.pop();
            Ok(MovetextSection {
                sequence,
                termination: term_clone,
            })
        } else {
            // Game terminator is missing, but we'll be generous and insert it for you.
            Ok(MovetextSection {
                sequence,
                termination: GameTermination::Unterminated,
            })
        }
    })
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ElementSequence {
    members: Vec<SequenceMember>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum SequenceMember {
    Elem(ElementP),
    Recursion(ElementSequence),
}

fn element_sequence_matcher() -> impl Parser<char, ElementSequence, Error = Simple<char>> {
    recursive(|matcher| {
        matcher
            .delimited_by('(', ')')
            .map(SequenceMember::Recursion)
            .or(element_p_matcher().map(SequenceMember::Elem))
            .padded()
            .repeated()
            .collect::<Vec<SequenceMember>>()
            .map(|v| ElementSequence { members: v })
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::combinators::GameTermination;

    #[test]
    fn test_movetext_section_matcher() {
        let matcher = movetext_section_matcher();

        assert_eq!(
            matcher.parse("1. e5 1-0").unwrap(),
            MovetextSection {
                sequence: ElementSequence {
                    members: vec![
                        SequenceMember::Elem(ElementP::MoveNumber(1)),
                        SequenceMember::Elem(ElementP::SanMove("e5".to_string())),
                    ]
                },
                termination: GameTermination::White
            }
        )
    }

    macro_rules! qseq {
        ($($v:expr),+) => { ElementSequence { members: vec![ $(SequenceMember::Elem($v.to_element()),)+ ] } };
    }

    trait ToEl {
        fn to_element(&self) -> ElementP;
    }

    impl ToEl for i32 {
        fn to_element(&self) -> ElementP {
            ElementP::MoveNumber(*self)
        }
    }

    impl ToEl for &str {
        fn to_element(&self) -> ElementP {
            ElementP::SanMove(self.to_string())
        }
    }

    #[test]
    fn test_short_sequence() {
        let matcher = element_sequence_matcher();

        assert_eq!(
            matcher.parse("1. e4 c6 2. d4 d5 3. Nc3 dxe4").unwrap(),
            qseq!(1, "e4", "c6", 2, "d4", "d5", 3, "Nc3", "dxe4")
        );
    }

    #[test]
    fn test_short_sequence_with_newline() {
        let matcher = element_sequence_matcher();

        // TODO: make a macro to make this feasible.
        assert_eq!(matcher.parse(r#"1. e4 c6 2. d4 d5 3. Nc3 dxe4 4. Nxe4 Nf6 5. Bd3 Nbd7 6. Nxf6+ Nxf6 7. Bf4 e6 8.
    c3 Bd6 9. Bxd6 Qxd6 "#).unwrap(),
                qseq!(1, "e4", "c6", 2, "d4", "d5", 3, "Nc3", "dxe4", 4, "Nxe4", "Nf6",
                5, "Bd3", "Nbd7", 6, "Nxf6+", "Nxf6", 7, "Bf4", "e6",
                8, "c3", "Bd6", 9, "Bxd6", "Qxd6")
            );
    }

    #[test]
    fn test_element_sequence_matcher() {
        let matcher = element_sequence_matcher();

        assert_eq!(
            matcher.parse("1. e5").unwrap(),
            ElementSequence {
                members: vec![
                    SequenceMember::Elem(ElementP::MoveNumber(1)),
                    SequenceMember::Elem(ElementP::SanMove("e5".to_string()))
                ]
            }
        );
        assert_eq!(
            matcher.parse("(1. e5)").unwrap(),
            ElementSequence {
                members: vec![SequenceMember::Recursion(ElementSequence {
                    members: vec![
                        SequenceMember::Elem(ElementP::MoveNumber(1)),
                        SequenceMember::Elem(ElementP::SanMove("e5".to_string()))
                    ]
                })]
            }
        );
        // Another test with spaces around the recursion.
        assert_eq!(
            matcher.parse("( 1. e5 )").unwrap(),
            ElementSequence {
                members: vec![SequenceMember::Recursion(ElementSequence {
                    members: vec![
                        SequenceMember::Elem(ElementP::MoveNumber(1)),
                        SequenceMember::Elem(ElementP::SanMove("e5".to_string()))
                    ]
                })]
            }
        );
    }
}
