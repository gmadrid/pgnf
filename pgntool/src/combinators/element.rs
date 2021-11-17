/*
    <element> ::= <move-number-indication>
                  <SAN-move>
                  <numeric-annotation-glyph>
    <game-termination> ::= "1-0"
                           "0-1"
                           "1/2-1/2"
                           "*"

 NOTE: This grammar is not LL(1) because both game-termination and element can begin with '1' or
       '0'. We have to transform the grammar to remove the ambiguity. We make game termination
       into an element, then we preserve the integrity of the grammar by checking for the
       game termination at the end (and only the end) of the element sequence.

 So:

   <element-prime> ::= <number> <element-suffix>
                       '*'                         -- game-termination, unterminated
                       <san-move>
                       <numeric-annotation-glyph>
   <element-suffix> ::= '.'*           -- move number
                        '-' <number>   -- game-termination, but check in parent for 0-1 or 1-0
                        "/2-1/2"       -- game-termination, tie game
*/

use crate::combinators::{nag_matcher, symbol_matcher};
use chumsky::prelude::*;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum ElementP {
    MoveNumber(i32),
    SanMove(String),
    NAG(i32),

    Termination(GameTermination),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum GameTermination {
    White,
    Black,
    Tie,
    Unterminated,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum ElementSuffix {
    MoveNumber,
    Termination(GameTermination),

    // For the case with a bad GameTermination suffix, e.g. "-2", we need to match _something_,
    // otherwise, the empty move number matcher will succeed and the production will not error.
    // We carry up the number parsed for use in error reporting at the parent production.
    // TODO: this may not behave correctly with this input: "-foobar".
    BadTermination(String),
}

pub fn element_p_matcher() -> impl Parser<char, ElementP, Error = Simple<char>> {
    /*
        <element-prime> ::= <number> <element-suffix>
                            '*'                         -- game-termination, unterminated
                            <san-move>
                            <numeric-annotation-glyph>
    */
    chumsky::text::int(10)
        .then(element_suffix_matcher())
        .try_map(|(num_str, suffix), span| {
            // TODO: check for consistency of both halves of the game termination.
            match suffix {
                ElementSuffix::Termination(gt) => {
                    validate(span, &num_str, &gt)?;
                    Ok(ElementP::Termination(gt))
                }
                // TODO: deal with parser errors.
                ElementSuffix::MoveNumber => {
                    Ok(ElementP::MoveNumber(num_str.parse().map_err(|e| {
                        Simple::custom(span, format!("Failed parsing a move number: {}", e))
                    })?))
                }
                // TODO: dael with errors.
                ElementSuffix::BadTermination(bad_str) => {
                    Err(Simple::custom(span, "bad termination"))
                }
            }
        })
        .or(just('*').map(|_| ElementP::Termination(GameTermination::Unterminated)))
        .or(symbol_matcher().map(|token| ElementP::SanMove(token.into())))
        .or(nag_matcher().map(|token| ElementP::NAG(token.into())))
}

fn validate(
    span: std::ops::Range<usize>,
    num_str: &str,
    gt: &GameTermination,
) -> Result<(), Simple<char>> {
    match gt {
        GameTermination::White => {
            if num_str != "1" {
                return Err(Simple::custom(
                    span,
                    format!("Invalid game termination after \"{}\"", num_str),
                ));
            }
        }
        GameTermination::Black => {
            if num_str != "0" {
                return Err(Simple::custom(
                    span,
                    format!("Invalid game termination after \"{}\"", num_str),
                ));
            }
        }
        GameTermination::Tie => {}
        GameTermination::Unterminated => {}
    }
    Ok(())
}

fn element_suffix_matcher() -> impl Parser<char, ElementSuffix, Error = Simple<char>> {
    seq("/2-1/2".chars())
        .map(|_| ElementSuffix::Termination(GameTermination::Tie))
        .or(just('-').ignore_then(chumsky::text::int(10)).try_map(
            |num, span: std::ops::Range<usize>| {
                let span_clone = span.clone();

                if let Ok(parsed_num) = num.parse::<i32>() {
                    match parsed_num {
                        0 => Ok(ElementSuffix::Termination(GameTermination::White)),
                        1 => Ok(ElementSuffix::Termination(GameTermination::Black)),
                        _ => Ok(ElementSuffix::BadTermination(num)),
                    }
                } else {
                    Ok(ElementSuffix::BadTermination(num))
                }
            },
        ))
        .or(just('.').repeated().map(|_| ElementSuffix::MoveNumber))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_element_prime() {
        let matcher = element_p_matcher();

        // TODO: add some funny failure cases here.
        assert_eq!(
            matcher.parse("1-0").unwrap(),
            ElementP::Termination(GameTermination::White)
        );
        assert_eq!(
            matcher.parse("0-1").unwrap(),
            ElementP::Termination(GameTermination::Black)
        );
        assert_eq!(
            matcher.parse("1/2-1/2").unwrap(),
            ElementP::Termination(GameTermination::Tie)
        );

        assert_eq!(
            matcher.parse("*").unwrap(),
            ElementP::Termination(GameTermination::Unterminated)
        );

        assert_eq!(matcher.parse("1").unwrap(), ElementP::MoveNumber(1));
        assert_eq!(matcher.parse("2.").unwrap(), ElementP::MoveNumber(2));
        assert_eq!(matcher.parse("23...").unwrap(), ElementP::MoveNumber(23));
        assert_eq!(matcher.parse("31 .").unwrap(), ElementP::MoveNumber(31));
        assert_eq!(matcher.parse("42 ...").unwrap(), ElementP::MoveNumber(42));

        assert_eq!(
            matcher.parse("e5").unwrap(),
            ElementP::SanMove("e5".to_string())
        );

        assert_eq!(matcher.parse("$32").unwrap(), ElementP::NAG(32))
    }

    #[test]
    fn test_bad_game_terminations() {
        let matcher = element_p_matcher();

        // Bad game terminations are still valid symbols, so they will be read as SanMove.
        // We trust the SanMove parser to find the error.
        assert!(matches!(matcher.parse("1-1"), Ok(ElementP::SanMove(_))));
        assert!(matches!(matcher.parse("0-0"), Ok(ElementP::SanMove(_))));
        assert!(matches!(matcher.parse("2-0"), Ok(ElementP::SanMove(_))));
        assert!(matches!(matcher.parse("2-1"), Ok(ElementP::SanMove(_))));

        assert!(matches!(
            dbg!(matcher.parse("0-2")),
            Ok(ElementP::SanMove(_))
        ));
        assert!(matches!(matcher.parse("1-2"), Ok(ElementP::SanMove(_))));
        assert!(matches!(matcher.parse("2-2"), Ok(ElementP::SanMove(_))));
    }

    #[test]
    fn test_element_suffix_matcher() {
        let matcher = element_suffix_matcher();

        assert_eq!(
            matcher.parse("/2-1/2").unwrap(),
            ElementSuffix::Termination(GameTermination::Tie)
        );

        assert_eq!(
            matcher.parse("-0").unwrap(),
            ElementSuffix::Termination(GameTermination::White)
        );
        assert_eq!(
            matcher.parse("-1").unwrap(),
            ElementSuffix::Termination(GameTermination::Black)
        );

        assert_eq!(
            matcher.parse("-2").unwrap(),
            ElementSuffix::BadTermination("2".to_string())
        );
        assert_eq!(
            matcher.parse("-32").unwrap(),
            ElementSuffix::BadTermination("32".to_string())
        );

        assert_eq!(matcher.parse(".").unwrap(), ElementSuffix::MoveNumber);
        assert_eq!(matcher.parse("..").unwrap(), ElementSuffix::MoveNumber);
        assert_eq!(matcher.parse("....").unwrap(), ElementSuffix::MoveNumber);
        assert_eq!(matcher.parse("").unwrap(), ElementSuffix::MoveNumber);
    }
}
