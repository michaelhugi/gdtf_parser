//! Defines the attribute definitions for the Fixture Type Attributes.
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::fixture_type::FixtureType;
use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::name::Name;

pub mod feature_group;
pub mod attribute;
pub(crate) mod activation_group;


#[derive(Debug, Clone, PartialEq, Default)]
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

impl ReadGdtf for AttributeDefinitions {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = Self;

    const NODE_NAME: &'static [u8] = b"AttributeDefinitions";
    const PARENT_NODE_NAME: &'static [u8] = FixtureType::NODE_NAME_DS;
    const PRIMARY_KEY_NAME: &'static [u8] = b"";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_primary_key_from_attr(_: quick_xml::events::attributes::Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed");
    }

    fn read_any_attribute(_: &mut Self::DataHolder, _: quick_xml::events::attributes::Attribute<'_>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read_any_child(data_holder: &mut Self::DataHolder, reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(), Self::Error> {
        match event.name() {
            FeatureGroup::PARENT_NODE_NAME => data_holder.feature_groups = FeatureGroup::read_hash_map_from_event(reader, event, has_children)?,
            Attribute::PARENT_NODE_NAME => data_holder.attributes = Attribute::read_hash_map_from_event(reader, event, has_children)?,
            ActivationGroup::PARENT_NODE_NAME => data_holder.activation_groups = ActivationGroup::read_primary_key_vec_from_event(reader, event, has_children)?,
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }
}


#[cfg(test)]
impl TestReadGdtf for AttributeDefinitions {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(Self { feature_groups: FeatureGroup::testdata_hash_map(), attributes: Attribute::testdata_hash_map(), activation_groups: ActivationGroup::testdata_primary_key_vec() })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: Attribute::testdata_hash_map(), activation_groups: ActivationGroup::testdata_primary_key_vec() })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: Attribute::testdata_hash_map(), activation_groups: ActivationGroup::testdata_primary_key_vec() })),
            (None, Some(Self { feature_groups: FeatureGroup::testdata_hash_map(), attributes: HashMap::new(), activation_groups: ActivationGroup::testdata_primary_key_vec() })),
            (None, Some(Self { feature_groups: FeatureGroup::testdata_hash_map(), attributes: HashMap::new(), activation_groups: ActivationGroup::testdata_primary_key_vec() })),
            (None, Some(Self { feature_groups: FeatureGroup::testdata_hash_map(), attributes: Attribute::testdata_hash_map(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: FeatureGroup::testdata_hash_map(), attributes: Attribute::testdata_hash_map(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: HashMap::new(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: HashMap::new(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: HashMap::new(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: HashMap::new(), activation_groups: vec![] })),
            (None, Some(Self { feature_groups: HashMap::new(), attributes: HashMap::new(), activation_groups: vec![] })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            format!(r#"<AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions>"#, FeatureGroup::testdata_xml(), Attribute::testdata_xml(), ActivationGroup::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups></FeatureGroups><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions>"#, Attribute::testdata_xml(), ActivationGroup::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups/><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions>"#, Attribute::testdata_xml(), ActivationGroup::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes></Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions>"#, FeatureGroup::testdata_xml(), ActivationGroup::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes/><ActivationGroups>{}</ActivationGroups></AttributeDefinitions>"#, FeatureGroup::testdata_xml(), ActivationGroup::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups></ActivationGroups></AttributeDefinitions>"#, FeatureGroup::testdata_xml(), Attribute::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups/></AttributeDefinitions>"#, FeatureGroup::testdata_xml(), Attribute::testdata_xml()),
            format!(r#"$<AttributeDefinitions><FeatureGroups></FeatureGroups><Attributes></Attributes><ActivationGroups></ActivationGroups></AttributeDefinitions>"#),
            format!(r#"$<AttributeDefinitions><FeatureGroups/><Attributes/><ActivationGroups/></AttributeDefinitions>"#),
            format!(r#"$<AttributeDefinitions><FeatureGroups/><Attributes/><ActivationGroups/></AttributeDefinitions>"#),
            format!(r#"$<AttributeDefinitions></AttributeDefinitions>"#),
            format!(r#"$<AttributeDefinitions/>"#)
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::AttributeDefinitions as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}