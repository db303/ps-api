#[derive(Debug)]
pub struct Stem(String);

impl Stem {
    pub fn parse(s: String) -> Result<Stem, String> {
        let is_not_valid_stem = !["up", "down", "none"].contains(&s.as_str());

        if is_not_valid_stem {
            Err(format!(
                "{} is not a valid stem. Can only be one of 'up', 'down', 'none'",
                s
            ))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for Stem {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Stem;
    use claims::{assert_err, assert_ok};

    fn valid_stem_values() -> Vec<String> {
        vec!["up".to_string(), "down".to_string(), "none".to_string()]
    }

    #[test]
    fn invalid_stem_is_rejected() {
        let stem = "invalid_stem".to_string();
        assert_err!(Stem::parse(stem));
    }

    #[test]
    fn valid_stems_are_accepted() {
        for stem in valid_stem_values() {
            assert_ok!(Stem::parse(stem));
        }
    }
}
