use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct CRIs {
    #[serde(rename = "CRIGroup")]
    pub cri_groups: Option<Vec<CRIGroup>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct CRIGroup {
    #[serde(rename = "CRI")]
    pub cris: Option<Vec<CRI>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct CRI {}