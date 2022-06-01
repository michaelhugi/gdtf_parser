use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Revisions {
    #[serde(rename = "Revision")]
    pub items: Option<Vec<Revision>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Revision {}