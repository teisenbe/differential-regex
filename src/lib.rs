use std::fmt;

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

impl Regex {
    pub fn format_regex(&self) -> String {
        match self {
            Regex::Null => String::from("∅"),
            Regex::Empty => String::from("\"\""),
            Regex::Char(b) => format!("Char({})", b),
            Regex::Concat(s, t) => format!("Concat({}, {})", s.format_regex(), t.format_regex()),
            Regex::Star(s) => format!("Star({})", s.format_regex()),
            Regex::Or(s, t) => format!("Or({}, {})", s.format_regex(), t.format_regex()),
            Regex::And(s, t) => format!("And({}, {})", s.format_regex(), t.format_regex()),
            Regex::Not(s) => format!("Not({})", s.format_regex()),
        }
    }

    fn nullable_bool(&self) -> bool {
        match self {
            Regex::Null => false,
            Regex::Empty => true,
            Regex::Char(_) => false,
            Regex::Concat(s, t) | Regex::And(s, t) => s.nullable_bool() && t.nullable_bool(),
            Regex::Or(s, t) => s.nullable_bool() || t.nullable_bool(),
            Regex::Star(_) => true,
            Regex::Not(s) => !s.nullable_bool(),
        }
    }

    fn nullable(&self) -> Regex {
        if self.nullable_bool() {
            Regex::Empty
        } else {
            Regex::Null
        }
    }

    fn differentiate_char(&self, c: char) -> Regex {
        match self {
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
                    Box::new(s.clone().differentiate_char(c)),
                    t.clone(),
                )),
                Box::new(Regex::Concat(
                    Box::new(s.nullable()),
                    Box::new(t.differentiate_char(c)),
                )),
            ),
            Regex::Star(s) => Regex::Concat(
                Box::new(s.clone().differentiate_char(c)),
                Box::new(Regex::Star(s.clone())),
            ),
            Regex::Or(s, t) => Regex::Or(
                Box::new(s.differentiate_char(c)),
                Box::new(t.differentiate_char(c)),
            ),
            Regex::And(s, t) => Regex::And(
                Box::new(s.differentiate_char(c)),
                Box::new(t.differentiate_char(c)),
            ),
            Regex::Not(s) => Regex::Not(Box::new(s.differentiate_char(c))),
        }
    }

    fn differentiate(&self, s: &str) -> Regex {
        let mut result = self.clone();
        for c in s.chars() {
            result = result.differentiate_char(c);
        }
        result
    }

    pub fn match_regex(&self, s: &str) -> bool {
        self.differentiate(s).nullable_bool()
    }
}

impl fmt::Display for Regex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_regex())
    }
}

#[cfg(test)]
mod tests {
    use crate::Regex;

    #[test]
    fn match_null() {
        let regex = Regex::Null;
        assert!("∅" == regex.to_string());
        assert!(!regex.match_regex(""));
        assert!(!regex.match_regex("a"));
    }

    #[test]
    fn match_empty() {
        let regex = Regex::Empty;
        assert!("\"\"" == regex.to_string());
        assert!(regex.match_regex(""));
        assert!(!regex.match_regex("a"));
    }

    #[test]
    fn match_char() {
        let regex = Regex::Char('b');
        assert!("Char(b)" == regex.to_string());
        assert!(!regex.match_regex(""));
        assert!(!regex.match_regex("a"));
        assert!(regex.match_regex("b"));
        assert!(!regex.match_regex("bb"));
    }

    #[test]
    fn match_star() {
        let regex = Regex::Star(Box::new(Regex::Char('a')));
        assert!("Star(Char(a))" == regex.to_string());
        assert!(regex.match_regex(""));
        assert!(regex.match_regex("a"));
        assert!(regex.match_regex("aa"));
        assert!(regex.match_regex("aaa"));
        assert!(!regex.match_regex("aaab"));
        assert!(!regex.match_regex("aaba"));
        assert!(!regex.match_regex("baaa"));
        assert!(!regex.match_regex("b"));
    }

    #[test]
    fn match_concat() {
        let regex = Regex::Concat(Box::new(Regex::Char('a')), Box::new(Regex::Char('b')));
        assert!("Concat(Char(a), Char(b))" == regex.to_string());
        assert!(!regex.match_regex(""));
        assert!(!regex.match_regex("a"));
        assert!(!regex.match_regex("aa"));
        assert!(regex.match_regex("ab"));
        assert!(!regex.match_regex("abc"));
    }

    #[test]
    fn match_or() {
        let regex = Regex::Or(Box::new(Regex::Char('a')), Box::new(Regex::Char('b')));
        assert!("Or(Char(a), Char(b))" == regex.to_string());
        println!("{:?}", regex);
        assert!(!regex.match_regex(""));
        assert!(regex.match_regex("a"));
        assert!(regex.match_regex("b"));
        assert!(!regex.match_regex("ab"));
    }

    #[test]
    fn match_and() {
        let regex = Regex::And(
            Box::new(Regex::Char('a')),
            Box::new(Regex::Star(Box::new(Regex::Char('a')))),
        );
        assert!("And(Char(a), Star(Char(a)))" == regex.to_string());
        assert!(!regex.match_regex(""));
        assert!(regex.match_regex("a"));
        assert!(!regex.match_regex("aa"));
    }

    #[test]
    fn match_not() {
        let regex = Regex::Not(Box::new(Regex::Char('a')));
        assert!("Not(Char(a))" == regex.to_string());
        assert!(regex.match_regex(""));
        assert!(!regex.match_regex("a"));
        assert!(regex.match_regex("b"));
        assert!(regex.match_regex("aa"));
    }
}
