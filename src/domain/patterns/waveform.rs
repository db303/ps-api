#[derive(Debug)]
pub struct Waveform(String);

impl Waveform {
    pub fn parse(s: String) -> Result<Waveform, String> {
        let is_not_sawtooth = s != "sawtooth";
        let is_not_square = s != "square";

        if is_not_sawtooth && is_not_square {
            Err(format!(
                "{} is not a valid waveform. Can only be 'sawtooth' or 'square'",
                s
            ))
        } else {
            Ok(Self(s))
        }
    }
}
impl AsRef<str> for Waveform {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Waveform;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_sawtooth_waveform_is_valid() {
        let waveform = "sawtooth".to_string();
        assert_ok!(Waveform::parse(waveform));
    }

    #[test]
    fn a_square_waveform_is_valid() {
        let waveform = "square".to_string();
        assert_ok!(Waveform::parse(waveform));
    }

    #[test]
    fn a_waveform_other_than_sawtooth_or_square_is_rejected() {
        let waveform = "sine".to_string();
        assert_err!(Waveform::parse(waveform));
    }
}
