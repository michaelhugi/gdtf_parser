use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::logical_channel::LogicalChannel;

pub mod channel_function;
pub mod channel_set;
pub mod logical_channel;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXChannels {
    #[serde(rename = "DMXChannel")]
    pub items: Option<Vec<DMXChannel>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXChannel {
    #[serde(rename = "LogicalChannel")]
    pub logical_channels: Option<Vec<LogicalChannel>>,
}