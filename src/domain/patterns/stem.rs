use std::fmt;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Stem {
    UP,
    DOWN,
}

impl fmt::Display for Stem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
