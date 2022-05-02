use crate::domain::{Note, Stem, Time};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum PatternRequestData {
    JsonTB303(JsonTB303),
    JsonTR909(JsonTR909),
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct JsonTB303 {
    length: i32,
    steps: Vec<TB303Step>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TB303Step {
    note: Note,
    stem: Stem,
    accent: bool,
    slide: bool,
    time: Time,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct JsonTR909 {
    length: i32,
    steps: Vec<TR909Step>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct TR909Step {
    accent: bool,
    bd: bool,
    sd: bool,
    lt: bool,
    mt: bool,
    ht: bool,
    rs: bool,
    cp: bool,
    oh: bool,
    ch: bool,
    cr: bool,
    ri: bool,
}
