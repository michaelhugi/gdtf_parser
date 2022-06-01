use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Attributes {
    #[serde(rename = "Attribute")]
    pub items: Option<Vec<Attribute>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Attribute {}