use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::DMXChannels;
use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::FTMacros;
use crate::gdtf_v_1::fixture_type::dmx_modes::relations::Relations;

pub mod relations;
pub mod ft_macros;
pub mod dmx_channels;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXModes {
    #[serde(rename = "DMXMode")]
    pub items: Option<Option<Vec<DMXMode>>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXMode {
    #[serde(rename = "DMXChannels")]
    pub dmx_channels: DMXChannels,
    #[serde(rename = "Relations")]
    pub relations: Option<Relations>,
    #[serde(rename = "FTMacros")]
    pub ft_macros: Option<FTMacros>,
}