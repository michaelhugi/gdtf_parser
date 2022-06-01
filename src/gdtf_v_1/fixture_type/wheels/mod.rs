use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Wheels {
    #[serde(rename = "Wheel")]
    pub items: Option<Vec<Wheel>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Wheel {
    #[serde(rename = "Slot")]
    pub slots: Option<Vec<Slot>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Slot {
    #[serde(rename = "Facet")]
    pub facets: Option<Vec<Facet>>,
    #[serde(rename = "Facet")]
    pub animation_system: Option<AnimationSystem>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Facet {}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct AnimationSystem {}