#[derive(Debug)]
pub struct Note(String);

impl Note {
    pub fn parse(s: String) -> Result<Note, String> {
        let is_not_in_array_of_notes = ![
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B", "Ch",
        ]
        .contains(&s.as_str());

        if is_not_in_array_of_notes {
            Err(format!(
                "{} is not a valid note. Can only be one of C, C#, D, D#, E, F, F#, G, G#, A, A#, B, Ch",
                s
            ))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for Note {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Note;
    use claims::{assert_err, assert_ok};

    fn valid_notes() -> Vec<String> {
        vec![
            "C".to_string(),
            "C#".to_string(),
            "D".to_string(),
            "D#".to_string(),
            "E".to_string(),
            "F".to_string(),
            "F#".to_string(),
            "G".to_string(),
            "G#".to_string(),
            "A".to_string(),
            "A#".to_string(),
            "B".to_string(),
            "Ch".to_string(),
        ]
    }

    #[test]
    fn invalid_note_is_rejected() {
        let note = "L".to_string();
        assert_err!(Note::parse(note));
    }

    #[test]
    fn valid_notes_are_accepted() {
        for note in valid_notes() {
            assert_ok!(Note::parse(note));
        }
    }
}
