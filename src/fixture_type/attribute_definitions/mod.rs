use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::utils::deparse::{DeparsePrimaryKey, DeparseSingle};
use crate::utils::deparse::DeparseHashMap;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXmlError;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::name::Name;

pub mod feature_group;
pub mod attribute;
pub mod activation_group;


#[derive(Debug, Clone)]
pub struct AttributeDefinitions {
    pub feature_groups: HashMap<Name, FeatureGroup>,
    pub attributes: HashMap<AttributeName, Attribute>,
    pub activation_groups: Vec<Name>,
}

impl PartialEq for AttributeDefinitions {
    fn eq(&self, other: &Self) -> bool {
        self.feature_groups == other.feature_groups &&
            self.attributes == other.attributes &&
            self.activation_groups == other.activation_groups
    }
}

impl DeparseSingle for AttributeDefinitions {
    type PrimaryKey = ();

    fn single_from_event(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut feature_groups: HashMap<Name, FeatureGroup> = HashMap::new();
        let mut attributes: HashMap<AttributeName, Attribute> = HashMap::new();
        let mut activation_groups: Vec<Name> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        b"FeatureGroups" => feature_groups = FeatureGroup::hash_map_from_event(reader, e)?,
                        b"Attributes" => attributes = Attribute::hash_map_from_event(reader, e)?,
                        b"ActivationGroups" => activation_groups = ActivationGroup::primary_key_vec_from_event(reader, e)?,
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
                Err(e) => return Err(QuickXmlError(e)),
                _ => {}
            }
        }
        buf.clear();
        Ok((AttributeDefinitions {
            feature_groups,
            attributes,
            activation_groups,
        }, None))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"AttributeDefinitions"
    }

    fn single_event_name() -> String {
        "AttributeDefinitions".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for AttributeDefinitions {}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::name::Name;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_some() -> Result<(), GdtfError> {
        AttributeDefinitions {
            feature_groups: testdata::vec_to_hash_map(vec![Name::new_unchecked("PositionG"), Name::new_unchecked("GoboG")], vec![
                FeatureGroup {
                    pretty: "PositionP".to_string(),
                    features: vec![Name::new_unchecked("PanTiltF")],
                },
                FeatureGroup {
                    pretty: "GoboP".to_string(),
                    features: vec![
                        Name::new_unchecked("GoboF"),
                        Name::new_unchecked("Gobo2F")
                    ],
                }]),
            attributes: testdata::vec_to_hash_map(vec![AttributeName::Pan, AttributeName::Tilt, AttributeName::Gobo_n_(1)], vec![
                Attribute {
                    pretty: "P".to_string(),
                    activation_group: Some("PanTilt".to_string()),
                    feature: "Position.PanTilt".try_into()?,
                    main_attribute: None,
                    physical_unit: PhysicalUnit::Angle,
                    color: None,
                },
                Attribute {
                    activation_group: Some("PanTilt".to_string()),
                    feature: "Position.PanTilt".try_into()?,
                    physical_unit: PhysicalUnit::Angle,
                    pretty: "T".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: Some("Gobo1".to_string()),
                    feature: "Gobo.Gobo".try_into()?,
                    physical_unit: PhysicalUnit::None,
                    pretty: "G1".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ]),
            activation_groups: vec![
                Name::new_unchecked("PanTilt"),
                Name::new_unchecked("Gobo1")
            ],
        }.test(None,
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
     "#,
        );
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        AttributeDefinitions {
            feature_groups: testdata::vec_to_hash_map(vec![Name::new_unchecked(""), Name::new_unchecked("")], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                }]),
            attributes: testdata::vec_to_hash_map(vec![AttributeName::UserDefined(Name::new("")?), AttributeName::UserDefined(Name::new("")?), AttributeName::UserDefined(Name::new("")?)], vec![
                Attribute {
                    pretty: "".to_string(),
                    activation_group: None,
                    feature: "".try_into()?,
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".try_into()?,

                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".try_into()?,
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ]),
            activation_groups: vec![
                Name::new_unchecked(""),
                Name::new_unchecked("")
            ],
        }.test(None,
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
     "#,
        );
        Ok(())
    }


    #[test]
    fn test_empty() -> Result<(), GdtfError> {
        AttributeDefinitions {
            feature_groups: testdata::vec_to_hash_map(vec![Name::new_unchecked(""), Name::new_unchecked("")], vec![
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                },
                FeatureGroup {
                    pretty: "".to_string(),
                    features: vec![Name::new_unchecked("")],
                }]),
            attributes: testdata::vec_to_hash_map(vec![AttributeName::UserDefined(Name::new("")?), AttributeName::UserDefined(Name::new("")?), AttributeName::UserDefined(Name::new("")?)], vec![
                Attribute {
                    pretty: "".to_string(),
                    activation_group: None,
                    feature: "".try_into()?,
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".try_into()?,
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                },
                Attribute {
                    activation_group: None,
                    feature: "".try_into()?,
                    physical_unit: PhysicalUnit::None,
                    pretty: "".to_string(),
                    main_attribute: None,
                    color: None,
                }
            ]),
            activation_groups: vec![
                Name::new_unchecked(""),
                Name::new_unchecked("")
            ],
        }.test(None,
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
     "#,
        );
        Ok(())
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