use serde::Deserialize;

use crate::gdtf_v_1_1::attribute_definitions::activation_groups::{ActivationGroup, ActivationGroups};

mod activation_groups;


#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct AttributeDefinitions {
    #[serde(rename = "ActivationGroups")]
    pub activation_groups: ActivationGroups,
}

#[cfg(test)]
pub(crate) mod attribute_definitions_test {
    use crate::gdtf_v_1_1::attribute_definitions::activation_groups::activation_groups_test;
    use crate::gdtf_v_1_1::attribute_definitions::AttributeDefinitions;
    use crate::GdtfError;

    pub(crate) fn test_jb_12_spot(ad: &AttributeDefinitions) -> Result<(), GdtfError> {
       // activation_groups_test::test_jb_12_spot(&ad.activation_groups)?;
        Ok(())
    }
}