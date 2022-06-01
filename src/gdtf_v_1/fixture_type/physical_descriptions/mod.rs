pub mod dmx_profiles;
pub mod color_space;
pub mod emitters;
pub mod filters;
pub mod cris;
pub mod connectors;
pub mod properties;
pub mod measurement;

use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::physical_descriptions::color_space::ColorSpace;
use crate::gdtf_v_1::fixture_type::physical_descriptions::connectors::Connectors;
use crate::gdtf_v_1::fixture_type::physical_descriptions::cris::CRIs;
use crate::gdtf_v_1::fixture_type::physical_descriptions::dmx_profiles::DMXProfiles;
use crate::gdtf_v_1::fixture_type::physical_descriptions::emitters::Emitters;
use crate::gdtf_v_1::fixture_type::physical_descriptions::filters::Filters;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::Properties;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct PhysicalDescriptions {
    #[serde(rename = "Emitters")]
    pub emitters: Option<Emitters>,
    #[serde(rename = "Filters")]
    pub filters: Option<Filters>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<ColorSpace>,
    #[serde(rename = "DMXProfiles")]
    pub dmx_profiles: Option<DMXProfiles>,
    #[serde(rename = "CRIs")]
    pub cris: Option<CRIs>,
    #[serde(rename = "Connectors")]
    pub connectors: Option<Connectors>,
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,
}