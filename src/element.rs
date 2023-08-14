use std::str::Chars;

#[derive(PartialEq, Debug, Clone)]
pub struct Elem {
    pub value: Value,
    pub amount: Amount,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Literal(char),
    Wildcard,
    //Group(Vec<Elem>),
}
#[derive(PartialEq, Debug, Clone)]
pub enum Amount {
    One,                                        // 'x'
    //Optional,                                 // 'x?'
    //Any,                                      // 'x*'
    // OneOrMore = One + Any                    // 'x+'
    //Exactly(u32),                             // 'x{n}' hvor n >= 1
    // AtLeast(n) = Exactly(n) + Any            // 'x{n,}' hvor n >= 1
    //UpTo(u32),                                // 'x{,m}' hvor n >= 1
    // FromTo(n, m) = Exactly(n) + UpTo(m-n)    // 'x{n,m}' hvor n >= 0, m > n
}

impl Elem {
    pub fn new(value: Value) -> Elem {
        Elem { value, amount: Amount::One }
    }

    pub fn compare_value(&self, chars: &mut Chars) -> bool {
        match self.value {
            Value::Literal(val) => match chars.next() {
                Some(char) if char == val => true,
                _ => false,
            },
            Value::Wildcard => match chars.next() {
                Some(_) => true,
                _ => false,
            },
        }
    }

    pub fn group_compare(elements: &Vec<Elem>, chars: &mut Chars) -> bool {
        for elem in elements {
            if !elem.compare_value(chars) {
                return false;
            }
        }

        true
    }
}

pub enum Match {
    Consumed(u32),
    Failed,
}

impl Match {
    pub fn to_bool(&self) -> bool {
        match *self {
            Self::Consumed(_) => true,
            Self::Failed => false,
        }
    }
}
