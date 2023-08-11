use std::iter::Iterator;

#[derive(PartialEq, Debug, Clone)]
pub struct Elem {
    pub value: Value,
    pub quantity: Amount,
}
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Literal(char),
    //Wildcard,
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
    
}

pub enum Match {
    Consumed(u32),
    Failed,
}
