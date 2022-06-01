use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::channel_function::ChannelFunction;


#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct LogicalChannel{
    #[serde(rename = "ChannelFunction")]
    pub channel_functions: Option<Vec<ChannelFunction>>
}