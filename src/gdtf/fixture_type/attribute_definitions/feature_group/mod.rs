use std::borrow::Borrow;
use std::convert::TryInto;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::deparse::{DeparseSingle, DeparseVec};
use crate::errors::GdtfError;
use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;
use crate::units::name::Name;

pub mod feature;


#[derive(Debug)]
pub struct FeatureGroup {
    pub(crate) name: Name,
    pub(crate) pretty: String,
    pub(crate) features: Vec<Feature>,
}

impl DeparseSingle for FeatureGroup {
    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name = Name::new();
        let mut pretty = String::new();
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    name = std::str::from_utf8(attr.value.borrow())?.try_into()?;
                }
                b"Pretty" => {
                    pretty = std::str::from_utf8(attr.value.borrow())?.to_owned();
                }
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
                        let feature = Feature::single_from_event(reader, e)?;
                        features.push(feature);
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

        if name.is_empty() {
            return Err(GdtfError::RequiredValueNotFoundError("Name not found in FeatureGroup".to_string()));
        }
        if pretty == "" {
            return Err(GdtfError::RequiredValueNotFoundError("Pretty not found in FeatureGroup".to_string()));
        }
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
    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.pretty == other.pretty &&
            self.features.len() == other.features.len() &&
            Feature::is_vec_eq(&self.features, &other.features)
    }
}

impl DeparseVec for FeatureGroup {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroups"
    }

    fn group_event_name() -> String {
        "FeatureGroups".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::deparse::{DeparseSingle, DeparseVec};
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;

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
}

