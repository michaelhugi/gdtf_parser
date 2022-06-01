use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::physical_descriptions::measurement::Measurement;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Emitters {
    #[serde(rename = "Emitter")]
    pub items: Option<Vec<Emitter>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Emitter {
    #[serde(rename = "Measurement")]
    pub measurements: Option<Vec<Measurement>>,
}

