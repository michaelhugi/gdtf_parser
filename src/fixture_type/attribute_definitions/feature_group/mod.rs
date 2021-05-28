use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
#[cfg(test)]
use crate::utils::deparse::TestDeparseVec;
use crate::utils::units::name::Name;

pub mod feature;


#[derive(Debug)]
pub struct FeatureGroup {
    pub name: Name,
    pub pretty: String,
    pub features: Vec<Feature>,
}

impl DeparseSingle for FeatureGroup {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
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
        let mut features: Vec<Feature> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"Feature" {
                        features.push(Feature::single_from_event(reader, e)?);
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

        Ok(FeatureGroup {
            name,
            pretty,
            features,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroup"
    }

    fn single_event_name() -> String {
        "FeatureGroup".to_string()
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for FeatureGroup {
    fn is_eq_allow_empty_impl(&self, other: &Self,log:bool) -> bool {
        self.name.is_eq_allow_empty(&other.name,log) &&
            self.pretty == other.pretty &&
            Feature::is_vec_eq_unordered(&self.features, &other.features)
    }
}

#[cfg(test)]
impl TestDeparseSingle for FeatureGroup {
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty(&compare.name,false)
    }
}

#[cfg(test)]
impl TestDeparseVec for FeatureGroup {}

impl DeparseVec for FeatureGroup {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroups"
    }

}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::utils::deparse::TestDeparseVec;
    use crate::utils::deparse::TestDeparseSingle;

    #[test]
    fn test_feature_group_no_child() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            name: "Position".try_into().unwrap(),
            features: vec![],
        }.test(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_no_child_min() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![],
        }.test(
            r#"<FeatureGroup Name="" Pretty="">
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_no_child_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![],
        }.test(
            r#"<FeatureGroup/>"#
        );
    }

    #[test]
    fn test_feature_group_one_child() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            name: "Position".try_into().unwrap(),
            features: vec![Feature { name: "PanTilt".try_into().unwrap() }],
        }.test(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_one_child_min() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![Feature { name: "".try_into().unwrap() }],
        }.test(
            r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_one_child_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![Feature { name: "".try_into().unwrap() }],
        }.test(
            r#"<FeatureGroup>
              <Feature/>
              </FeatureGroup>"#
        );
    }


    #[test]
    fn test_feature_group_two_children() {
        FeatureGroup {
            pretty: "PositionPretty".to_string(),
            name: "Position".try_into().unwrap(),
            features: vec![
                Feature { name: "PanTilt".try_into().unwrap() },
                Feature { name: "Other".try_into().unwrap() }
            ],
        }.test(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              <Feature Name="Other"/>
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_two_children_min() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![
                Feature { name: "".try_into().unwrap() },
                Feature { name: "".try_into().unwrap() }
            ],
        }.test(
            r#"<FeatureGroup Name="" Pretty="">
              <Feature Name=""/>
              <Feature Name=""/>
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_two_children_empty() {
        FeatureGroup {
            pretty: "".to_string(),
            name: "".try_into().unwrap(),
            features: vec![
                Feature { name: "".try_into().unwrap() },
                Feature { name: "".try_into().unwrap() }
            ],
        }.test(
            r#"<FeatureGroup>
              <Feature/>
              <Feature/>
              </FeatureGroup>"#
        );
    }

    #[test]
    fn test_feature_group_list() {
        FeatureGroup::test_group(
            vec![
                FeatureGroup {
                    name: "BeamG".try_into().unwrap(),
                    pretty: "BeamP".to_string(),
                    features: vec![Feature {
                        name: "BeamF".try_into().unwrap()
                    }],
                },
                FeatureGroup {
                    name: "DimmerG".try_into().unwrap(),
                    pretty: "DimmerP".to_string(),
                    features: vec![Feature {
                        name: "DimmerF".try_into().unwrap()
                    }],
                }
            ],
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
            vec![
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![Feature {
                        name: "".try_into().unwrap()
                    }],
                },
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![Feature {
                        name: "".try_into().unwrap()
                    }],
                }
            ],
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
            vec![
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![Feature {
                        name: "".try_into().unwrap()
                    }],
                },
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![Feature {
                        name: "".try_into().unwrap()
                    }],
                }
            ],
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
        match FeatureGroup::vec_from_xml(
            r#"<FeatureGroups>
                                FeatureGroup >
                                    <Feature/>
                                </FeatureGroup>
                                <FeatureGroup>
                                    <Feature/>
                                </FeatureGroup>"#
        ) {
            Ok(_) => { panic!("test_feature_group_list_faulty should return an error"); }
            Err(_) => {}
        }
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

