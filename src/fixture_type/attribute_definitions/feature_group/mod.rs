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

pub mod feature;


#[derive(Debug, PartialEq, Clone)]
pub struct FeatureGroup {
    pub pretty: String,
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
                    if e.name() == b"Feature" {
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

impl DeparseHashMap for FeatureGroup {}

#[cfg(test)]
impl TestDeparseSingle for FeatureGroup {}

#[cfg(test)]
impl TestDeparseHashMap for FeatureGroup {
    const GROUP_NODE_NAME: &'static [u8] = b"FeatureGroups";
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;

    #[test]
    fn test_feature_group_no_child() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![],
        }.compare_to_primary_key_and_xml(Some(Name::new("Position")?),
                                         r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_no_child_min() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup Name="" Pretty="">
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_no_child_empty() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup/>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_one_child() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![Name::new("PanTilt")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("Position")?),
                                         r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_one_child_min() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new("")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_one_child_empty() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new("")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup>
              <Feature/>
              </FeatureGroup>"#,
        );
        Ok(())
    }


    #[test]
    fn test_feature_group_two_children() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![Name::new("PanTilt")?, Name::new("Other")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("Position")?),
                                         r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              <Feature Name="Other"/>
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_two_children_min() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new("")?, Name::new("")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              <Feature Name=""/>
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_two_children_empty() -> Result<(), GdtfError> {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new("")?, Name::new("")?],
        }.compare_to_primary_key_and_xml(Some(Name::new("")?),
                                         r#"<FeatureGroup>
              <Feature/>
              <Feature/>
              </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_list() -> Result<(), GdtfError> {
        FeatureGroup::compare_hash_maps(
            testdata::vec_to_hash_map(vec![Name::new("BeamG")?, Name::new("DimmerG")?], vec![
                FeatureGroup {
                    pretty: "BeamP".to_string(),
                    features: vec![Name::new("BeamF")?],
                },
                FeatureGroup {
                    pretty: "DimmerP".to_string(),
                    features: vec![Name::new("DimmerF")?],
                }
            ]),
            r#"<FeatureGroups>
                                <FeatureGroup Name="BeamG" Pretty="BeamP">
                                    <Feature Name="BeamF"/>
                                </FeatureGroup>
                                <FeatureGroup Name="DimmerG" Pretty="DimmerP">
                                    <Feature Name="DimmerF"/>
                                </FeatureGroup>"#,
        );
        Ok(())
    }


    #[test]
    fn test_feature_group_list_min() -> Result<(), GdtfError> {
        FeatureGroup::compare_hash_maps(
            testdata::vec_to_hash_map(vec![Name::new("")?, Name::new("")?], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new("")?],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new("")?],
                }
            ]),
            r#"<FeatureGroups>
                                <FeatureGroup Name="" Pretty="">
                                    <Feature Name=""/>
                                </FeatureGroup>
                                <FeatureGroup Name="" Pretty="">
                                    <Feature Name=""/>
                                </FeatureGroup>"#,
        );
        Ok(())
    }


    #[test]
    fn test_feature_group_list_empty() -> Result<(), GdtfError> {
        FeatureGroup::compare_hash_maps(
            testdata::vec_to_hash_map(vec![Name::new("")?, Name::new("")?], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new("")?],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new("")?],
                }
            ]),
            r#"<FeatureGroups>
                                <FeatureGroup >
                                    <Feature/>
                                </FeatureGroup>
                                <FeatureGroup>
                                    <Feature/>
                                </FeatureGroup>"#,
        );
        Ok(())
    }

    #[test]
    fn test_feature_group_list_faulty() {
        assert!(FeatureGroup::read_hash_map_from_xml(
            r#"<FeatureGroups>
                                FeatureGroup >
                                    <Feature/>
                                </FeatureGroup>
                                <FeatureGroup>
                                    <Feature/>
                                </FeatureGroup>"#
        ).is_err());
    }

    #[test]
    fn test_feature_group_faulty() {
        match FeatureGroup::read_single_from_xml(
            r#"FeatureGroup >
                     <Feature/>
                     </FeatureGroup>"#
        ) {
            Ok(_) => { panic!("test_feature_group_list_faulty should return an error"); }
            Err(_) => {}
        }
    }
}

