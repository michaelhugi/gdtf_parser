use std::collections::HashMap;
use std::fmt::Formatter;
use serde::{Deserialize, Deserializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use crate::utils::units::name::Name;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroups {
    //#[serde(rename = "ActivationGroup", default)]
    //pub activation_group: HashMap<Name, ActivationGroup>,

}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroup {
    #[serde(rename = "Name")]
    pub name: String,
}

#[cfg(test)]
pub(crate) mod activation_groups_test {
    use crate::gdtf_v_1::attribute_definitions::activation_groups::{ActivationGroup, ActivationGroups};
    use crate::gdtf_v_1::attribute_definitions::AttributeDefinitions;
    use crate::GdtfError;

    pub(crate) fn test_acme_ae_610_beam(ag: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(ad: &Option<ActivationGroups>) -> Result<(), GdtfError> {
        return Ok(());
    }
}