/*
   <movetext-section> ::= <element-sequence> <game-termination>
   <element-sequence> ::= <element> <element-sequence>
                          '(' <element-sequence> ')' <element-sequence>
                          <empty>


*/

use crate::combinators::ElementP::MoveNumber;
use crate::combinators::{
    element_p_matcher, integer_matcher, nag_matcher, symbol_matcher, ElementP, GameTermination,
};
use chumsky::prelude::*;
use chumsky::text::whitespace;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MovetextSection {
    sequence: ElementSequence,
    termination: GameTermination,
}

fn movetext_section_matcher() -> impl Parser<char, MovetextSection, Error = Simple<char>> {
    element_sequence_matcher().try_map(|mut sequence, span| {
        // TODO: need to check for GameTermination which is not at the end of the list.
        if let Some(SequenceMember::Elem(ElementP::Termination(term))) = sequence.members.last() {
            // unwrap: okay because we just checked it in the enclosing if/let.
            let term_clone = term.clone();
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
            .map(|es| SequenceMember::Recursion(es))
            .or(element_p_matcher().map(|el| SequenceMember::Elem(el)))
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
