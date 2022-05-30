use serde::{Deserialize, Deserializer};

use crate::utils::units::name::Name;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroups {
    //  #[serde(rename = "ActivationGroup")]
  //  pub activation_group: Vec<ActivationGroup>,

}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct ActivationGroup {
    // #[serde(rename = "Name")]
    // pub name: String,
}

#[cfg(test)]
pub(crate) mod activation_groups_test {
    use crate::gdtf_v_1_1::attribute_definitions::activation_groups::{ActivationGroup, ActivationGroups};
    use crate::gdtf_v_1_1::attribute_definitions::AttributeDefinitions;
    use crate::GdtfError;

    pub(crate) fn test_jb_12_spot(ag: &Vec<ActivationGroup>) -> Result<(), GdtfError> {
        //assert_eq!(ag.elements.len(), 4);
        return Ok(());
    }
}