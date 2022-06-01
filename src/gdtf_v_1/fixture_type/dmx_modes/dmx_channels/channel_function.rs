use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::channel_set::ChannelSet;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ChannelFunction{
    #[serde(rename = "ChannelSet")]
    pub channel_sets: Option<Vec<ChannelSet>>
}