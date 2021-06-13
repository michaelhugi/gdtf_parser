//! # Definition of Feature
//! groups the Fixture Type Attributes into a structured way for easier access and search
//!
use std::fmt::Debug;

use quick_xml::Reader;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;

use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::utils::errors::GdtfError;
use crate::utils::read::{ReadGdtf, ReadGdtfDataHolder};
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

///Feature only contains one feature Name, so only this primary keys are stored in a vec in FeatureGroup
#[derive(Default, Debug, PartialEq)]
pub struct Feature {}

impl ReadGdtf<Feature> for Feature {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"Feature";
    const PARENT_NODE_NAME: &'static [u8] = FeatureGroup::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = true;

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

impl ReadGdtfDataHolder<Feature> for Feature {
    fn read_any_attribute(&mut self, _: Attribute<'_>) -> Result<(), <Feature as ReadGdtf<Self>>::Error> {
        panic!("Should not be executed");
    }

    fn read_any_child(&mut self, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), <Feature as ReadGdtf<Self>>::Error> {
        panic!("Should not be executed");
    }

    fn move_data(self) -> Result<Feature, <Feature as ReadGdtf<Self>>::Error> {
        panic!("Should not be executed");
    }
}

#[cfg(test)]
impl TestReadGdtf<Feature> for Feature {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("Beam").unwrap()), None),
            (Some(Name::new("Dimmer").unwrap()), None),
            (Some(Name::new("Color").unwrap()), None),
            (Some(Name::new("Indirect").unwrap()), None),
            (Some(Name::new("PanTilt").unwrap()), None),
            (Some(Name::new("Gobo").unwrap()), None),
            (Some(Name::new("").unwrap()), None)
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Feature Name="Beam"/>"#.to_string(),
            r#"<Feature Name="Dimmer"/>"#.to_string(),
            r#"<Feature Name="Color"/>"#.to_string(),
            r#"<Feature Name="Indirect"/>"#.to_string(),
            r#"<Feature Name="PanTilt"/>"#.to_string(),
            r#"<Feature Name="Gobo"/>"#.to_string(),
            r#"<Feature Name=""/>"#.to_string()
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}


#[cfg(test)]
pub mod tests {
    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}