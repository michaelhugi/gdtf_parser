use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FTRDM {
    #[serde(rename = "SoftwareVersionID")]
    pub software_version_ids: Option<Vec<SoftwareVersionID>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct SoftwareVersionID {
    #[serde(rename = "DMXPersonality")]
    pub dmx_personalities: Option<Vec<DMXPersonality>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXPersonality {}