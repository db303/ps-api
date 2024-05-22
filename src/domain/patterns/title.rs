use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Title(String);

impl Title {
    pub fn parse(s: String) -> Result<Title, String> {
        let is_too_long = s.graphemes(true).count() > 100;
        let is_empty = s.trim().is_empty();

        if is_too_long || is_empty {
            Err(format!("{} is not a valid pattern title.", s))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for Title {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Title;
    use claims::{assert_err, assert_ok};
    #[test]
    fn a_100_grapheme_long_title_is_valid() {
        let title = "a".repeat(100);
        assert_ok!(Title::parse(title));
    }
    #[test]
    fn a_name_longer_than_100_graphemes_is_rejected() {
        let title = "a".repeat(101);
        assert_err!(Title::parse(title));
    }
    #[test]
    fn a_valid_title_is_parsed_successfully() {
        let title = "May demo".to_string();
        assert_ok!(Title::parse(title));
    }
}
