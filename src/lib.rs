#![allow(dead_code)]

#[derive(Debug, Clone)]
pub enum Regex {
    Null,
    Empty,
    Char(char),
    Concat(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
    Or(Box<Regex>, Box<Regex>),
    And(Box<Regex>, Box<Regex>),
    Not(Box<Regex>),
}

fn nullable_bool(r: &Regex) -> bool {
    match r {
        Regex::Null => false,
        Regex::Empty => true,
        Regex::Char(_) => false,
        Regex::Concat(s, t) | Regex::And(s, t) => nullable_bool(s) && nullable_bool(t),
        Regex::Or(s, t) => nullable_bool(s) || nullable_bool(t),
        Regex::Star(_) => true,
        Regex::Not(s) => !nullable_bool(s),
    }
}

fn nullable(r: &Regex) -> Regex {
    if nullable_bool(r) {
        Regex::Empty
    } else {
        Regex::Null
    }
}

fn differentiate_char(c: char, r: &Regex) -> Regex {
    match r {
        Regex::Empty => Regex::Null,
        Regex::Char(b) => {
            if *b == c {
                Regex::Empty
            } else {
                Regex::Null
            }
        }
        Regex::Null => Regex::Null,
        Regex::Concat(s, t) => Regex::Or(
            Box::new(Regex::Concat(
                Box::new(differentiate_char(c, &s.clone())),
                t.clone(),
            )),
            Box::new(Regex::Concat(
                Box::new(nullable(s)),
                Box::new(differentiate_char(c, t)),
            )),
        ),
        Regex::Star(s) => Regex::Concat(
            Box::new(differentiate_char(c, &s.clone())),
            Box::new(Regex::Star(s.clone())),
        ),
        Regex::Or(s, t) => Regex::Or(
            Box::new(differentiate_char(c, s)),
            Box::new(differentiate_char(c, t)),
        ),
        Regex::And(s, t) => Regex::And(
            Box::new(differentiate_char(c, s)),
            Box::new(differentiate_char(c, t)),
        ),
        Regex::Not(s) => Regex::Not(Box::new(differentiate_char(c, s))),
    }
}

fn differentiate(s: &str, r: &Regex) -> Regex {
    let mut result = r.clone();
    for c in s.chars() {
        result = differentiate_char(c, &result);
    }
    result
}

pub fn match_regex(r: &Regex, s: &str) -> bool {
    nullable_bool(&differentiate(s, r))
}

#[cfg(test)]
mod tests {
    use crate::{Regex, match_regex};

    #[test]
    fn match_null() {
        let regex = Regex::Null;
        assert!(!match_regex(&regex, ""));
        assert!(!match_regex(&regex, "a"));
    }

    #[test]
    fn match_empty() {
        let regex = Regex::Empty;
        assert!(match_regex(&regex, ""));
        assert!(!match_regex(&regex, "a"));
    }

    #[test]
    fn match_char() {
        let regex = Regex::Char('b');
        assert!(!match_regex(&regex, ""));
        assert!(!match_regex(&regex, "a"));
        assert!(match_regex(&regex, "b"));
        assert!(!match_regex(&regex, "bb"));
    }

    #[test]
    fn match_star() {
        let regex = Regex::Star(Box::new(Regex::Char('a')));
        assert!(match_regex(&regex, ""));
        assert!(match_regex(&regex, "a"));
        assert!(match_regex(&regex, "aa"));
        assert!(match_regex(&regex, "aaa"));
        assert!(!match_regex(&regex, "aaab"));
        assert!(!match_regex(&regex, "aaba"));
        assert!(!match_regex(&regex, "baaa"));
        assert!(!match_regex(&regex, "b"));
    }

    #[test]
    fn match_concat() {
        let regex = Regex::Concat(Box::new(Regex::Char('a')), Box::new(Regex::Char('b')));
        assert!(!match_regex(&regex, ""));
        assert!(!match_regex(&regex, "a"));
        assert!(!match_regex(&regex, "aa"));
        assert!(match_regex(&regex, "ab"));
        assert!(!match_regex(&regex, "abc"));
    }

    #[test]
    fn match_or() {
        let regex = Regex::Or(Box::new(Regex::Char('a')), Box::new(Regex::Char('b')));
        assert!(!match_regex(&regex, ""));
        assert!(match_regex(&regex, "a"));
        assert!(match_regex(&regex, "b"));
        assert!(!match_regex(&regex, "ab"));
    }

    #[test]
    fn match_and() {
        let regex = Regex::And(Box::new(Regex::Char('a')), Box::new(Regex::Star(Box::new(Regex::Char('a')))));
        assert!(!match_regex(&regex, ""));
        assert!(match_regex(&regex, "a"));
        assert!(!match_regex(&regex, "aa"));
    }

    #[test]
    fn match_not() {
        let regex = Regex::Not(Box::new(Regex::Char('a')));
        assert!(match_regex(&regex, ""));
        assert!(!match_regex(&regex, "a"));
        assert!(match_regex(&regex, "b"));
        assert!(match_regex(&regex, "aa"));
    }
}
