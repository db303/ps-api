use crate::domain::{Author, EFXNotes, Knob, Title, Waveform};

pub struct NewTB303Pattern {
    pub author: Author,
    pub title: Title,
    pub efx_notes: EFXNotes,
    pub waveform: Waveform,
    pub cut_off_freq: Knob,
    pub resonance: Knob,
    pub env_mod: Knob,
    pub decay: Knob,
    pub accent: Knob,
}

