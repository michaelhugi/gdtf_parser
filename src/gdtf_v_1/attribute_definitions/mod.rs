use serde::Deserialize;

use crate::gdtf_v_1::attribute_definitions::activation_groups::ActivationGroups;
use crate::gdtf_v_1::attribute_definitions::feature_groups::FeatureGroups;

pub mod activation_groups;
pub mod feature_groups;


#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct AttributeDefinitions {
    #[serde(rename = "ActivationGroups")]
    pub activation_groups: Option<ActivationGroups>,
    #[serde(rename = "FeatureGroups")]
    pub feature_groups: FeatureGroups,
}

#[cfg(test)]
pub(crate) mod attribute_definitions_test {
    use crate::gdtf_v_1::attribute_definitions::activation_groups::activation_groups_test;
    use crate::gdtf_v_1::attribute_definitions::AttributeDefinitions;
    use crate::gdtf_v_1::attribute_definitions::feature_groups::feature_groups_test;
    use crate::GdtfError;

    pub(crate) fn test_acme_ae_610_beam(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_acme_ae_610_beam(&ad.activation_groups)?;
        feature_groups_test::test_acme_ae_610_beam(&ad.feature_groups)?;
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_jb_12_spot_hp(&ad.activation_groups)?;
        feature_groups_test::test_jb_12_spot_hp(&ad.feature_groups)?;
        return Ok(());
    }


    pub(crate) fn test_robe_robin_viva_cmy(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_robe_robin_viva_cmy(&ad.activation_groups)?;
        feature_groups_test::test_robe_robin_viva_cmy(&ad.feature_groups)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_sgm_g7_spot(&ad.activation_groups)?;
        feature_groups_test::test_sgm_g7_spot(&ad.feature_groups)?;
        return Ok(());
    }
}