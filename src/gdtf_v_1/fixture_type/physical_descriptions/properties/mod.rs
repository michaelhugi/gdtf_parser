use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::leg_height::LegHeight;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::operating_temperature::OperatingTemperature;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::power_consumption::PowerConsumption;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::weight::Weight;

pub mod leg_height;
pub mod operating_temperature;
pub mod power_consumption;
pub mod weight;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Properties {
    #[serde(rename = "OperatingTemperature")]
    pub operating_temperature: Option<OperatingTemperature>,
    #[serde(rename = "Weight")]
    pub weight: Option<Weight>,
    #[serde(rename = "PowerConsumption")]
    pub power_consumption: Option<Vec<PowerConsumption>>,
    #[serde(rename = "LegHeight")]
    pub leg_height: Option<LegHeight>,
}