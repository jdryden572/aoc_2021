use std::fmt::Display;

pub struct Pair(pub Element, pub Element);

impl Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(write!(f, "[{},{}]", self.0, self.1)?)
    }
}

pub enum Element {
    Number(usize),
    Pair(Box<Pair>)
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(match self {
            Element::Number(num) => write!(f, "{}", num)?,
            Element::Pair(pair) => write!(f, "{}", pair)?,
        })
    }
}