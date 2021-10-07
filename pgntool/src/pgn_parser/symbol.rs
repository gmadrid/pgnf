use crate::pgn_parser::GrammarNode;

#[derive(Debug, Eq, PartialEq)]
pub struct Symbol(String);

impl From<Symbol> for String {
    fn from(id: Symbol) -> Self {
        id.0
    }
}

impl AsRef<str> for Symbol {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn is_identifier_continuation(ch: char) -> bool {
    ch.is_ascii_alphanumeric()
        || ch == '_'
        || ch == '+'
        || ch == '#'
        || ch == '='
        || ch == ':'
        || ch == '-'
}

/*
 A symbol token starts with a letter or digit character and is immediately followed by a sequence
 of zero or more symbol continuation characters. These continuation characters are letter characters
 ("A-Za-z"), digit characters ("0-9"), the underscore ("_"), the plus sign ("+"), the octothorpe
 sign ("#"), the equal sign ("="), the colon (":"), and the hyphen ("-"). Symbols are used for a
 variety of purposes. All characters in a symbol are significant. A symbol token is terminated just
 prior to the first non-symbol character following the symbol character sequence. Currently, a
 symbol is limited to a maximum of 255 characters in length.
*/
impl GrammarNode for Symbol {
    fn check_start(s: &str) -> bool {
        s.starts_with(|ch: char| ch.is_ascii_alphanumeric())
    }

    fn parse_wrapped(s: &str) -> crate::Result<(Self, &str)>
    where
        Self: Sized,
    {
        if let Some(end_index) = s.find(|ch| !is_identifier_continuation(ch)) {
            Ok((Symbol(s[..end_index].to_string()), &s[end_index..]))
        } else {
            Ok((Symbol(s.to_string()), ""))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_check_start() {
        assert!(Symbol::check_start("abc"));
        assert!(Symbol::check_start("zyx"));
        assert!(Symbol::check_start("Abc"));
        assert!(Symbol::check_start("Zyx"));
        assert!(Symbol::check_start("1bc"));
        assert!(Symbol::check_start("0zf"));
        assert!(Symbol::check_start("100"));

        assert!(!Symbol::check_start("_"));
        assert!(!Symbol::check_start("-"));
        assert!(!Symbol::check_start("#"));
        assert!(!Symbol::check_start("&"));
        assert!(!Symbol::check_start("@"));
        assert!(!Symbol::check_start("$"));
        assert!(!Symbol::check_start("("));
        assert!(!Symbol::check_start(")"));
        assert!(!Symbol::check_start("+"));
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            (Symbol("simple".to_string()), " XXX"),
            Symbol::parse("simple XXX").unwrap()
        );
        assert_eq!(
            (Symbol("UPPER".to_string()), " XXX"),
            Symbol::parse("UPPER XXX").unwrap()
        );
        assert_eq!(
            (Symbol("MiXeD".to_string()), " XXX"),
            Symbol::parse("MiXeD XXX").unwrap()
        );
        assert_eq!(
            (Symbol("Z-+#_=:".to_string()), "@XXX"),
            Symbol::parse("Z-+#_=:@XXX").unwrap()
        );
        assert_eq!(
            (Symbol("Z---".to_string()), "@--- XXX"),
            Symbol::parse("Z---@--- XXX").unwrap()
        );

        assert_eq!(
            (Symbol("simple".to_string()), ""),
            Symbol::parse("simple").unwrap()
        );
        assert_eq!(
            (Symbol("UPPER".to_string()), ""),
            Symbol::parse("UPPER").unwrap()
        );
        assert_eq!(
            (Symbol("MiXeD".to_string()), ""),
            Symbol::parse("MiXeD").unwrap()
        );
        assert_eq!(
            (Symbol("Z-+#_=:".to_string()), ""),
            Symbol::parse("Z-+#_=:").unwrap()
        );
    }
}
