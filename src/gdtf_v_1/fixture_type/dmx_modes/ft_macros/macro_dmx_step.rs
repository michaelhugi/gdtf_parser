use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx_value::MacroDMXValue;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MacroDMXStep {
    #[serde(rename = "MacroDMXValue")]
    pub dmx_values: Option<Vec<MacroDMXValue>>,
}