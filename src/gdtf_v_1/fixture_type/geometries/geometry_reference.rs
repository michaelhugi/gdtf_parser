use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct GeometryReference {
    #[serde(rename = "Break")]
    pub breaks: Option<Vec<Break>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Break {}