use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct EFXNotes(String);

impl EFXNotes {
    pub fn parse(s: String) -> Result<EFXNotes, String> {
        let is_too_long = s.graphemes(true).count() > 500;
        let is_empty = s.trim().is_empty();

        if is_too_long || is_empty {
            Err(format!("{} is not a valid efx / notes.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for EFXNotes {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::EFXNotes;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_500_grapheme_long_notes_are_valid() {
        let efx_notes = "a".repeat(500);
        assert_ok!(EFXNotes::parse(efx_notes));
    }
    #[test]
    fn a_efx_note_longer_than_500_graphemes_is_rejected() {
        let efx_notes = "a".repeat(501);
        assert_err!(EFXNotes::parse(efx_notes));
    }
    #[test]
    fn a_valid_efx_note_is_parsed_successfully() {
        let description = "This is efx/notes for the pattern".to_string();
        assert_ok!(EFXNotes::parse(description));
    }
}
