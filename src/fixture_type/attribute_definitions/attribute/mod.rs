//! The Defines a Fixture Type Attribute
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::{DeparseHashMap, DeparseSingle, GdtfDeparseError};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::color_cie::ColorCie;
use crate::utils::units::node::Node;
use crate::utils::units::physical_unit::PhysicalUnit;

///Describes a singular mutual exclusive control function
#[derive(Debug, PartialEq, Clone)]
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

impl DeparseSingle for Attribute {
    /// Attribute ist stored in a HashMap in AttributeDefinitions. Therefore name is used as primary-key for the hashmap
    type PrimaryKey = AttributeName;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"Attribute";

    fn read_single_from_event(_reader: &mut Reader<&[u8]>, event: BytesStart<'_>, _: bool) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where
        Self: Sized {
        let mut name = Default::default();
        let mut pretty = String::new();
        let mut activation_group = None;
        let mut feature: Option<Node> = None;
        let mut main_attribute = None;
        let mut physical_unit: PhysicalUnit = PhysicalUnit::None;
        let mut color: Option<ColorCie> = None;

        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = AttributeName::new_from_attr(attr)?,
                b"Pretty" => pretty = deparse::attr_to_string(&attr),
                b"ActivationGroup" => activation_group = Node::new_from_attr(attr)?,
                b"Feature" => feature = Node::new_from_attr(attr)?,
                b"MainAttribute" => main_attribute = Node::new_from_attr(attr)?,
                b"PhysicalUnit" => physical_unit = PhysicalUnit::new_from_attr(attr),
                b"Color" => color = ColorCie::new_from_attr(attr).ok(),
                _ => {}
            }
        }
        if feature.is_none() {
            return Err(GdtfDeparseError::new_xml_attribute_not_found(Self::NODE_NAME, b"Feature").into());
        }
        let feature = feature.unwrap();

        Ok((Some(name), Attribute {
            feature,
            pretty,
            color,
            activation_group,
            main_attribute,
            physical_unit,
        }))
    }
}

impl DeparseHashMap for Attribute {
    const PARENT_NODE_NAME: &'static [u8] = b"Attributes";
}

#[cfg(test)]
impl TestDeparseSingle for Attribute {}

#[cfg(test)]
impl TestDeparseHashMap for Attribute {}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::attribute_definitions::attribute::Attribute as T;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::errors::GdtfError;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::color_cie::ColorCie;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_read_single() -> Result<(), GdtfError> {
        assert_eq!(attribute_testdata(1), T::read_single_from_xml(&attribute_testdata_xml(1)).unwrap());
        assert_eq!(attribute_testdata(2), T::read_single_from_xml(&attribute_testdata_xml(2)).unwrap());
        assert_eq!(attribute_testdata(3), T::read_single_from_xml(&attribute_testdata_xml(3)).unwrap());
        assert_eq!(attribute_testdata(4), T::read_single_from_xml(&attribute_testdata_xml(4)).unwrap());
        assert_eq!(attribute_testdata(5), T::read_single_from_xml(&attribute_testdata_xml(5)).unwrap());
        assert_eq!(attribute_testdata(6), T::read_single_from_xml(&attribute_testdata_xml(6)).unwrap());
        assert_eq!(attribute_testdata(7), T::read_single_from_xml(&attribute_testdata_xml(7)).unwrap());
        assert_eq!(attribute_testdata(8), T::read_single_from_xml(&attribute_testdata_xml(8)).unwrap());
        assert_eq!(attribute_testdata(9), T::read_single_from_xml(&attribute_testdata_xml(9)).unwrap());
        assert_eq!(attribute_testdata(10), T::read_single_from_xml(&attribute_testdata_xml(10)).unwrap());

        assert!(T::read_single_from_xml(&attribute_testdata_missing_feature_xml()).is_err());

