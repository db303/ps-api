use std::fmt;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Note {
    A,
    B,
    C,
    Cs,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
