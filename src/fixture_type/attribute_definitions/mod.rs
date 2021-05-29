use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXMLError;
#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;

pub mod feature_group;
pub mod attribute;
pub mod activation_group;


#[derive(Debug)]
pub struct AttributeDefinitions {
    pub feature_groups: Vec<FeatureGroup>,
    pub attributes: Vec<Attribute>,
    pub activation_groups: Vec<ActivationGroup>,
}


impl DeparseSingle for AttributeDefinitions {
    fn single_from_event(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut feature_groups: Vec<FeatureGroup> = Vec::new();
        let mut attributes: Vec<Attribute> = Vec::new();
        let mut activation_groups: Vec<ActivationGroup> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        b"FeatureGroups" => feature_groups = FeatureGroup::vec_from_event(reader, e)?,
                        b"Attributes" => attributes = Attribute::vec_from_event(reader, e)?,
                        b"ActivationGroups" => activation_groups = ActivationGroup::vec_from_event(reader, e)?,
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
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            }
        }
        buf.clear();
        Ok(AttributeDefinitions {
            feature_groups,
            attributes,
            activation_groups,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"AttributeDefinitions"
    }

    fn single_event_name() -> String {
        "AttributeDefinitions".to_string()
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for AttributeDefinitions {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        FeatureGroup::is_vec_eq_unordered(&self.feature_groups, &other.feature_groups, log) &&
            Attribute::is_vec_eq_unordered(&self.attributes, &other.attributes, log) &&
            ActivationGroup::is_vec_eq_unordered(&self.activation_groups, &other.activation_groups, log)
    }
}

#[cfg(test)]
impl TestDeparseSingle for AttributeDefinitions {
    fn is_same_item_identifier(&self, _: &Self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::units::physical_unit::PhysicalUnit;

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
            activation_groups: vec![
                ActivationGroup {
                    name: "PanTilt".try_into().unwrap()
                },
                ActivationGroup {
                    name: "Gobo1".try_into().unwrap()
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

    #[test]
    fn test_min() {
        AttributeDefinitions {
            feature_groups: vec![
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![
                        Feature {
                            name: "".try_into().unwrap()
                        }
                    ],
                },
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![
                        Feature {
                            name: "".try_into().unwrap()
                        }
                    ],
                }],
            attributes: vec![
                Attribute {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    activation_group: None,
                    feature: "".to_string(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".to_string(),
                    name: "".try_into().unwrap(),
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".to_string(),
                    name: "".try_into().unwrap(),
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ],
            activation_groups: vec![
                ActivationGroup {
                    name: "".try_into().unwrap()
                },
                ActivationGroup {
                    name: "".try_into().unwrap()
                }
            ],
        }.test(
            r#"
    <AttributeDefinitions>
        <ActivationGroups>
            <ActivationGroup Name=""/>
            <ActivationGroup Name=""/>
        </ActivationGroups>
        <FeatureGroups>
            <FeatureGroup Name="" Pretty="">
                <Feature Name=""/>
            </FeatureGroup>
            <FeatureGroup Name="" Pretty="">
                <Feature Name=""/>
            </FeatureGroup>
        </FeatureGroups>
        <Attributes>
            <Attribute ActivationGroup="" Feature="" Name="" PhysicalUnit="" Pretty=""/>
            <Attribute ActivationGroup="" Feature="" Name="" PhysicalUnit="" Pretty=""/>
            <Attribute ActivationGroup="" Feature="" Name="" PhysicalUnit="" Pretty=""/>
        </Attributes>
     </AttributeDefinitions>
     "#
        );
    }


    #[test]
    fn test_empty() {
        AttributeDefinitions {
            feature_groups: vec![
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![
                        Feature {
                            name: "".try_into().unwrap()
                        }
                    ],
                },
                FeatureGroup {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    features: vec![
                        Feature {
                            name: "".try_into().unwrap()
                        }
                    ],
                }],
            attributes: vec![
                Attribute {
                    name: "".try_into().unwrap(),
                    pretty: "".to_string(),
                    activation_group: None,
                    feature: "".to_string(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".to_string(),
                    name: "".try_into().unwrap(),
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".to_string(),
                    name: "".try_into().unwrap(),
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ],
            activation_groups: vec![
                ActivationGroup {
                    name: "".try_into().unwrap()
                },
                ActivationGroup {
                    name: "".try_into().unwrap()
                }
            ],
        }.test(
            r#"
    <AttributeDefinitions>
        <ActivationGroups>
            <ActivationGroup />
            <ActivationGroup />
        </ActivationGroups>
        <FeatureGroups>
            <FeatureGroup >
                <Feature />
            </FeatureGroup>
            <FeatureGroup  >
                <Feature />
            </FeatureGroup>
        </FeatureGroups>
        <Attributes>
            <Attribute />
            <Attribute />
            <Attribute  />
        </Attributes >
     </AttributeDefinitions>
     "#
        );
    }

    #[test]
    fn test_faulty() {
        match AttributeDefinitions::single_from_xml(
            r#"
    <AttributeDefinitions>
        <ActivationGroups>
            <ActivationGroup />
            <ActivationGroup />
        </ActivationGroups>
        <FeatureGroups>
            <FeatureGroup >
                <Feature />
            </FeatureGroup>
            <FeatureGroup  >
                <Feature />
            </FeatureGroup>
        </FeatureGroups>
        Attributes>
            <Attribute />
            <Attribte />
            <Attribute  />
        </Attributes >
     </AttributeDefinitions>
     "#
        )
        {
            Ok(_) => { panic!("test_faulty should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_faulty_child() {
        match AttributeDefinitions::single_from_xml(
            r#"
    <AttributeDefinitions>
        <ActivationGroups>
            <ActivationGroup />
            <ActivationGroup />
        </ActivationGroups>
        <FeatureGroups>
            <FeatureGroup >
                <Feature />
            </FeatureGroup>
            <FeatureGroup  >
                <Feature />
            </FeatureGroup>
        </FeatureGroups>
        <Attributes>
            <Attribute />
            <Attribte />
            <Attribute  />
        </Attributes >
     </AttributeDefinitions>
     "#
        )
        {
            Ok(_) => {}
            Err(_) => { panic!("test_faulty_child should not return an error"); }
        }
    }
}