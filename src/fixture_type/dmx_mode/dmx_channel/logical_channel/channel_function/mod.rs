//! Contains ChannelFunction and it's children

use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute as XmlAttribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
use crate::utils::deparse;
use crate::utils::deparse::{DeparseHashMap, DeparseSingle};
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;
use crate::utils::units::node::{GdtfNodeError, Node};

pub mod channel_set;

///The Fixture Type Attribute is assinged to a Channel Function and defines the function of its DMX Range
#[derive(Debug, Clone, PartialEq)]
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
    pub mode_master: Option<ModeMaster>,
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

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut attribute = Attribute::NoFeature;
        let mut original_attribute: String = String::new();
        let mut dmx_from: DmxValue = DEFAULT_DMX_FROM;
        let mut default: DmxValue = DEFAULT_DMX_DEFAULT;
        let mut physical_from: f32 = 0.;
        let mut physical_to: f32 = 1.;
        let mut real_fade: f32 = 0.;
        let mut real_acceleration: f32 = 0.;
        let mut wheel = None;
        let mut emitter = None;
        let mut filter = None;
        let mut mode_master = None;
        let mut mode_from: Option<DmxValue> = None;
        let mut mode_to: Option<DmxValue> = None;
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
        let channel_sets;
        if has_children {
            channel_sets = ChannelSet::read_hash_map_from_event(reader, event)?;
        } else {
            channel_sets = Default::default();
        }
        let mode_master = match mode_master {
            None => None,
            Some(node) => Some(ModeMaster::new(node, mode_from, mode_to))
        };

        Ok((Some(name), ChannelFunction {
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
            channel_sets,
        }))
    }
}

#[cfg(test)]
impl TestDeparseSingle for ChannelFunction {}

impl DeparseHashMap for ChannelFunction { const PARENT_NODE_NAME: &'static [u8] = b"LogicalChannel"; }

#[cfg(test)]
impl TestDeparseHashMap for ChannelFunction {}

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

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of ModeMaster
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
pub struct ModeMaster {
    ///Link to DMX Channel or Channel Function; Starting point DMX mode
    pub mode_master: Node,
    ///DMX start value; Default value: 0/1
    pub mode_from: DmxValue,
    ///DMX end value; Default value: 0/1
    pub mode_to: DmxValue,
}

impl ModeMaster {
    /// Creates a new instance of ModeMaster
    ///
    /// # Attributes
    ///
    /// * `mode_master` - Link to DMX Channel or Channel Function; Starting point DMX mode
    /// * `mode_from` - Dmx start value. If None is passed it will be replaced with Default value: 0/1
    /// * `mode_to` - Dmx end value. If None is passed it will be replaced with Default value: 0/1
    pub fn new(mode_master: Node, mode_from: Option<DmxValue>, mode_to: Option<DmxValue>) -> Self {
        let mode_from = mode_from.unwrap_or(
            DmxValue {
                initial_value: 0,
                n: 1,
                is_byte_shifting: false,
            }
        );
        let mode_to = mode_to.unwrap_or(
            DmxValue {
                initial_value: 0,
                n: 1,
                is_byte_shifting: false,
            }
        );

        Self {
            mode_master,
            mode_from,
            mode_to,
        }
    }
}


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of ModeMaster
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::{Attribute, ChannelFunction as T, ChannelFunction, ModeMaster};
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::errors::GdtfError;
    use crate::utils::read::TestReadSingle;
    use crate::utils::testdata;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_deparse_single() -> Result<(), GdtfError> {
        assert_eq!(channel_function_testdata(1), T::read_single_from_xml(&channel_function_testdata_xml(1))?);
        assert_eq!(channel_function_testdata(2), T::read_single_from_xml(&channel_function_testdata_xml(2))?);
        assert_eq!(channel_function_testdata(3), T::read_single_from_xml(&channel_function_testdata_xml(3))?);
        assert_eq!(channel_function_testdata(4), T::read_single_from_xml(&channel_function_testdata_xml(4))?);
        assert_eq!(channel_function_testdata(5), T::read_single_from_xml(&channel_function_testdata_xml(5))?);
        assert_eq!(channel_function_testdata(6), T::read_single_from_xml(&channel_function_testdata_xml(6))?);
        assert_eq!(channel_function_testdata(7), T::read_single_from_xml(&channel_function_testdata_xml(7))?);
        Ok(())
    }