        Ok(())
    }

    #[test]
    fn test_read_hash_map() -> Result<(), GdtfError> {
        let map = attribute_testdata_hash_map();
        assert!(map.len() == 10);
        assert_eq!(map, T::read_hash_map_from_xml(&attribute_testdata_xml_group()).unwrap());
        assert_eq!(T::read_hash_map_from_xml(&attribute_testdata_xml_group_empty()).unwrap(), HashMap::new());
        assert_eq!(T::read_hash_map_from_xml(
            r#"
                    <Attributes>
                        asfasdf
                    </Attributes>
                "#
        ).unwrap(), HashMap::new());

        Ok(())
    }


    ///Returns 10 different attributes for testing
    pub(crate) fn attribute_testdata(i: u8) -> (Option<AttributeName>, T) {
        match i {
            1 => (Some(AttributeName::Shutter_n_(1)), T { feature: Node::new_from_str("Beam.Beam").unwrap().unwrap(), pretty: "".to_string(), activation_group: None, color: None, main_attribute: None, physical_unit: PhysicalUnit::None }),
            2 => (Some(AttributeName::Dimmer), T { feature: Node::new_from_str("Dimmer.Dimmer").unwrap().unwrap(), pretty: "Dim".to_string(), activation_group: None, color: None, main_attribute: None, physical_unit: PhysicalUnit::None }),
            3 => (Some(AttributeName::Color_n_(1)), T { pretty: "".to_string(), activation_group: None, feature: Node::new_from_str("Color.Color").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::None, color: None }),
            4 => (Some(AttributeName::Pan), T { feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::Angle, pretty: "P".to_string(), activation_group: Node::new_from_str("PanTilt").unwrap(), color: None }),
            5 => (Some(AttributeName::Tilt), T { activation_group: Node::new_from_str("PanTilt").unwrap(), feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::None, pretty: "T".to_string(), color: None }),
            6 => (Some(AttributeName::UserDefined(Name::new("Something Else").unwrap())), T { activation_group: Node::new_from_str("PanTilt").unwrap(), feature: Node::new_from_str("Position.PanTilt").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::None, pretty: "T".to_string(), color: None }),
            7 => (Some(AttributeName::Gobo_n_(1)), T { feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::None, pretty: "G1".to_string(), activation_group: None, color: None }),
            8 => (Some(AttributeName::Gobo_n_SelectShake(1)), T { activation_group: Node::new_from_str("Gobo1").unwrap(), feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(), main_attribute: Node::new_from_str("Gobo1").unwrap(), physical_unit: PhysicalUnit::Frequency, pretty: "Select Shake".to_string(), color: None }),
            9 => (Some(AttributeName::Gobo_n_WheelSpin(2)), T { activation_group: Node::new_from_str("Gobo1").unwrap(), feature: Node::new_from_str("Gobo.Gobo").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::AngularSpeed, pretty: "Wheel Spin".to_string(), color: None }),
            _ => (Some(AttributeName::UserDefined(Name::new("Reserved").unwrap())), T { color: Some(ColorCie { x: 0.312700, y: 0.329, Y: 100.0 }), feature: Node::new_from_str("Control.Control").unwrap().unwrap(), main_attribute: None, physical_unit: PhysicalUnit::None, pretty: "Reserved".to_string(), activation_group: None })
        }
    }

    ///Returns a HashMap of attributes for testing
    pub(crate) fn attribute_testdata_hash_map() -> HashMap<AttributeName, T> {
        let mut map = HashMap::new();

        map.insert(attribute_testdata(1).0.unwrap(), attribute_testdata(1).1);
        map.insert(attribute_testdata(2).0.unwrap(), attribute_testdata(2).1);
        map.insert(attribute_testdata(3).0.unwrap(), attribute_testdata(3).1);
        map.insert(attribute_testdata(4).0.unwrap(), attribute_testdata(4).1);
        map.insert(attribute_testdata(5).0.unwrap(), attribute_testdata(5).1);
        map.insert(attribute_testdata(6).0.unwrap(), attribute_testdata(6).1);
        map.insert(attribute_testdata(7).0.unwrap(), attribute_testdata(7).1);
        map.insert(attribute_testdata(8).0.unwrap(), attribute_testdata(8).1);
        map.insert(attribute_testdata(9).0.unwrap(), attribute_testdata(9).1);
        map.insert(attribute_testdata(10).0.unwrap(), attribute_testdata(10).1);

        map
    }

    ///Returns 10 different Attribute xmls for testing
    pub(crate) fn attribute_testdata_xml(i: u8) -> String {
        match i {
            1 => r#"<Attribute Feature="Beam.Beam" Name="Shutter1" PhysicalUnit="None" Pretty=""/>"#.to_string(),
            2 => r#"<Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="None" Pretty="Dim"/>"#.to_string(),
            3 => r#"<Attribute Feature="Color.Color" Name="Color1" PhysicalUnit="None"/>"#.to_string(),
            4 => r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>"#.to_string(),
            5 => r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="" Pretty="T"/>"#.to_string(),
            6 => r#"<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Something Else" Pretty="T"/>"#.to_string(),
            7 => r#"<Attribute ActivationGroup="" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>"#.to_string(),
            8 => r#"<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>"#.to_string(),
            9 => r#"<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="" Name="Gobo2WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>"#.to_string(),
            _ => r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Reserved" PhysicalUnit="None" Pretty="Reserved"/>"#.to_string()
        }
    }

    ///Returns an xml Attribute with no Feature set
    pub(crate) fn attribute_testdata_missing_feature_xml() -> String {
        r#" <Attribute ActivationGroup="Gobo1" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>"#.to_string()
    }

    ///Returns an xml with 7 different Attribute nodes inside one Attributes
    pub(crate) fn attribute_testdata_xml_group() -> String {
        r#"
    <Attributes>
        "<Attribute Feature="Beam.Beam" Name="Shutter1" PhysicalUnit="None" Pretty=""/>
        <Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="None" Pretty="Dim"/>
        <Attribute Feature="Color.Color" Name="Color1" PhysicalUnit="None"/>
        <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>
        <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="" Pretty="T"/>
        <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Something Else" Pretty="T"/>
        <Attribute ActivationGroup="" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>
        <Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>
        <Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="" Name="Gobo2WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>
        <Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Reserved" PhysicalUnit="None" Pretty="Reserved"/>
    </Attributes>
    "#.to_string()
    }

    ///Returns an xml with nothing nodes inside one Attributes
    pub(crate) fn attribute_testdata_xml_group_empty() -> String {
        r#"
    <Attributes>
    </Attributes>
    "#.to_string()
    }
}
