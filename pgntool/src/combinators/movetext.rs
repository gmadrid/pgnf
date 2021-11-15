/*
    <movetext-section> ::= <element-sequence> <game-termination>
    <element-sequence> ::= <element> <element-sequence>
                           <recursive-variation> <element-sequence>
                           <empty>
    <element> ::= <move-number-indication>
                  <SAN-move>
                  <numeric-annotation-glyph>
    <recursive-variation> ::= ( <element-sequence> )
    <game-termination> ::= 1-0
                           0-1
                           1/2-1/2
                           *

 */

use chumsky::prelude::*;
use chumsky::text::whitespace;
use crate::combinators::{integer_matcher, nag_matcher, symbol_matcher};

pub struct ElementSequence {
    members: Vec<SequenceMember>,
}

enum SequenceMember {
    Elem(Element),
    Recursion(Vec<SequenceMember>),
}

fn element_sequence_matcher() -> impl Parser<char, ElementSequence, Error = Simple<char>> {
    element_matcher().map(|el| SequenceMember::Elem(el))
        .or(recursive_variation_matcher().map(|es| SequenceMember::Recursion(es.members)))
        .repeated()
        .collect::<Vec<SequenceMember>>()
        .map(|v| ElementSequence { members: v })
}

struct RecursiveVariation {
    sequence: ElementSequence
}

fn recursive_variation_matcher() -> impl Parser<char, ElementSequence, Error = Simple<char>> {
    element_sequence_matcher()
        .delimited_by('(', ')')
}

enum Element {
    MoveNumber(i32),
    SanMove(String),
    NAG(i32),
}

fn element_matcher() -> impl Parser<char, Element, Error = Simple<char>> {
    move_number_matcher().map(|mn| Element::MoveNumber(mn.move_number))
        .or(san_move_matcher().map(|sm| Element::SanMove(sm.san_move)))
        .or(nag_matcher().map(|nag| Element::NAG(nag.into())))
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct RawSanMove {
    san_move: String,
}

fn san_move_matcher() -> impl Parser<char, RawSanMove, Error = Simple<char>> {
    symbol_matcher()
        .map(|token| RawSanMove { san_move: token.into()} )
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct MoveNumber {
    move_number: i32
}

fn move_number_matcher() -> impl Parser<char, MoveNumber, Error = Simple<char>>{
    integer_matcher()
        .then_ignore(whitespace())
        .then_ignore(just('.').repeated())
        .map(|num| MoveNumber { move_number: num.into() })
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum GameTermination {
    White,
    Black,
    Tie,
    Unterminated,
}

fn game_termination_matcher() -> impl Parser<char, GameTermination, Error = Simple<char>> {
    seq("1-0".chars()).map(|_| GameTermination::White)
        .or(seq("0-1".chars()).map(|_| GameTermination::Black))
        .or(seq("1/2-1/2".chars()).map(|_| GameTermination::Tie))
        .or(seq("*".chars()).map(|_| GameTermination::Unterminated))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_san_move() {
        let matcher = san_move_matcher();

        assert_eq!(matcher.parse("e4").unwrap(), RawSanMove { san_move: "e4".to_string()});
    }

    #[test]
    fn test_move_number() {
        let matcher = move_number_matcher();

        assert_eq!(matcher.parse("1").unwrap(), MoveNumber { move_number: 1 });
        assert_eq!(matcher.parse("1.").unwrap(), MoveNumber { move_number: 1 });
        assert_eq!(matcher.parse("32...").unwrap(), MoveNumber { move_number: 32 });
        assert_eq!(matcher.parse("100 .").unwrap(), MoveNumber { move_number: 100 });
        assert_eq!(matcher.parse("1 ...").unwrap(), MoveNumber { move_number: 1 });
    }

    #[test]
    fn test_game_termination() {
        let matcher = game_termination_matcher();

        assert_eq!(matcher.parse("1-0").unwrap(), GameTermination::White);
        assert_eq!(matcher.parse("0-1").unwrap(), GameTermination::Black);
        assert_eq!(matcher.parse("1/2-1/2").unwrap(), GameTermination::Tie);
        assert_eq!(matcher.parse("*").unwrap(), GameTermination::Unterminated);
    }
}