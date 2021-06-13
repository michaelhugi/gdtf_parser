//! # Definition of Feature Groups
//! groups the logical control elements called Feature into a structured way for easier access and finding
//!

use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::{ReadGdtf, ReadGdtfDataHolder};
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

pub(crate) mod feature;


#[derive(Debug, PartialEq, Clone, Default)]
///Groups the logical control elements called Feature into a structured way for easier access and finding
pub struct FeatureGroup {
    /// The pretty name of the feature group
    pub pretty: String,
    /// All Features that are grouped in this feature group
    pub features: Vec<Name>,
}

impl ReadGdtf<FeatureGroup> for FeatureGroup {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"FeatureGroup";
    const PARENT_NODE_NAME: &'static [u8] = b"FeatureGroups";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

impl ReadGdtfDataHolder<FeatureGroup> for FeatureGroup {
    fn read_any_attribute(&mut self, attr: Attribute<'_>) -> Result<(), <FeatureGroup as ReadGdtf<Self>>::Error> {
        if attr.key == b"Pretty" {
            self.pretty = read::attr_to_string(attr);
        }
        Ok(())
    }

    fn read_any_child(&mut self, _: &mut Reader<&[u8]>, event: BytesStart<'_>, _: bool) -> Result<(), <FeatureGroup as ReadGdtf<Self>>::Error> {
        if event.name() == Feature::NODE_NAME {
            self.features.push(Feature::read_primary_key_from_event(event)?.unwrap_or_else(|| Name::new("").unwrap()));
        }
        Ok(())
    }

    fn move_data(self) -> Result<FeatureGroup, <FeatureGroup as ReadGdtf<Self>>::Error> {
        Ok(self)
    }
}


#[cfg(test)]
impl TestReadGdtf<FeatureGroup> for FeatureGroup {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("Beam").unwrap()), Some(Self { pretty: "B".to_string(), features: Feature::testdata_primary_key_vec() })),
            (Some(Name::new("Dimmer").unwrap()), Some(Self { pretty: "D".to_string(), features: Feature::testdata_primary_key_vec() })),
            (Some(Name::new("Color").unwrap()), Some(Self { pretty: "C".to_string(), features: vec![] })),
            (Some(Name::new("").unwrap()), Some(Self { pretty: "P".to_string(), features: vec![] })),
            (Some(Name::new("Focus").unwrap()), Some(Self { pretty: "".to_string(), features: Feature::testdata_primary_key_vec() })),
            (Some(Name::new("Control").unwrap()), Some(Self { pretty: "Ctrl".to_string(), features: Feature::testdata_primary_key_vec() })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            format!(r#"<FeatureGroup Name="Beam" Pretty="B">{}</FeatureGroup>"#, Feature::testdata_xml()),
            format!(r#"<FeatureGroup Name="Dimmer" Pretty="D">{}</FeatureGroup>"#, Feature::testdata_xml()),
            format!(r#"<FeatureGroup Name="Color" Pretty="C"/>"#),
            format!(r#"<FeatureGroup Name="" Pretty="P"></FeatureGroup>"#),
            format!(r#"<FeatureGroup Name="Focus" Pretty="">{}</FeatureGroup>"#, Feature::testdata_xml()),
            format!(r#"<FeatureGroup Name="Control" Pretty="Ctrl">{}</FeatureGroup>"#, Feature::testdata_xml()),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}

