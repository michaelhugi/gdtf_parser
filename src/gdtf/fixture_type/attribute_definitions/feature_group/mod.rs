use std::borrow::Borrow;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::deparse::{Deparse, DeparseList};
use crate::errors::GdtfError;
use crate::errors::GdtfError::QuickXMLError;
use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;

pub mod feature;


#[derive(Debug)]
pub struct FeatureGroup {
    name: String,
    pretty: String,
    features: Vec<Feature>,
}

impl Deparse for FeatureGroup {
    fn from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name = String::new();
        let mut pretty = String::new();
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    name = std::str::from_utf8(attr.value.borrow())?.to_owned();
                }
                b"Pretty" => {
                    pretty = std::str::from_utf8(attr.value.borrow())?.to_owned();
                }
                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut features: Vec<Feature> = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if e.name() == b"Feature" {
                        let feature = Feature::from_event(reader, e)?;
                        features.push(feature);
                    }
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            }
        }

        if name == "" {
            return Err(GdtfError::RequiredValueNotFoundError("Name not found in FeatureGroup".to_string()));
        }
        if pretty == "" {
            return Err(GdtfError::RequiredValueNotFoundError("Pretty not found in FeatureGroup".to_string()));
        }
        Ok(new(name, pretty, features))
    }

    fn is_event_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroup"
    }

    fn event_name() -> String {
        "FeatureGroup".to_string()
    }
    #[cfg(test)]
    fn loose_eq_test(&self, other: &Self) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.pretty != other.pretty {
            return false;
        }
        if self.features.len() != other.features.len() {
            return false;
        }
        for f in &self.features {
            let mut b = false;
            for f2 in &other.features {
                if f2.loose_eq_test(&f) {
                    b = true;
                }
            }
            if !b {
                return false;
            }
        }
        return true;
    }
}

impl DeparseList for FeatureGroup {
    fn is_event_group_name(event_name: &[u8]) -> bool {
        event_name == b"FeatureGroups"
    }

    fn event_group_name() -> String {
        "FeatureGroups".to_string()
    }
}

fn new(name: String, pretty: String, features: Vec<Feature>) -> FeatureGroup {
    FeatureGroup {
        name,
        pretty,
        features,
    }
}

#[cfg(test)]
mod tests {
    use quick_xml::events::Event;
    use quick_xml::Reader;

    use crate::deparse::{Deparse, DeparseList};
    use crate::errors::GdtfError;
    use crate::errors::GdtfError::QuickXMLError;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;

    #[test]
    fn test_feature_group_no_child() {
        FeatureGroup::from_xml(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              </FeatureGroup>"#
        ).expect("Unexpected error in test feature group").test_eq(
            &FeatureGroup {
                pretty: "PositionPretty".to_string(),
                name: "Position".to_string(),
                features: vec![],
            });
    }

    #[test]
    fn test_feature_group_one_child() {
        FeatureGroup::from_xml(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              </FeatureGroup>"#
        ).expect("Unexpected error in test feature group").test_eq(
            &FeatureGroup {
                pretty: "PositionPretty".to_string(),
                name: "Position".to_string(),
                features: vec![Feature { name: "PanTilt".to_string() }],
            });
    }

    #[test]
    fn test_feature_group_two_children() {
        FeatureGroup::from_xml(
            r#"<FeatureGroup Name="Position" Pretty="PositionPretty">
              <Feature Name="PanTilt"/>
              <Feature Name="Other"/>
              </FeatureGroup>"#
        ).expect("Unexpected error in test feature group").test_eq(
            &FeatureGroup {
                pretty: "PositionPretty".to_string(),
                name: "Position".to_string(),
                features: vec![
                    Feature { name: "PanTilt".to_string() },
                    Feature { name: "Other".to_string() }
                ],
            });
    }

    #[test]
    fn test_feature_group_list() {
        FeatureGroup::vec_test_eq(FeatureGroup::vec_from_xml(
            r#"<FeatureGroups>
                                <FeatureGroup Name="BeamG" Pretty="BeamP">
                                    <Feature Name="BeamF"/>
                                </FeatureGroup>
                                <FeatureGroup Name="DimmerG" Pretty="DimmerP">
                                    <Feature Name="DimmerF"/>
                                </FeatureGroup>"#
        ), vec![
            FeatureGroup {
                name: "BeamG".to_string(),
                pretty: "BeamP".to_string(),
                features: vec![Feature {
                    name: "BeamF".to_string()
                }],
            },
            FeatureGroup {
                name: "DimmerG".to_string(),
                pretty: "DimmerP".to_string(),
                features: vec![Feature {
                    name: "DimmerF".to_string()
                }],
            }
        ]);
    }

    impl FeatureGroup {
        pub fn from_reader(reader: &mut Reader<&[u8]>) -> Result<Self, GdtfError> {
            reader.trim_text(true);

            let mut buf: Vec<u8> = Vec::new();

            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                        if e.name() == b"FeatureGroup" {
                            return FeatureGroup::from_event(reader, e);
                        }
                    }
                    Ok(Event::Eof) | Ok(Event::End(_)) => {
                        break;
                    }
                    Err(e) => return Err(QuickXMLError(e)),
                    _ => {}
                };
                buf.clear();
            }

            Err(GdtfError::RequiredValueNotFoundError("Could not find FeatureGroup".to_string()))
        }

        pub fn from_xml(xml: &str) -> Result<Self, GdtfError> {
            let mut reader = Reader::from_str(xml);
            FeatureGroup::from_reader(&mut reader)
        }
    }
}

