#[derive(Debug)]
pub struct Time(String);

impl Time {
    pub fn parse(s: String) -> Result<Time, String> {
        let is_not_valid_time = !["note", "tied", "rest"].contains(&s.as_str());

        if is_not_valid_time {
            Err(format!(
                "{} is not a valid time. Can only be one of 'note', 'tied', 'rest'",
                s
            ))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for Time {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Time;
    use claims::{assert_err, assert_ok};

    #[test]
    fn invalid_time_is_rejected() {
        let time = "invalid_time".to_string();
        assert_err!(Time::parse(time));
    }

    #[test]
    fn valid_times_are_accepted() {
        for time in vec!["note", "tied", "rest"] {
            assert_ok!(Time::parse(time.to_string()));
        }
    }
}