    #[test]
    fn test_deparse_hash_map() -> Result<(), GdtfError> {
        assert_eq!(channel_function_testdata_hash_map(), T::read_hash_map_from_xml(&channel_function_testdata_xml_group(true))?);
        Ok(())
    }

    /// Returns 7 ChannelFunction instances for testing
    pub fn channel_function_testdata(i: u8) -> (Option<Name>, T) {
        match i {
            1 => (Some(Name::new("Reserved").unwrap()), T {
                attribute: Attribute::new_from_str("NoFeature").unwrap(),
                dmx_from: DmxValue { initial_value: 185, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 185, n: 1, is_byte_shifting: false },
                original_attribute: "".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 12.234101,
                real_fade: 0.000000,
                emitter: Node::new_from_str("Emitter1").unwrap(),
                filter: None,
                wheel: None,
                mode_master: None,
                channel_sets: HashMap::new(),
            }),

            2 => (Some(Name::new("Fade Wave Up").unwrap()), T {
                attribute: Attribute::new_from_str("Shutter1StrobeEffect").unwrap(),
                dmx_from: DmxValue { initial_value: 225, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 225, n: 1, is_byte_shifting: false },
                original_attribute: "".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 0.0,
                real_fade: 0.0,
                emitter: None,
                filter: None,
                wheel: None,
                mode_master: None,
                channel_sets: ChannelSet::testdata_hash_map(),
            }),

            3 => (Some(Name::new("Random Pixel").unwrap()), T {
                attribute: Attribute::new_from_str("").unwrap(),
                dmx_from: DmxValue { initial_value: 230, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 230, n: 1, is_byte_shifting: false },
                original_attribute: "".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 0.0,
                real_fade: 0.0,
                emitter: None,
                filter: None,
                wheel: None,
                mode_master: None,
                channel_sets: ChannelSet::testdata_hash_map(),
            }),

            4 => (Some(Name::new("Wave Up Down").unwrap()), T {
                attribute: Attribute::new_from_str("").unwrap(),
                dmx_from: DmxValue { initial_value: 235, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 0, n: 1, is_byte_shifting: false },
                original_attribute: "".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 0.0,
                real_fade: 0.0,
                emitter: None,
                filter: None,
                wheel: None,
                mode_master: Some(ModeMaster { mode_master: Node::new_from_str("Base_ColorMacro1").unwrap().unwrap(), mode_from: DmxValue { initial_value: 14, n: 1, is_byte_shifting: false }, mode_to: DmxValue { initial_value: 20, n: 1, is_byte_shifting: true } }),
                channel_sets: ChannelSet::testdata_hash_map(),
            }),

            5 => (Some(Name::new("Wave Up").unwrap()), T {
                attribute: Attribute::new_from_str("Shutter1StrobeEffect").unwrap(),
                dmx_from: DmxValue { initial_value: 240, n: 1, is_byte_shifting: true },
                default: DmxValue { initial_value: 0, n: 1, is_byte_shifting: false },
                original_attribute: "".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 0.0,
                real_fade: 58.000134,
                emitter: None,
                filter: Node::new_from_str("Magenta").unwrap(),
                wheel: None,
                mode_master: None,
                channel_sets: ChannelSet::testdata_hash_map(),
            }),

            6 => (Some(Name::new("Wave Down").unwrap()), T {
                attribute: Attribute::new_from_str("Shutter1StrobeEffect").unwrap(),
                dmx_from: DmxValue { initial_value: 245, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 245, n: 1, is_byte_shifting: false },
                original_attribute: "ShutStrEff".to_string(),
                physical_from: 0.0,
                physical_to: 1.0,
                real_acceleration: 0.0,
                real_fade: 0.0,
                emitter: None,
                filter: None,
                wheel: Node::new_from_str("Wheel1").unwrap(),
                mode_master: Some(ModeMaster { mode_master: Node::new_from_str("Base_ColorMacro1").unwrap().unwrap(), mode_from: DmxValue { initial_value: 0, n: 1, is_byte_shifting: false }, mode_to: DmxValue { initial_value: 0, n: 1, is_byte_shifting: false } }),
                channel_sets: ChannelSet::testdata_hash_map(),
            }),

            _ => (Some(Name::new("Open (2)").unwrap()), T {
                attribute: Attribute::new_from_str("Shutter1").unwrap(),
                dmx_from: DmxValue { initial_value: 0, n: 1, is_byte_shifting: false },
                default: DmxValue { initial_value: 250, n: 1, is_byte_shifting: true },
                original_attribute: "".to_string(),
                physical_from: -85.000012,
                physical_to: 70.000012,
                real_acceleration: 0.0,
                real_fade: 0.0,
                emitter: None,
                filter: None,
                wheel: None,
                mode_master: None,
                channel_sets: ChannelSet::testdata_hash_map(),
            }),
        }
    }

