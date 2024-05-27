use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Author(String);

impl Author {
    pub fn parse(s: String) -> Result<Author, String> {
        let is_too_long = s.graphemes(true).count() > 50;
        let is_empty = s.trim().is_empty();

        if is_too_long || is_empty {
            Err(format!("{} is not a valid author.", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Author {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Author;
    use claims::{assert_err, assert_ok};

    #[test]
    fn an_empty_author_is_rejected() {
        let author = "".to_string();
        assert_err!(Author::parse(author));
    }

    #[test]
    fn an_author_with_only_whitespace_is_rejected() {
        let author = " ".to_string();
        assert_err!(Author::parse(author));
    }

    #[test]
    fn a_50_grapheme_long_author_is_valid() {
        let author = "a".repeat(50);
        assert_ok!(Author::parse(author));
    }
    #[test]
    fn an_author_longer_than_50_graphemes_is_rejected() {
        let author = "a".repeat(51);
        assert_err!(Author::parse(author));
    }
    #[test]
    fn a_valid_author_is_parsed_successfully() {
        let author = "myself".to_string();
        assert_ok!(Author::parse(author));
    }
}
