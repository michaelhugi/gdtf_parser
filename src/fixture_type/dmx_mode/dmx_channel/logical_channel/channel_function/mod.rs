//! Contains ChannelFunction and it's children

use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attribute as XmlAttribute;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;
use crate::utils::units::node::{GdtfNodeError, Node};

pub mod channel_set;

///The Fixture Type Attribute is assinged to a Channel Function and defines the function of its DMX Range
#[derive(Debug, PartialEq, Clone)]
pub struct ChannelFunction {
    ///Link to attribute; Starting point is the attributes node_2. Default value: “NoFeature”.
    pub attribute: Attribute,
    ///The manufacturer’s original name of the attribute; Default: empty
    pub original_attribute: String,
    ///Start DMX value; The end DMX value is calculated as a DMXFrom of the next channel function – 1 or the maximum value of the DMX channel. Default value: "0/1".
    pub dmx_from: DmxValue,
    ///Default DMX value of channel function when activated by the control system.
    pub default: DmxValue,
    ///Physical start value; Default value: 0
    pub physical_from: f32,
    ///Physical end value; Default value: 1
    pub physical_to: f32,
    ///Time in seconds to move from min to max of the Channel Function; Default value: 0
    pub real_fade: f32,
    ///Time in seconds to accelerate from stop to maximum velocity; Default value: 0
    pub real_acceleration: f32,
    ///Optional link to wheel; Starting point: Wheel Collect
    pub wheel: Option<Node>,
    ///Optional link to emitter in the physical description; Starting point: Emitter Collect
    pub emitter: Option<Node>,
    ///Optional link to filter in the physical description; Starting point: Filter Collect
    pub filter: Option<Node>,
    ///Link to DMX Channel or Channel Function; Starting point DMX mode
    pub mode_master: Option<Node>,
    ///Only used together with ModeMaster; DMX start value; Default value: 0/1
    pub mode_from: Option<DmxValue>,
    ///Only used together with ModeMaster; DMX end value; Default value: 0/1
    pub mode_to: Option<DmxValue>,
    //A list of channel sets for the channel function
    pub channel_sets: HashMap<Name, ChannelSet>,
}

const DEFAULT_DMX_FROM: DmxValue = DmxValue {
    initial_value: 0,
    n: 1,
    is_byte_shifting: false,
};

const DEFAULT_DMX_DEFAULT: DmxValue = DmxValue {
    initial_value: 0,
    n: 1,
    is_byte_shifting: false,
};

impl DeparseSingle for ChannelFunction {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"ChannelFunction";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut attribute = Attribute::NoFeature;
        let mut original_attribute: String = String::new();
        let mut dmx_from: DmxValue = DEFAULT_DMX_FROM;
        let mut default: DmxValue = DEFAULT_DMX_DEFAULT;
        let mut physical_from: f32 = 0.;
        let mut physical_to: f32 = 0.;
        let mut real_fade: f32 = 0.;
        let mut real_acceleration: f32 = 0.;
        let mut wheel = None;
        let mut emitter = None;
        let mut filter = None;
        let mut mode_master = None;
        let mut mode_from: Option<DmxValue> = None;
        let mut mode_to: Option<DmxValue> = None;
        let mut channel_sets: HashMap<Name, ChannelSet> = HashMap::new();
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"Attribute" => attribute = Attribute::new_from_attr(attr)?,
                b"OriginalAttribute" => original_attribute = deparse::attr_to_string(&attr),
                b"DMXFrom" => dmx_from = DmxValue::new_from_attr(attr).unwrap_or(DEFAULT_DMX_FROM),
                b"Default" => default = DmxValue::new_from_attr(attr).unwrap_or(DEFAULT_DMX_DEFAULT),
                b"PhysicalFrom" => physical_from = deparse::attr_to_f32(&attr),
                b"PhysicalTo" => physical_to = deparse::attr_to_f32(&attr),
                b"RealFade" => real_fade = deparse::attr_to_f32(&attr),
                b"RealAcceleration" => real_acceleration = deparse::attr_to_f32(&attr),
                b"Wheel" => wheel = Node::new_from_attr(attr)?,
                b"Emitter" => emitter = Node::new_from_attr(attr)?,
                b"Filter" => filter = Node::new_from_attr(attr)?,
                b"ModeMaster" => mode_master = Node::new_from_attr(attr)?,
                b"ModeFrom" => mode_from = match DmxValue::new_from_attr(attr) {
                    Ok(val) => Some(val),
                    Err(_) => None
                },
                b"ModeTo" => mode_to = match DmxValue::new_from_attr(attr) {
                    Ok(val) => Some(val),
                    Err(_) => None
                },
                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"ChannelSet" {
                        let cs = ChannelSet::read_single_from_event(reader, e)?;
                        channel_sets.insert(cs.1.unwrap(), cs.0);
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

        Ok((ChannelFunction {
            attribute,
            original_attribute,
            dmx_from,
            default,
            physical_from,
            physical_to,
            real_fade,
            real_acceleration,
            wheel,
            emitter,
            filter,
            mode_master,
            mode_from,
            mode_to,
            channel_sets,
        }, Some(name)))
    }

}

