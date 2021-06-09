//! Defines the attribute definitions for the Fixture Type Attributes.
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::utils::deparse::{DeparsePrimaryKey, DeparseSingle};
use crate::utils::deparse::DeparseHashMap;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXmlError;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::name::Name;

pub mod feature_group;
pub mod attribute;
pub(crate) mod activation_group;


#[derive(Debug, Clone, PartialEq)]
/// Defines the attribute definitions for the Fixture Type Attributes.
pub struct AttributeDefinitions {
    ///Describes the logical grouping of attributes. For example, Gobo 1 and Gobo 2 are grouped in the feature Gobo of the feature group Gobo.
    pub feature_groups: HashMap<Name, FeatureGroup>,
    ///List of Fixture Type Attributes that are used.
    pub attributes: HashMap<AttributeName, Attribute>,
    /// # Definition of ActivationGroup
    /// Attributes which need to be activated together to gain control over one logical function
    /// Note 1 to entry: As example Pan & Tilt are paired to gain control over Position.
    /// Defines which attributes are to be activated together. For example, Pan and Tilt are in the same activation group.
    pub activation_groups: Vec<Name>,
}

impl DeparseSingle for AttributeDefinitions {
    type PrimaryKey = ();
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"AttributeDefinitions";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut feature_groups: HashMap<Name, FeatureGroup> = HashMap::new();
        let mut attributes: HashMap<AttributeName, Attribute> = HashMap::new();
        let mut activation_groups: Vec<Name> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        FeatureGroup::PARENT_NODE_NAME => feature_groups = FeatureGroup::read_hash_map_from_event(reader)?,
                        Attribute::PARENT_NODE_NAME => attributes = Attribute::read_hash_map_from_event(reader)?,
                        ActivationGroup::PARENT_NODE_NAME => activation_groups = ActivationGroup::read_primary_key_vec_from_event(reader)?,
                        _ => { tree_down += 1; }
                    }
                }
                Ok(Event::End(_)) => {
                    tree_down -= 1;
                    if tree_down <= 0 { break; }
                }
                Ok(Event::Eof) => {
                    break;
                }
                Err(e) => return Err(QuickXmlError(e)),
                _ => {}
            }
        }
        buf.clear();
        Ok((None, AttributeDefinitions {
            feature_groups,
            attributes,
            activation_groups,
        }))
    }
}

#[cfg(test)]
impl TestDeparseSingle for AttributeDefinitions {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::attribute_definitions::activation_group::tests::{activation_group_testdata_vec, activation_group_testdata_xml_group};
    use crate::fixture_type::attribute_definitions::attribute::tests::{attribute_testdata_hash_map, attribute_testdata_xml_group};
    use crate::fixture_type::attribute_definitions::AttributeDefinitions as T;
    use crate::fixture_type::attribute_definitions::feature_group::tests::{feature_group_teatdata_xml_group, feature_group_testdata_hash_map};
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;

    #[test]
    fn test_deparse_single() -> Result<(), GdtfError> {
        assert_eq!(attribute_definitions_testdata(1), T::read_single_from_xml(&attribute_definitions_testdata_xml(1)).unwrap().1);
        assert_eq!(attribute_definitions_testdata(2), T::read_single_from_xml(&attribute_definitions_testdata_xml(2)).unwrap().1);
        assert_eq!(attribute_definitions_testdata(3), T::read_single_from_xml(&attribute_definitions_testdata_xml(3)).unwrap().1);
        assert_eq!(attribute_definitions_testdata(4), T::read_single_from_xml(&attribute_definitions_testdata_xml(4)).unwrap().1);
        assert_eq!(attribute_definitions_testdata(5), T::read_single_from_xml(&attribute_definitions_testdata_xml(5)).unwrap().1);
        Ok(())
    }

    ///Returns 5 different AttributeDefinitions for testing
    pub fn attribute_definitions_testdata(i: u8) -> T {
        match i {
            1 => T {
                feature_groups: feature_group_testdata_hash_map(),
                attributes: attribute_testdata_hash_map(),
                activation_groups: activation_group_testdata_vec(),
            },
            2 => T {
                feature_groups: HashMap::new(),
                attributes: attribute_testdata_hash_map(),
                activation_groups: activation_group_testdata_vec(),
            },
            3 => T {
                feature_groups: feature_group_testdata_hash_map(),
                attributes: HashMap::new(),
                activation_groups: activation_group_testdata_vec(),
            },
            4 => T {
                feature_groups: feature_group_testdata_hash_map(),
                attributes: attribute_testdata_hash_map(),
                activation_groups: vec![],
            },
            _ => T {
                feature_groups: HashMap::new(),
                attributes: HashMap::new(),
                activation_groups: vec![],
            },
        }
    }

    ///Returns 5 different xmls with AttributeDefinitions for testing
    pub fn attribute_definitions_testdata_xml(i: u8) -> String {
        match i {
            1 => format!(r#"$
            <AttributeDefinitions>
                {}
                {}
                {}
            </AttributeDefinitions>
            "#, feature_group_teatdata_xml_group(), attribute_testdata_xml_group(), activation_group_testdata_xml_group()),
            2 => format!(r#"$
            <AttributeDefinitions>
                <FeatureGroups></FeatureGroups>
                {}
                {}
            </AttributeDefinitions>
            "#, attribute_testdata_xml_group(), activation_group_testdata_xml_group()),
            3 => format!(r#"$
            <AttributeDefinitions>
                {}
                <Attributes></Attributes>
                {}
            </AttributeDefinitions>
            "#, feature_group_teatdata_xml_group(), activation_group_testdata_xml_group()),
            4 => format!(r#"$
            <AttributeDefinitions>
                {}
                {}
                "<ActivationGroups></ActivationGroups>
            </AttributeDefinitions>
            "#, feature_group_teatdata_xml_group(), attribute_testdata_xml_group()),
            _ => format!(r#"$
            <AttributeDefinitions>
                <FeatureGroups></FeatureGroups>
                <Attributes></Attributes>
                <ActivationGroups></ActivationGroups>
            </AttributeDefinitions>
            "#),
        }
    }
}