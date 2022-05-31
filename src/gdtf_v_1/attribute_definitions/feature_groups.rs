use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FeatureGroups {
    #[serde(rename = "FeatureGroup")]
    pub feature_group: Vec<FeatureGroup>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FeatureGroup {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Pretty")]
    pub pretty: String
}

#[cfg(test)]
pub(crate) mod feature_groups_test {
    use crate::gdtf_v_1::attribute_definitions::activation_groups::{ActivationGroup, ActivationGroups};
    use crate::gdtf_v_1::attribute_definitions::AttributeDefinitions;
    use crate::gdtf_v_1::attribute_definitions::feature_groups::FeatureGroups;
    use crate::GdtfError;

    pub(crate) fn test_acme_ae_610_beam(fg: &FeatureGroups) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(fg: &FeatureGroups) -> Result<(), GdtfError> {
        return Ok(());
    }


    pub(crate) fn test_robe_robin_viva_cmy(fg: &FeatureGroups) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(fg: &FeatureGroups) -> Result<(), GdtfError> {
        return Ok(());
    }
}