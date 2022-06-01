pub mod macro_dmx_step;
pub mod macro_dmx_value;
pub mod macro_dmx;

use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx::MacroDmx;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FTMacros {
    #[serde(rename = "FTMacro")]
    pub items: Option<Vec<FTMacro>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FTMacro {
    #[serde(rename = "MacroDmx")]
    pub macro_dmx: Option<MacroDmx>,
}