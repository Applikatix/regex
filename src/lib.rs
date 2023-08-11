use std::{fmt, error::Error, str::Chars};

use element::{Elem, Amount, Value};

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

        for char in string.chars() {
            elements.push(match char {
                char => Elem {
                    value: Value::Literal(char),
                    quantity: Amount::One,
                }
            })
        }

        Ok(Regex { elements })
    }

    pub fn is_match(&self, string: &str) -> bool {
        let mut string = string.chars();

        loop {
            if self.match_elems(string.clone()) {
                return true;
            }

            if let None = string.next() { break; }
        }

        false
    }

    fn match_elems(&self, mut chars: Chars) -> bool {
        let mut elements = self.elements.iter();
        for elem in elements { match elem.value {
            Value::Literal(val) => match chars.next() {
                Some(char) if char == val => continue,
                _ => return false,
            },
        }}
        
        true
    }

    pub fn find(&self, _string: &str) -> Matches {
        todo!();
    }
}

pub struct Matches; // todo

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
