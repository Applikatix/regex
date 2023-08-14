use std::{fmt, error::Error};

use element::{Elem, Amount, Value, Match};

pub struct Regex {
    elements: Vec<Elem>,
}
impl Regex {
    pub fn new(string: &str) -> Regex {
        Regex::parse(string).unwrap_or_else(|err| {
            panic!("problem parsing string to regex: {err}")
        })
    }

    pub fn parse(string: &str) -> Result<Regex, ParseRegexError> {
        let mut elements = Vec::new();

        let mut chars = string.chars();
        while let Some(char) = chars.next() { match char {
            '\\' => {
                let char = match chars.next() {
                    Some(c) => c,
                    None => return Err(ParseRegexError),
                };
                elements.push(Elem::new(Value::Literal(char)));
            }
            '.' => elements.push(Elem::new(Value::Wildcard)),
            char => elements.push(Elem::new(Value::Literal(char))),
        }}

        Ok(Regex { elements })
    }

    pub fn is_match(&self, string: &str) -> bool {
        let mut chars = string.chars();

        loop {
            if Elem::group_compare(&self.elements, &mut chars.clone()) {
                return true;
            }

            if let None = chars.next() { break; }
        }

        false
    }

    pub fn find(&self, _string: &str) -> Vec<Match> {
        todo!();
    }
}

#[derive(Debug)]
pub struct ParseRegexError; // todo: Værdier
impl fmt::Display for ParseRegexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to parse")
    }
}
impl Error for ParseRegexError { }

mod element;

#[cfg(test)]
mod tests;
