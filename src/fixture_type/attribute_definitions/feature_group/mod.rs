//! # Definition of Feature Groups
//! groups the logical control elements called Feature into a structured way for easier access and finding
//!

use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
use crate::utils::deparse::{DeparseHashMap, DeparsePrimaryKey, DeparseSingle};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub(crate) mod feature;


#[derive(Debug, PartialEq, Clone)]
///Groups the logical control elements called Feature into a structured way for easier access and finding
pub struct FeatureGroup {
    /// The pretty name of the feature group
    pub pretty: String,
    /// All Features that are grouped in this feature group
    pub features: Vec<Name>,
}

impl DeparseSingle for FeatureGroup {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"FeatureGroup";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name = Default::default();
        let mut pretty = String::new();
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"Pretty" => pretty = deparse::attr_to_string(&attr),
                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut features: Vec<Name> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Feature::NODE_NAME {
                        features.push(Feature::read_primary_key_from_event(e)?);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        buf.clear();

        Ok((FeatureGroup {
            pretty,
            features,
        }, Some(name)))
    }
}

impl DeparseHashMap for FeatureGroup {
    const PARENT_NODE_NAME: &'static [u8] = b"FeatureGroups";
}

#[cfg(test)]
impl TestDeparseSingle for FeatureGroup {}

#[cfg(test)]
impl TestDeparseHashMap for FeatureGroup {}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::attribute_definitions::feature_group::feature::tests::{feature_testdata, feature_testdata_xml};
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup as T;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::errors::GdtfError;
    use crate::utils::units::name::Name;

    #[test]
    fn test_deparse_single() -> Result<(), GdtfError> {
        assert_eq!(feature_group_testdata(1), T::read_single_from_xml(&feature_group_testdata_xml(1)).unwrap());
        assert_eq!(feature_group_testdata(2), T::read_single_from_xml(&feature_group_testdata_xml(2)).unwrap());
        assert_eq!(feature_group_testdata(3), T::read_single_from_xml(&feature_group_testdata_xml(3)).unwrap());
        assert_eq!(feature_group_testdata(4), T::read_single_from_xml(&feature_group_testdata_xml(4)).unwrap());
        assert_eq!(feature_group_testdata(5), T::read_single_from_xml(&feature_group_testdata_xml(5)).unwrap());
        assert_eq!(feature_group_testdata(6), T::read_single_from_xml(&feature_group_testdata_xml(6)).unwrap());
        Ok(())
    }

    #[test]
    fn test_deparse_hash_map() -> Result<(), GdtfError> {
        assert_eq!(feature_group_testdata_hash_map(), T::read_hash_map_from_xml(&feature_group_teatdata_xml_group()).unwrap());
        assert_eq!(T::read_hash_map_from_xml(&feature_group_teatdata_xml_group_empty()).unwrap(), HashMap::new());
        Ok(())
    }

    ///Returns a vec of names for testing
    pub(crate) fn feature_group_testdata_hash_map() -> HashMap<Name, T> {
        let mut map = HashMap::new();
        map.insert(feature_group_testdata(1).1.unwrap(), feature_group_testdata(1).0);
        map.insert(feature_group_testdata(2).1.unwrap(), feature_group_testdata(2).0);
        map.insert(feature_group_testdata(3).1.unwrap(), feature_group_testdata(3).0);
        map.insert(feature_group_testdata(4).1.unwrap(), feature_group_testdata(4).0);
        map.insert(feature_group_testdata(5).1.unwrap(), feature_group_testdata(5).0);
        map.insert(feature_group_testdata(6).1.unwrap(), feature_group_testdata(6).0);
        map.insert(feature_group_testdata(7).1.unwrap(), feature_group_testdata(7).0);
        map
    }


    ///Returns 6 different FeatureGroup xmls for testing
    pub(crate) fn feature_group_testdata_xml(i: u8) -> String {
        match i {
            1 => format!(r#"<FeatureGroup Name="Beam" Pretty="B">{}</FeatureGroup>"#, feature_testdata_xml(1)),
            2 => format!(r#"<FeatureGroup Name="Dimmer" Pretty="D">{}{}</FeatureGroup>"#, feature_testdata_xml(2), feature_testdata_xml(3)),
            3 => format!(r#"<FeatureGroup Name="Color" Pretty="C"></FeatureGroup>"#),
            4 => format!(r#"<FeatureGroup Name="" Pretty="P">{}</FeatureGroup>"#, feature_testdata_xml(4)),
            5 => format!(r#"<FeatureGroup Name="Focus" Pretty="">{}</FeatureGroup>"#, feature_testdata_xml(5)),
            _ => format!(r#"<FeatureGroup Name="Control" Pretty="Ctrl">{}</FeatureGroup>"#, feature_testdata_xml(7)),
        }
    }

    ///Returns 6 different FeatureGroups for testing
    pub(crate) fn feature_group_testdata(i: u8) -> (T, Option<Name>) {
        match i {
            1 => (T { pretty: "B".to_string(), features: vec![feature_testdata(1)] }, Some(Name::new("Beam").unwrap())),
            2 => (T { pretty: "D".to_string(), features: vec![feature_testdata(2), feature_testdata(3)] }, Some(Name::new("Dimmer").unwrap())),
            3 => (T { pretty: "C".to_string(), features: vec![] }, Some(Name::new("Color").unwrap())),
            4 => (T { pretty: "P".to_string(), features: vec![feature_testdata(4)] }, Some(Name::new("").unwrap())),
            5 => (T { pretty: "".to_string(), features: vec![feature_testdata(5)] }, Some(Name::new("Focus").unwrap())),
            _ => (T { pretty: "Ctrl".to_string(), features: vec![feature_testdata(7)] }, Some(Name::new("Control").unwrap())),
        }
    }


    ///Returns an xml with 7 different Feature nodes inside one FeatureGroup
    pub(crate) fn feature_group_teatdata_xml_group() -> String {
        let out = format!(r#"<FeatureGroups>
        {}
        {}
        {}
        {}
        {}
        {}
        {}
      </FeatureGroups>"#,
                          feature_group_testdata_xml(1),
                          feature_group_testdata_xml(2),
                          feature_group_testdata_xml(3),
                          feature_group_testdata_xml(4),
                          feature_group_testdata_xml(5),
                          feature_group_testdata_xml(6),
                          feature_group_testdata_xml(7),
        );
        out
    }

    ///Returns an xml with no nodes inside one FeatureGroup
    pub(crate) fn feature_group_teatdata_xml_group_empty() -> String {
        r#"
      <FeatureGroups>
      </FeatureGroups>
    "#.to_string()
    }
}

