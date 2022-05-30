use serde::Deserialize;

use crate::gdtf_v_1_1::attribute_definitions::AttributeDefinitions;

mod attribute_definitions;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct GdtfV1_1 {
    #[serde(rename = "FixtureType")]
    pub fixture_type: FixtureType,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FixtureType {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "ShortName")]
    pub short_name: String,
    #[serde(rename = "LongName")]
    pub long_name: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "FixtureTypeID")]
    pub fixture_type_id: String,
    #[serde(rename = "Thumbnail")]
    pub thumbnail: String,
    #[serde(rename = "RefFT")]
    pub ref_ft:String,
    #[serde(rename = "CanHaveChildren")]
    pub can_have_children: String,

    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,
}

#[cfg(test)]
pub mod gdtfv_1_1_tests {
    use crate::{GdtfError, GdtfV1_0, GdtfV1_1};
    use crate::gdtf_v_1_1::attribute_definitions::attribute_definitions_test;

    pub(crate) fn test_jb_12_spot(gdtf: &GdtfV1_1) -> Result<(), GdtfError> {
        let ft = &gdtf.fixture_type;
        assert_eq!(ft.name, "P12 Spot HP".to_string());
        assert_eq!(ft.short_name,"P12SPHP".to_string());
        assert_eq!(ft.long_name,"P12 Spot HP".to_string());
        assert_eq!(ft.manufacturer,"JB-Lighting".to_string());
        assert_eq!(ft.description,"P12 Spot HP (High Power) 640W".to_string());
        assert_eq!(ft.fixture_type_id,"807DC00C-18D5-4133-B781-1A003FA988FA".to_string());
        assert_eq!(ft.thumbnail,"P12 dunkel".to_string());
        assert_eq!(ft.ref_ft,"".to_string());
        assert_eq!(ft.can_have_children,"Yes".to_string());

        attribute_definitions_test::test_jb_12_spot(&gdtf.fixture_type.attribute_definitions)?;
        return Ok(());
    }
}