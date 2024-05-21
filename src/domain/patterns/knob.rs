use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct Knob(i16);

impl Knob {
    pub fn parse(i: i16) -> Result<Knob, String> {

        let is_greater_than_360 = i > 360;
        let is_less_than_0 = i < 0;

        if is_greater_than_360 || is_less_than_0 {
            // show error message if value is less than 0 or greater than 360
            Err(format!("{} is not a valid knob value. ", i))
        } else {
            Ok(Self(i))
        }
    }
}

impl AsRef<i16> for Knob {
    fn as_ref(&self) -> &i16 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Knob;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_value_less_than_0_is_rejected() {
        let knob: i16 = -1;
        assert_err!(Knob::parse(knob));
    }
    #[test]
    fn a_value_greater_than_360_is_rejected() {
        let knob:i16 = 361;
        assert_err!(Knob::parse(knob));
    }
    #[test]
    fn a_value_in_a_range_between_0_and_360_is_valid() {
        let knob:i16 = 180;
        assert_ok!(Knob::parse(knob));
    }
}