    /// Returns 7 xmls with ChannelFunction node for testing
    pub fn channel_function_testdata_xml(i: u8) -> String {
        match i {
            1 => format!(r#"<ChannelFunction Attribute="NoFeature" DMXFrom="185/1" Default="185/1" Name="Reserved" OriginalAttribute="" Emitter="Emitter1" PhysicalTo="1.000000" RealAcceleration="12.234101" RealFade="0.000000"/>"#),

            2 => format!(r#"<ChannelFunction Attribute="Shutter1StrobeEffect" DMXFrom="225/1" Default="225/1" Name="Fade Wave Up" OriginalAttribute="" PhysicalFrom="0.000000" RealAcceleration="0.000000" RealFade="0.000000">{}</ChannelFunction>"#, ChannelSet::testdata_xml()),

            3 => format!(r#"<ChannelFunction Attribute="" DMXFrom="230/1" Default="230/1" Name="Random Pixel" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">{}</ChannelFunction>"#, ChannelSet::testdata_xml()),

            4 => format!(r#"<ChannelFunction DMXFrom="235/1" Default="" Name="Wave Up Down" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" ModeMaster="Base_ColorMacro1" ModeFrom="14/1" ModeTo="20/1s">{}</ChannelFunction>"#,ChannelSet::testdata_xml()),

            5 => format!(r#"<ChannelFunction Attribute="Shutter1StrobeEffect" Filter="Magenta" DMXFrom="240/1s" Name="Wave Up" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="58.000134">{}</ChannelFunction>"#, ChannelSet::testdata_xml()),

            6 => format!(r#"<ChannelFunction Attribute="Shutter1StrobeEffect" Wheel="Wheel1" DMXFrom="245/1" Default="245/1" Name="Wave Down" OriginalAttribute="ShutStrEff" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealFade="0.000000"  ModeMaster="Base_ColorMacro1">{}</ChannelFunction>"#, ChannelSet::testdata_xml()),

            _ => format!(r#"<ChannelFunction Attribute="Shutter1" Default="250/1s" Name="Open (2)" OriginalAttribute="" PhysicalFrom="-85.000012" PhysicalTo="70.000015" RealAcceleration="0.000000" RealFade="0.000000">{}</ChannelFunction>"#, ChannelSet::testdata_xml()),
        }
    }

    ///Returns a HashMap with 7 ChannelFunction instances for testing
    pub fn channel_function_testdata_hash_map() -> HashMap<Name, ChannelFunction> {
        let mut map = HashMap::new();
        map.insert(channel_function_testdata(1).0.unwrap(), channel_function_testdata(1).1);
        map.insert(channel_function_testdata(2).0.unwrap(), channel_function_testdata(2).1);
        map.insert(channel_function_testdata(3).0.unwrap(), channel_function_testdata(3).1);
        map.insert(channel_function_testdata(4).0.unwrap(), channel_function_testdata(4).1);
        map.insert(channel_function_testdata(5).0.unwrap(), channel_function_testdata(5).1);
        map.insert(channel_function_testdata(6).0.unwrap(), channel_function_testdata(6).1);
        map.insert(channel_function_testdata(7).0.unwrap(), channel_function_testdata(7).1);
        map
    }

    ///Returns an xml with 7 ChannelFunction for testing. Depending on the agrument provided they are wrapped in a LogicalChannel node
    pub fn channel_function_testdata_xml_group(add_parent_node: bool) -> String {
        let parent_node_start = if add_parent_node { r#"<LogicalChannel Attribute="Control1" DMXChangeTimeLimit="0.000000" Master="None" MibFade="0.000000" Snap="No">"# } else { "" };
        let parent_node_end = if add_parent_node { "</LogicalChannel>" } else { "" };

        let out = format!("{}{}{}{}{}{}{}{}{}",
                          parent_node_start,
                          channel_function_testdata_xml(1),
                          channel_function_testdata_xml(2),
                          channel_function_testdata_xml(3),
                          channel_function_testdata_xml(4),
                          channel_function_testdata_xml(5),
                          channel_function_testdata_xml(6),
                          channel_function_testdata_xml(7),
                          parent_node_end
        );
        out
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

    #[test]
    fn test_mode_master_new() -> Result<(), GdtfError> {
        assert_eq!(
            ModeMaster::new(Node::new_from_str("Name")?.unwrap(), None, None),
            ModeMaster {
                mode_master: Node(vec![Name("Name".to_string())]),
                mode_from: DmxValue {
                    initial_value: 0,
                    n: 1,
                    is_byte_shifting: false,
                },
                mode_to: DmxValue {
                    initial_value: 0,
                    n: 1,
                    is_byte_shifting: false,
                },
            }
        );

        assert_eq!(
            ModeMaster::new(Node::new_from_str("Name")?.unwrap(), Some(DmxValue { initial_value: 13, n: 2, is_byte_shifting: true }), None),
            ModeMaster {
                mode_master: Node(vec![Name("Name".to_string())]),
                mode_from: DmxValue {
                    initial_value: 13,
                    n: 2,
                    is_byte_shifting: true,
                },
                mode_to: DmxValue {
                    initial_value: 0,
                    n: 1,
                    is_byte_shifting: false,
                },
            }
        );

        assert_eq!(
            ModeMaster::new(Node::new_from_str("Name")?.unwrap(), None, Some(DmxValue { initial_value: 13, n: 2, is_byte_shifting: true })),
            ModeMaster {
                mode_master: Node(vec![Name("Name".to_string())]),
                mode_to: DmxValue {
                    initial_value: 13,
                    n: 2,
                    is_byte_shifting: true,
                },
                mode_from: DmxValue {
                    initial_value: 0,
                    n: 1,
                    is_byte_shifting: false,
                },
            }
        );

        assert_eq!(
            ModeMaster::new(Node::new_from_str("Name")?.unwrap(), Some(DmxValue { initial_value: 22, n: 3, is_byte_shifting: false }), Some(DmxValue { initial_value: 13, n: 2, is_byte_shifting: true })),
            ModeMaster {
                mode_master: Node(vec![Name("Name".to_string())]),
                mode_to: DmxValue {
                    initial_value: 13,
                    n: 2,
                    is_byte_shifting: true,
                },
                mode_from: DmxValue {
                    initial_value: 22,
                    n: 3,
                    is_byte_shifting: false,
                },
            }
        );

        Ok(())
    }
}