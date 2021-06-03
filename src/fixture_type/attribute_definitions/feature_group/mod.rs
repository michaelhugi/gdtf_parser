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

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name = Default::default();
        let mut pretty = String::new();
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
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
                        features.push(Feature::primary_key_from_event(reader, e)?);
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

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroup"
    }

    fn single_event_name() -> String {
        "FeatureGroup".to_string()
    }
}

impl DeparseHashMap for FeatureGroup {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroups"
    }
}

#[cfg(test)]
impl TestDeparseSingle for FeatureGroup {}

#[cfg(test)]
impl TestDeparseHashMap for FeatureGroup {}

#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::testdata;
    use crate::utils::units::name::Name;

    #[test]
    fn test_feature_group_no_child() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![],
        }.test(Some(Name::new_unchecked("Position")),
               r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_no_child_min() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup Name="" Pretty="">
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_no_child_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup/>"#,
        );
    }

    #[test]
    fn test_feature_group_one_child() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![Name::new_unchecked("PanTilt")],
        }.test(Some(Name::new_unchecked("Position")),
               r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_one_child_min() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new_unchecked("")],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_one_child_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new_unchecked("")],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup>
              <Feature/>
              </FeatureGroup>"#,
        );
    }


    #[test]
    fn test_feature_group_two_children() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            features: vec![Name::new_unchecked("PanTilt"), Name::new_unchecked("Other")],
        }.test(Some(Name::new_unchecked("Position")),
               r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              <Feature Name="Other"/>
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_two_children_min() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new_unchecked(""), Name::new_unchecked("")],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              <Feature Name=""/>
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_two_children_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            features: vec![Name::new_unchecked(""), Name::new_unchecked("")],
        }.test(Some(Name::new_unchecked("")),
               r#"<FeatureGroup>
              <Feature/>
              <Feature/>
              </FeatureGroup>"#,
        );
    }

    #[test]
    fn test_feature_group_list() {
        FeatureGroup::test_group(
            testdata::vec_to_hash_map(vec![Name::new_unchecked("BeamG"), Name::new_unchecked("DimmerG")], vec![
                FeatureGroup {
                    pretty: "BeamP".to_string(),
                    features: vec![Name::new_unchecked("BeamF")],
                },
                FeatureGroup {
                    pretty: "DimmerP".to_string(),
                    features: vec![Name::new_unchecked("DimmerF")],
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
    }


    #[test]
    fn test_feature_group_list_min() {
        FeatureGroup::test_group(
            testdata::vec_to_hash_map(vec![Name::new_unchecked(""), Name::new_unchecked("")], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
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
    }


    #[test]
    fn test_feature_group_list_empty() {
        FeatureGroup::test_group(
            testdata::vec_to_hash_map(vec![Name::new_unchecked(""), Name::new_unchecked("")], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
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
    }

    #[test]
    fn test_feature_group_list_faulty() {
        assert!(FeatureGroup::hash_map_from_xml(
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
        match FeatureGroup::single_from_xml(
            r#"FeatureGroup >
                     <Feature/>
                     </FeatureGroup>"#
        ) {
            Ok(_) => { panic!("test_feature_group_list_faulty should return an error"); }
            Err(_) => {}
        }
    }
}

