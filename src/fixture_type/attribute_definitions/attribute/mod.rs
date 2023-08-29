//! The Defines a Fixture Type Attribute
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::color_cie::ColorCie;
#[cfg(test)]
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;
use crate::utils::units::physical_unit::PhysicalUnit;

///Describes a singular mutual exclusive control function
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// The pretty name of the attribute
    pub pretty: String,
    /// Optional link ot the activation group. `FixtureType.activation_groups`
    pub activation_group: Option<Node>,
    /// Link to the corresponding feature. `AttributeDefinitions.feature_groups`
    pub feature: Node,
    /// Optional link to the main attribute. `AttributeDefinitions.attributes`
    /// TODO Node for Attribute Name
    pub main_attribute: Option<Node>,
    /// The physical Unit of the attribute. `Units`
    pub physical_unit: PhysicalUnit,
    /// Optional: Defines the color for the attribute
    pub color: Option<ColorCie>,
}

#[derive(Default)]
pub(crate) struct AttributeDataHolder {
    /// The pretty name of the attribute
    pub pretty: Option<String>,
    /// Optional link ot the activation group. `FixtureType.activation_groups`
    pub activation_group: Option<Node>,
    /// Link to the corresponding feature. `AttributeDefinitions.feature_groups`
    pub feature: Option<Node>,
    /// Optional link to the main attribute. `AttributeDefinitions.attributes`
    /// TODO Node for Attribute Name
    pub main_attribute: Option<Node>,
    /// The physical Unit of the attribute. `Units`
    pub physical_unit: Option<PhysicalUnit>,
    /// Optional: Defines the color for the attribute
    pub color: Option<ColorCie>,
}

impl ReadGdtf for Attribute {
    type PrimaryKey = AttributeName;
    type Error = GdtfError;
    type DataHolder = AttributeDataHolder;

    const NODE_NAME: &'static [u8] = b"Attribute";
    const PARENT_NODE_NAME: &'static [u8] = b"Attributes";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_primary_key_from_attr(
        attr: quick_xml::events::attributes::Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(AttributeName::new_from_attr(attr)?))
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            pretty: data_holder.pretty.unwrap_or_else(|| "".to_string()),
            activation_group: data_holder.activation_group,
            feature: data_holder
                .feature
                .ok_or_else(|| Self::attribute_not_found(b"Feature"))?,
            main_attribute: data_holder.main_attribute,
            physical_unit: data_holder.physical_unit.unwrap_or(PhysicalUnit::None),
            color: data_holder.color,
        })
    }
    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: quick_xml::events::attributes::Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Pretty" => data_holder.pretty = read::attr_to_string_option(attr),
            b"ActivationGroup" => data_holder.activation_group = Node::new_from_attr(attr)?,
            b"Feature" => data_holder.feature = Node::new_from_attr(attr)?,
            b"MainAttribute" => data_holder.main_attribute = Node::new_from_attr(attr)?,
            b"PhysicalUnit" => data_holder.physical_unit = Some(PhysicalUnit::new_from_attr(attr)),
            b"Color" => data_holder.color = ColorCie::new_from_attr(attr).ok(),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        _: &mut Self::DataHolder,
        _: &mut Reader<&[u8]>,
        _: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
impl TestReadGdtf for Attribute {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                Some(AttributeName::Shutter_n_(1)),
                Some(Self {
                    feature: Node::new_from_str("Beam.Beam").unwrap().unwrap(),
                    pretty: "".to_string(),
                    activation_group: None,
                    color: None,
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                }),
            ),
            (
                Some(AttributeName::Dimmer),
                Some(Self {
                    feature: Node::new_from_str("Dimmer.Dimmer").unwrap().unwrap(),
                    pretty: "Dim".to_string(),
                    activation_group: None,
                    color: None,
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                }),
            ),
            (
                Some(AttributeName::Color_n_(1)),
                Some(Self {
                    pretty: "".to_string(),
                    activation_group: None,
                    feature: Node::new_from_str("Color.Color").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    color: None,
                }),
            ),
            (
                Some(AttributeName::Pan),
                Some(Self {
                    feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::Angle,
                    pretty: "P".to_string(),
                    activation_group: Node::new_from_str("PanTilt").unwrap(),
                    color: None,
                }),
            ),
            (
                Some(AttributeName::Tilt),
                Some(Self {
                    activation_group: Node::new_from_str("PanTilt").unwrap(),
                    feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    pretty: "T".to_string(),
                    color: None,
                }),
            ),
            (
                Some(AttributeName::UserDefined(
                    Name::new("Something Else").unwrap(),
                )),
                Some(Self {
                    activation_group: Node::new_from_str("PanTilt").unwrap(),
                    feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    pretty: "T".to_string(),
                    color: None,
                }),
            ),
            (
                Some(AttributeName::Gobo_n_(1)),
                Some(Self {
                    feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    pretty: "G1".to_string(),
                    activation_group: None,
                    color: None,
                }),
            ),
            (
                Some(AttributeName::Gobo_n_SelectShake(1)),
                Some(Self {
                    activation_group: Node::new_from_str("Gobo1").unwrap(),
                    feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(),
                    main_attribute: Node::new_from_str("Gobo1").unwrap(),
                    physical_unit: PhysicalUnit::Frequency,
                    pretty: "Select Shake".to_string(),
                    color: None,
                }),
            ),
            (
                Some(AttributeName::Gobo_n_WheelSpin(2)),
                Some(Self {
                    activation_group: Node::new_from_str("Gobo1").unwrap(),
                    feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::AngularSpeed,
                    pretty: "Wheel Spin".to_string(),
                    color: None,
                }),
            ),
            (
                Some(AttributeName::UserDefined(Name::new("Reserved").unwrap())),
                Some(Self {
                    color: Some(ColorCie {
                        x: 0.312700,
                        y: 0.329,
                        Y: 100.0,
                    }),
                    feature: Node::new_from_str("Control.Control").unwrap().unwrap(),
                    main_attribute: None,
                    physical_unit: PhysicalUnit::None,
                    pretty: "Reserved".to_string(),
                    activation_group: None,
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Attribute Feature="Beam.Beam" Name="Shutter1" PhysicalUnit="None" Pretty=""/>"#.to_string(),
            r#"<Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="None" Pretty="Dim"/>"#.to_string(),
            r#"<Attribute Feature="Color.Color" Name="Color1" PhysicalUnit="None"/>"#.to_string(),
            r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>"#.to_string(),
            r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="" Pretty="T"/>"#.to_string(),
            r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Something Else" Pretty="T"/>"#.to_string(),
            r#"<Attribute ActivationGroup="" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>"#.to_string(),
            r#"<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>"#.to_string(),
            r#"<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="" Name="Gobo2WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>"#.to_string(),
            r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Reserved" PhysicalUnit="None" Pretty="Reserved"/>"#.to_string()
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![r#" <Attribute ActivationGroup="Gobo1" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>"#.to_string()]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::fixture_type::attribute_definitions::attribute::Attribute as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}
