use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::deparse::{DeparseSingle, DeparseVec};
use crate::errors::GdtfError;
use crate::errors::GdtfError::QuickXMLError;
use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;

pub mod feature_group;
pub mod attribute;


#[derive(Debug)]
pub struct AttributeDefinitions {
    feature_groups: Vec<FeatureGroup>,
    attributes: Vec<Attribute>,
}


impl DeparseSingle for AttributeDefinitions {
    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut feature_groups: Vec<FeatureGroup> = Vec::new();
        let mut attributes: Vec<Attribute> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if e.name() == b"FeatureGroups" {
                        feature_groups = FeatureGroup::vec_from_event(reader, e)?;
                    } else if e.name() == b"Attributes" {
                        attributes = Attribute::vec_from_event(reader, e)?;
                    } else {
                        tree_down += 1;
                    }
                }
                Ok(Event::End(_)) => {
                    tree_down -= 1;
                    if tree_down <= 0 { break; }
                }
                Ok(Event::Eof) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            }
        }
        buf.clear();
        Ok(AttributeDefinitions {
            feature_groups,
            attributes,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"AttributeDefinitions"
    }

    fn single_event_name() -> String {
        "AttributeDefinitions".to_string()
    }

    #[cfg(test)]
    fn is_single_eq(&self, other: &Self) -> bool {
        FeatureGroup::is_vec_eq(&self.feature_groups, &other.feature_groups) &&
            Attribute::is_vec_eq(&self.attributes, &other.attributes)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::deparse::DeparseSingle;
    use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::gdtf::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_some() {
        AttributeDefinitions {
            feature_groups: vec![
                FeatureGroup {
                    name: "PositionG".try_into().unwrap(),
                    pretty: "PositionP".to_string(),
                    features: vec![
                        Feature {
                            name: "PanTiltF".try_into().unwrap()
                        }
                    ],
                },
                FeatureGroup {
                    name: "GoboG".try_into().unwrap(),
                    pretty: "GoboP".to_string(),
                    features: vec![
                        Feature {
                            name: "GoboF".try_into().unwrap()
                        },
                        Feature {
                            name: "Gobo2F".try_into().unwrap()
                        }
                    ],
                }],
            attributes: vec![
                Attribute {
                    name: "Pan".try_into().unwrap(),
                    pretty: "P".to_string(),
                    activation_group: Some("PanTilt".to_string()),
                    feature: "Position.PanTilt".to_string(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::Angle,
                    color: None,
                },
                Attribute {
                    activation_group: Some("PanTilt".to_string()),
                    feature: "Position.PanTilt".to_string(),
                    name: "Tilt".try_into().unwrap(),
                    physical_unit: PhysicalUnit::Angle,
                    pretty: "T".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: Some("Gobo1".to_string()),
                    feature: "Gobo.Gobo".to_string(),
                    name: "Gobo1".try_into().unwrap(),
                    physical_unit: PhysicalUnit::None,
                    pretty: "G1".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ],
        }.test(
            r#"
    <AttributeDefinitions>
        <ActivationGroups>
            <ActivationGroup Name="PanTilt"/>
            <ActivationGroup Name="Gobo1"/>
        </ActivationGroups>
        <FeatureGroups>
            <FeatureGroup Name="PositionG" Pretty="PositionP">
                <Feature Name="PanTiltF"/>
            </FeatureGroup>
            <FeatureGroup Name="GoboG" Pretty="GoboP">
                <Feature Name="GoboF"/>
                <Feature Name="Gobo2F"/>
            </FeatureGroup>
        </FeatureGroups>
        <Attributes>
            <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>
            <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="Angle" Pretty="T"/>
            <Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>
        </Attributes>
     </AttributeDefinitions>
     "#
        );
    }
}