#[cfg(test)]
impl TestDeparseSingle for ChannelFunction {}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Attribute
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


#[derive(Debug, PartialEq, Clone)]
///Node used in ChannelFunction.attribute. Link to attribute; Starting point is the attributes node. Default value: “NoFeature”.
pub enum Attribute {
    ///Used when a reference to a node is present
    Feature(Node),
    ///Used for special value NoFeature
    NoFeature,
}


impl Attribute {
    ///Parses a string defined in gdtf-xml-description to Attribute
    /// ```rust
    /// use gdtf_parser::utils::units::node::Node;
    /// use gdtf_parser::utils::units::name::Name;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::Attribute;
    ///
    /// assert_eq!(Attribute::new_from_str("NoFeature").unwrap(), Attribute::NoFeature);
    /// assert_eq!(Attribute::new_from_str("").unwrap(), Attribute::NoFeature);
    /// assert_eq!(Attribute::new_from_str("Name1").unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string())])));
    /// assert_eq!(Attribute::new_from_str("Name1.Name2").unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string()), Name("Name2".to_string())])));
    /// assert!(Attribute::new_from_str("Name with invalid char {").is_err());
    /// assert!(Attribute::new_from_str("Name with invalid char ȸ").is_err());
    /// ```
    pub fn new_from_str(value: &str) -> Result<Self, GdtfNodeError> {
        if value == "NoFeature" {
            Ok(Self::NoFeature)
        } else {
            match Node::new_from_str(value)? {
                None => Ok(Self::NoFeature),
                Some(value) => Ok(Self::Feature(value))
            }
        }
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to Attribute
    /// ```rust
    /// use gdtf_parser::utils::units::node::Node;
    /// use gdtf_parser::utils::units::name::Name;
    /// use quick_xml::events::attributes::Attribute as XmlAttribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::Attribute;
    ///
    /// assert_eq!(Attribute::new_from_attr(XmlAttribute{ key: &[], value: Cow::Borrowed(b"NoFeature")}).unwrap(), Attribute::NoFeature);
    /// assert_eq!(Attribute::new_from_attr(XmlAttribute{ key: &[], value: Cow::Borrowed(b"")}).unwrap(), Attribute::NoFeature);
    /// assert_eq!(Attribute::new_from_attr(XmlAttribute{ key: &[], value: Cow::Borrowed(b"Name1")}).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string())])));
    /// assert_eq!(Attribute::new_from_attr(XmlAttribute{ key: &[], value: Cow::Borrowed(b"Name1.Name2")}).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string()), Name("Name2".to_string())])));
    /// assert!(Attribute::new_from_attr(XmlAttribute{ key: &[], value: Cow::Borrowed(b"Name with invalid char {")}).is_err());
    /// ```
    pub fn new_from_attr(attr: XmlAttribute<'_>) -> Result<Self, GdtfNodeError> {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}

/// ```rust
/// use gdtf_parser::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::Attribute;
///
/// assert_eq!(Attribute::NoFeature, Default::default());
/// ```
impl Default for Attribute {
    fn default() -> Self {
        Attribute::NoFeature
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Attribute
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::{Attribute, ChannelFunction};
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        ChannelFunction {
            attribute: Attribute::new_from_str("ColorSub_M")?,
            original_attribute: "".to_string(),
            dmx_from: DmxValue::new_from_str("0/1")?,
            default: DmxValue::new_from_str("0/1")?,
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 0.000000,
            real_acceleration: 0.000000,
            wheel: None,
            emitter: None,
            filter: Node::new_from_str("Magenta")?,
            mode_master: Node::new_from_str("Base_ColorMacro1")?,
            mode_from: Some(DmxValue::new_from_str("0/1")?),
            mode_to: Some(DmxValue::new_from_str("0/1")?),
            channel_sets: testdata::vec_to_hash_map(vec![
                Name::new("min")?,
                Name::new("")?,
                Name::new("max")?,
            ], vec![
                ChannelSet {
                    dmx_from: DmxValue::new_from_str("0/1")?,
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    dmx_from: DmxValue::new_from_str("1/1")?,
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    dmx_from: DmxValue::new_from_str("255/1")?,
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
            ]),
        }.compare_to_primary_key_and_xml(Some(Name::new("Magenta")?),
                                         r#"
            <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
                <ChannelSet DMXFrom="0/1" Name="min" WheelSlotIndex="0"/>
                <ChannelSet DMXFrom="1/1" Name="" WheelSlotIndex="0"/>
                <ChannelSet DMXFrom="255/1" Name="max" WheelSlotIndex="0"/>
              </ChannelFunction>
            "#);
        Ok(())
    }

    #[test]
    fn test_max() -> Result<(), GdtfError> {
        ChannelFunction {
            attribute: Attribute::new_from_str("ColorSub_M")?,
            original_attribute: "orig".to_string(),
            dmx_from: DmxValue::new_from_str("0/1")?,
            default: DmxValue::new_from_str("0/1")?,
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: Node::new_from_str("Wheel1")?,
            emitter: Node::new_from_str("Emitter1")?,
            filter: Node::new_from_str("Magenta")?,
            mode_master: Node::new_from_str("Base_ColorMacro1")?,
            mode_from: Some(DmxValue::new_from_str("0/1")?),
            mode_to: Some(DmxValue::new_from_str("0/1")?),
            channel_sets: HashMap::new(),
        }.compare_to_primary_key_and_xml(Some(Name::new("Magenta")?),
                                         r#"
            <ChannelFunction Wheel="Wheel1" Emitter="Emitter1" Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#);
        Ok(())
    }

    #[test]
    fn test_min_1() -> Result<(), GdtfError> {
        ChannelFunction {
            attribute: Attribute::new_from_str("ColorSub_M")?,
            original_attribute: "orig".to_string(),
            dmx_from: DmxValue::new_from_str("0/1")?,
            default: DmxValue::new_from_str("0/1")?,
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: None,
            emitter: None,
            filter: None,
            mode_master: None,
            mode_from: None,
            mode_to: None,
            channel_sets: HashMap::new(),
        }.compare_to_primary_key_and_xml(Some(Name::new("Magenta")?),
                                         r#"
            <ChannelFunction Wheel="" Emitter="" Filter="" ModeFrom="" ModeMaster="" ModeTo=""  Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#);
        Ok(())
    }

    #[test]
    fn test_min_2() -> Result<(), GdtfError> {
        ChannelFunction {
            attribute: Attribute::new_from_str("ColorSub_M")?,
            original_attribute: "orig".to_string(),
            dmx_from: DmxValue::new_from_str("0/1")?,
            default: DmxValue::new_from_str("0/1")?,
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: None,
            emitter: None,
            filter: None,
            mode_master: None,
            mode_from: None,
            mode_to: None,
            channel_sets: HashMap::new(),
        }.compare_to_primary_key_and_xml(Some(Name::new("Magenta")?),
                                         r#"
            <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#);
        Ok(())
    }


    #[test]
    fn test_attribute_new_from_str() {
        assert_eq!(Attribute::new_from_str("NoFeature").unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_str("").unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_str("Name1").unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string())])));
        assert_eq!(Attribute::new_from_str("Name1.Name2").unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string()), Name("Name2".to_string())])));
        assert!(Attribute::new_from_str("Name with invalid char {").is_err());
        assert!(Attribute::new_from_str("Name with invalid char ȸ").is_err());
    }

    #[test]
    fn test_attribute_new_from_attr_owned() {
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_owned(b"NoFeature")).unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_owned(b"")).unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_owned(b"Name1")).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string())])));
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_owned(b"Name1.Name2")).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string()), Name("Name2".to_string())])));
        assert!(Attribute::new_from_attr(testdata::to_attr_owned(b"Name with invalid char {")).is_err());
    }

    #[test]
    fn test_attribute_new_from_attr_borrowed() {
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_borrowed(b"NoFeature")).unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_borrowed(b"")).unwrap(), Attribute::NoFeature);
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_borrowed(b"Name1")).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string())])));
        assert_eq!(Attribute::new_from_attr(testdata::to_attr_borrowed(b"Name1.Name2")).unwrap(), Attribute::Feature(Node(vec![Name("Name1".to_string()), Name("Name2".to_string())])));
        assert!(Attribute::new_from_attr(testdata::to_attr_borrowed(b"Name with invalid char {")).is_err());
    }

    #[test]
    fn test_attribute_default() {
        assert_eq!(Attribute::NoFeature, Default::default());
    }
}