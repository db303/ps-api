use std::fmt;

#[derive(Debug, serde::Deserialize)]
pub enum Device {
    TB303,
    TR909,
    TR808,
    TR606,
}

impl fmt::Display for Device {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
