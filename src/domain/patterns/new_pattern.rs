use crate::domain::patterns::device::Device;
use crate::domain::patterns::pattern_data::PatternData;
use crate::domain::patterns::pattern_name::PatternName;

pub struct NewPattern {
    pub name: PatternName,
    pub device: Device,
    pub data: PatternData,
}
