use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::physical_descriptions::measurement::Measurement;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Filters {
    #[serde(rename = "Filter")]
    pub items: Option<Vec<Filter>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Filter {
    #[serde(rename = "Measurement")]
    pub measurements: Option<Vec<Measurement>>,
}



