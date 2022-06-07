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
pub(crate) mod test {
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;




    use crate::gdtf_v_1::fixture_type::attribute_definitions::activation_groups::test as activation_groups_test;
    use crate::gdtf_v_1::fixture_type::attribute_definitions::attributes::test as attributes_test;
    use crate::gdtf_v_1::fixture_type::attribute_definitions::feature_groups::test as feature_groups_test;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_acme_ae_610t(gdtf)?;
        attributes_test::test_acme_ae_610t(gdtf)?;
        feature_groups_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_adb_klemantis(gdtf)?;
        attributes_test::test_adb_klemantis(gdtf)?;
        feature_groups_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_adj_mega_tripar(gdtf)?;
        attributes_test::test_adj_mega_tripar(gdtf)?;
        feature_groups_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_adsi_dataton(gdtf)?;
        attributes_test::test_adsi_dataton(gdtf)?;
        feature_groups_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_china_36x10(gdtf)?;
        attributes_test::test_china_36x10(gdtf)?;
        feature_groups_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_jb_lighting_p12(gdtf)?;
        attributes_test::test_jb_lighting_p12(gdtf)?;
        feature_groups_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_robe_robin_viva_cmy(gdtf)?;
        attributes_test::test_robe_robin_viva_cmy(gdtf)?;
        feature_groups_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_sgm_g7_spot(gdtf)?;
        attributes_test::test_sgm_g7_spot(gdtf)?;
        feature_groups_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_sgm_p6(gdtf)?;
        attributes_test::test_sgm_p6(gdtf)?;
        feature_groups_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_shenzhen_mini_led_spot(gdtf)?;
        attributes_test::test_shenzhen_mini_led_spot(gdtf)?;
        feature_groups_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        activation_groups_test::test_stairville_fan_200(gdtf)?;
        attributes_test::test_stairville_fan_200(gdtf)?;
        feature_groups_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}