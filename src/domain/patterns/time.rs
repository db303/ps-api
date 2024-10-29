use std::fmt;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum Time {
    NOTE,
    TIED,
    REST,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
