use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx_step::MacroDMXStep;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MacroDmx {
    #[serde(rename = "MacroDMXStep")]
    pub steps: Option<Vec<MacroDMXStep>>,
}