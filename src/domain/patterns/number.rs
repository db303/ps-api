#[derive(Debug)]
pub struct Number(i32);

impl Number {
    pub fn parse(i: i32) -> Result<Number, String> {
        let is_less_than_1 = i < 1;
        let is_greater_than_16 = i > 16;

        if is_greater_than_16 || is_less_than_1 {
            Err(format!("{} is not a valid step number value. ", i))
        } else {
            Ok(Self(i))
        }
    }
}

impl AsRef<i32> for Number {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::Number;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_value_less_than_1_is_rejected() {
        let number: i32 = -1;
        assert_err!(Number::parse(number));
    }

    #[test]
    fn a_value_greater_than_16_is_rejected() {
        let number: i32 = 17;
        assert_err!(Number::parse(number));
    }
    #[test]
    fn a_value_in_a_range_between_1_and_16_is_valid() {
        let number: i32 = 8;
        assert_ok!(Number::parse(number));
    }
}
