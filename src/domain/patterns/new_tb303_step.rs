use crate::domain::{Note, Stem, Time};

pub struct NewTB303Step {
    pub note: Option<Note>,
    pub stem: Option<Stem>,
    pub time: Time,
    pub accent: Option<bool>,
    pub slide: Option<bool>,
}
