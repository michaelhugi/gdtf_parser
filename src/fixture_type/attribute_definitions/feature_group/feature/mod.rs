//! # Definition of Feature
//! groups the Fixture Type Attributes into a structured way for easier access and search
//!
use quick_xml::events::BytesStart;

use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::utils::deparse::{DeparsePrimaryKey, DeparseSingle};
#[cfg(test)]
use crate::utils::deparse::TestDeparsePrimaryKey;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

///Feature only contains one feature Name, so only this primary keys are stored in a vec in FeatureGroup
pub struct Feature {}

impl DeparsePrimaryKey for Feature {
    type Error = GdtfError;
    type PrimaryKey = Name;

    const NODE_NAME: &'static [u8] = b"Feature";
    const PARENT_NODE_NAME: &'static [u8] = FeatureGroup::NODE_NAME;

    fn read_primary_key_from_event(event: BytesStart<'_>) -> Result<Name, GdtfError> {
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == b"Name" {
                return Ok(Name::new_from_attr(attr)?);
            }
        }
        Ok(Default::default())
    }
}

#[cfg(test)]
impl TestDeparsePrimaryKey for Feature {}


#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature as T;
    use crate::utils::deparse::TestDeparsePrimaryKey;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::name::Name;

    #[test]
    fn test_read_primary_key() -> Result<(), GdtfError> {
        assert_eq!(feature_testdata(1), T::read_primary_key_from_xml(&feature_testdata_xml(1))?);
        assert_eq!(feature_testdata(2), T::read_primary_key_from_xml(&feature_testdata_xml(2))?);
        assert_eq!(feature_testdata(3), T::read_primary_key_from_xml(&feature_testdata_xml(3))?);
        assert_eq!(feature_testdata(4), T::read_primary_key_from_xml(&feature_testdata_xml(4))?);
        assert_eq!(feature_testdata(5), T::read_primary_key_from_xml(&feature_testdata_xml(5))?);
        assert_eq!(feature_testdata(6), T::read_primary_key_from_xml(&feature_testdata_xml(6))?);
        assert_eq!(feature_testdata(7), T::read_primary_key_from_xml(&feature_testdata_xml(7))?);
        Ok(())
    }

    #[test]
    fn test_read_primary_key_vec() -> Result<(), GdtfError> {
        assert_eq!(feature_testdata_vec(), T::read_vec_from_xml(&feature_teatdata_xml_group())?);
        assert_eq!(T::read_vec_from_xml(&feature_teatdata_xml_group_empty())?, vec![]);
        Ok(())
    }

    ///Returns 7 different feature names for testing
    pub(crate) fn feature_testdata(i: u8) -> Name {
        match i {
            1 => Name::new("Beam").unwrap(),
            2 => Name::new("Dimmer").unwrap(),
            3 => Name::new("Color").unwrap(),
            4 => Name::new("Indirect").unwrap(),
            5 => Name::new("PanTilt").unwrap(),
            6 => Name::new("Gobo").unwrap(),
            _ => Name::new("").unwrap()
        }
    }

    ///Returns a vec of names for testing
    pub(crate) fn feature_testdata_vec() -> Vec<Name> {
        vec![
            feature_testdata(1),
            feature_testdata(2),
            feature_testdata(3),
            feature_testdata(4),
            feature_testdata(5),
            feature_testdata(6),
            feature_testdata(7),
        ]
    }


    ///Returns 7 different feature xmls for testing
    pub(crate) fn feature_testdata_xml(i: u8) -> String {
        match i {
            1 => r#"<Feature Name="Beam"/>"#.to_string(),
            2 => r#"<Feature Name="Dimmer"/>"#.to_string(),
            3 => r#"<Feature Name="Color"/>"#.to_string(),
            4 => r#"<Feature Name="Indirect"/>"#.to_string(),
            5 => r#"<Feature Name="PanTilt"/>"#.to_string(),
            6 => r#"<Feature Name="Gobo"/>"#.to_string(),
            _ => r#"<Feature Name="""#.to_string()
        }
    }

    ///Returns an xml with 7 different Feature nodes inside one activationGroup
    pub(crate) fn feature_teatdata_xml_group() -> String {
        r#"
     <FeatureGroup Name="Color" Pretty="Color">
          <Feature Name="Beam"/>
          <Feature Name="Dimmer"/>
          <Feature Name="Color"/>
          <Feature Name="Indirect"/>
          <Feature Name="PanTilt"/>
          <Feature Name="Gobo"/>
          <Feature Name=""/>
     </FeatureGroup>
    "#.to_string()
    }

    ///Returns an xml with 7 different Feature nodes inside one activationGroup
    pub(crate) fn feature_teatdata_xml_group_empty() -> String {
        r#"
     <FeatureGroup Name="Color" Pretty="Color">
     </FeatureGroup>
    "#.to_string()
    }
}