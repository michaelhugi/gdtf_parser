use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum DataVersion {
    #[serde(rename = "1.0")]
    Version1_0,
    #[serde(rename = "1.1")]
    Version1_1,
}

impl DataVersion {
    pub fn major(&self) -> u8 {
        match self {
            Self::Version1_0 => 1,
            Self::Version1_1 => 1,
        }
    }
    pub fn minor(&self) -> u8 {
        match self {
            Self::Version1_0 => 0,
            Self::Version1_1 => 1,
        }
    }
}
