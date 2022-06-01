use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FeatureGroups {
    #[serde(rename = "FeatureGroup")]
    pub items: Option<Vec<FeatureGroup>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FeatureGroup {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Pretty")]
    pub pretty: String,
    #[serde(rename = "Feature")]
    pub features: Option<Vec<Feature>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Feature {}
