use crate::domain::{Device, JsonTB303, JsonTR909, PatternRequestData};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct PatternData(serde_json::Value);

impl PatternData {
    pub fn parse(s: PatternRequestData, device: &Device) -> Result<PatternData, String> {
        let pattern_json_data: serde_json::Value = serde_json::to_value(s).unwrap();

        match device {
            Device::TB303 => {
                let data_303: Result<JsonTB303, _> =
                    serde_json::from_value(pattern_json_data.clone());

                if data_303.is_err() {
                    return Err("Not able to parse 303 pattern data.".to_string());
                }

                Ok(Self(pattern_json_data))
            }
            Device::TR909 => {
                let data_909: Result<JsonTR909, _> =
                    serde_json::from_value(pattern_json_data.clone());

                if data_909.is_err() {
                    return Err("Not able to parse 909 pattern data.".to_string());
                }

                Ok(Self(pattern_json_data))
            }
            _ => Err("Device is not recognized".to_string()),
        }
    }
}

//
// impl AsRef<str> for PatternData {
//     fn as_ref(&self) -> &str {
//         &self.0
//     }
// }
