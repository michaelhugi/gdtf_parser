use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::attribute_definitions::activation_groups::ActivationGroups;
use crate::gdtf_v_1::fixture_type::attribute_definitions::attributes::Attributes;
use crate::gdtf_v_1::fixture_type::attribute_definitions::feature_groups::FeatureGroups;

pub mod activation_groups;
pub mod feature_groups;
pub mod attributes;


#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct AttributeDefinitions {
    #[serde(rename = "ActivationGroups")]
    pub activation_groups: Option<ActivationGroups>,
    #[serde(rename = "FeatureGroups")]
    pub feature_groups: FeatureGroups,
    #[serde(rename = "Attributes")]
    pub attributes: Attributes,
}

#[cfg(test)]
pub(crate) mod attribute_definitions_test {
    use crate::gdtf_v_1::fixture_type::attribute_definitions::activation_groups::activation_groups_test;
    use crate::gdtf_v_1::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::utils::errors::GdtfError;

    pub(crate) fn test_acme_ae_610_beam(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_acme_ae_610_beam(&ad.activation_groups)?;
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_jb_12_spot_hp(&ad.activation_groups)?;
        return Ok(());
    }


    pub(crate) fn test_robe_robin_viva_cmy(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_robe_robin_viva_cmy(&ad.activation_groups)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
        activation_groups_test::test_sgm_g7_spot(&ad.activation_groups)?;
        return Ok(());
    }
}