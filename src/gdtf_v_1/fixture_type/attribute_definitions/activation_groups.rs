use serde::Deserialize;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroups {
    #[serde(rename = "ActivationGroup", default)]
    pub items: Option<Vec<ActivationGroup>>,

}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroup {
    #[serde(rename = "Name")]
    pub name: String,
}

#[cfg(test)]
pub(crate) mod activation_groups_test {
    use crate::gdtf_v_1::fixture_type::attribute_definitions::activation_groups::ActivationGroups;
    use crate::utils::errors::GdtfError;

    pub(crate) fn test_acme_ae_610_beam(_ag: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(_ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(_ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(_ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }
}