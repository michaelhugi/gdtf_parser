use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Connectors {
    #[serde(rename = "Connector")]
    pub items: Option<Vec<Connector>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Connector {}