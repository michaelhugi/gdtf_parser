//! # Definition of ActivationGroup
//! Attributes which need to be activated together to gain control over one logical function
//! Note 1 to entry: As example Pan & Tilt are paired to gain control over Position.

use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

///ActivationGroup only contains one attribute Name, so only this primary keys are stored in a vec in AttributeDefinitions
#[derive(Debug, PartialEq, Default)]
pub struct ActivationGroup {}

impl ReadGdtf for ActivationGroup {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = Self;

    const NODE_NAME: &'static [u8] = b"ActivationGroup";
    const PARENT_NODE_NAME: &'static [u8] = b"ActivationGroups";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = true;

    fn read_primary_key_from_attr(
        attr: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
    fn read_any_attribute(_: &mut Self::DataHolder, _: Attribute<'_>) -> Result<(), Self::Error> {
        panic!("Should not be used!");
    }

    fn read_any_child(
        _: &mut Self::DataHolder,
        _: &mut Reader<&[u8]>,
        _: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        panic!("Should not be used!");
    }

    fn move_data(_: Self::DataHolder) -> Result<Self, Self::Error> {
        panic!("Should not be used!");
    }
}

#[cfg(test)]
impl TestReadGdtf for ActivationGroup {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("ColorRGB").unwrap()), None),
            (Some(Name::new("PanTilt").unwrap()), None),
            (Some(Name::new("Gobo1").unwrap()), None),
            (Some(Name::new("ColorIndirect").unwrap()), None),
            (Some(Name::new("Gobo2").unwrap()), None),
            (Some(Name::new("Prism").unwrap()), None),
            (Some(Name::new("").unwrap()), None),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<ActivationGroup Name="ColorRGB"/>"#.to_string(),
            r#"<ActivationGroup Name="PanTilt"/>"#.to_string(),
            r#"<ActivationGroup Name="Gobo1"/>"#.to_string(),
            r#"<ActivationGroup Name="ColorIndirect"/>"#.to_string(),
            r#"<ActivationGroup Name="Gobo2"/>"#.to_string(),
            r#"<ActivationGroup Name="Prism"/>"#.to_string(),
            r#"<ActivationGroup Name=""/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}
