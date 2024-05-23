use crate::domain::{Author, EFXNotes, Knob, NewTB303Step, Title, Waveform};

pub struct NewTB303Pattern {
    pub author: Option<Author>,
    pub title: Title,
    pub efx_notes: Option<EFXNotes>,
    pub waveform: Option<Waveform>,
    pub cut_off_freq: Option<Knob>,
    pub resonance: Option<Knob>,
    pub env_mod: Option<Knob>,
    pub decay: Option<Knob>,
    pub accent: Option<Knob>,
    pub steps: Vec<NewTB303Step>,
}
