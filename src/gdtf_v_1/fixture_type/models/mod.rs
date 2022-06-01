use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Models {
    #[serde(rename = "Model")]
    pub items: Option<Vec<Model>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Model {}