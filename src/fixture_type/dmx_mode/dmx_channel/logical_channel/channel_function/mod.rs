//! Contains ChannelFunction and it's children

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;
use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;
use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;
use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;
use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;

pub mod channel_set;

///The Fixture Type Attribute is assinged to a Channel Function and defines the function of its DMX Range
#[derive(Debug, PartialEq, Clone)]
pub struct ChannelFunction {
    ///Link to attribute; Starting point is the attributes node_2. Default value: “NoFeature”.
    pub attribute: NodeChannelFunctionAttribute,
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
    pub wheel: NodeChannelFunctionWheel,
    ///Optional link to emitter in the physical description; Starting point: Emitter Collect
    pub emitter: NodeChannelFunctionEmitter,
    ///Optional link to filter in the physical description; Starting point: Filter Collect
    pub filter: NodeChannelFunctionFilter,
    ///Link to DMX Channel or Channel Function; Starting point DMX mode
    pub mode_master: NodeChannelFunctionModeMaster,
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

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut attribute: NodeChannelFunctionAttribute = Default::default();
        let mut original_attribute: String = String::new();
        let mut dmx_from: DmxValue = DEFAULT_DMX_FROM;
        let mut default: DmxValue = DEFAULT_DMX_DEFAULT;
        let mut physical_from: f32 = 0.;
        let mut physical_to: f32 = 0.;
        let mut real_fade: f32 = 0.;
        let mut real_acceleration: f32 = 0.;
        let mut wheel: NodeChannelFunctionWheel = Default::default();
        let mut emitter: NodeChannelFunctionEmitter = Default::default();
        let mut filter: NodeChannelFunctionFilter = Default::default();
        let mut mode_master: NodeChannelFunctionModeMaster = Default::default();
        let mut mode_from: Option<DmxValue> = None;
        let mut mode_to: Option<DmxValue> = None;
        let mut channel_sets: HashMap<Name, ChannelSet> = HashMap::new();
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Attribute" => attribute = attr.into(),
                b"OriginalAttribute" => original_attribute = deparse::attr_to_string(&attr),
                b"DMXFrom" => dmx_from = attr.try_into().unwrap_or( DEFAULT_DMX_FROM),
                b"Default" => default = attr.try_into().unwrap_or( DEFAULT_DMX_DEFAULT),
                b"PhysicalFrom" => physical_from = deparse::attr_to_f32(&attr),
                b"PhysicalTo" => physical_to = deparse::attr_to_f32(&attr),
                b"RealFade" => real_fade = deparse::attr_to_f32(&attr),
                b"RealAcceleration" => real_acceleration = deparse::attr_to_f32(&attr),
                b"Wheel" => wheel = attr.into(),
                b"Emitter" => emitter = attr.into(),
                b"Filter" => filter = attr.into(),
                b"ModeMaster" => mode_master = attr.into(),
                b"ModeFrom" => mode_from = match deparse::attr_to_str_option(&attr) {
                    None => None,
                    Some(v) => Some(v.try_into()?)
                },
                b"ModeTo" => mode_to = match deparse::attr_to_str_option(&attr) {
                    None => None,
                    Some(v) => Some(v.try_into()?)
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
                        let cs = ChannelSet::single_from_event(reader, e)?;
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

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"ChannelFunction"
    }

    fn single_event_name() -> String {
        "ChannelFunction".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for ChannelFunction {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
    use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;
    use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;
    use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        ChannelFunction {
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 0.000000,
            real_acceleration: 0.000000,
            wheel: NodeChannelFunctionWheel::none(),
            emitter: NodeChannelFunctionEmitter::none(),
            filter: NodeChannelFunctionFilter::new_from_str("Magenta")?,
            mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
            mode_from: Some("0/1".try_into().unwrap()),
            mode_to: Some("0/1".try_into().unwrap()),
            channel_sets: testdata::vec_to_hash_map(vec![
                Name::new_unchecked("min"),
                Name::new_unchecked(""),
                Name::new_unchecked("max"),
            ], vec![
                ChannelSet {
                    dmx_from: "0/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    dmx_from: "1/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    dmx_from: "255/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
            ]),
        }.test(Some(Name::new_unchecked("Magenta")),
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
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel::new_from_str("Wheel1")?,
            emitter: NodeChannelFunctionEmitter::new_from_str("Emitter1")?,
            filter: NodeChannelFunctionFilter::new_from_str("Magenta")?,
            mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
            mode_from: Some("0/1".try_into().unwrap()),
            mode_to: Some("0/1".try_into().unwrap()),
            channel_sets: HashMap::new(),
        }.test(Some(Name::new_unchecked("Magenta")),
               r#"
            <ChannelFunction Wheel="Wheel1" Emitter="Emitter1" Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#);
        Ok(())
    }

    #[test]
    fn test_min_1() {
        ChannelFunction {
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel::none(),
            emitter: NodeChannelFunctionEmitter::none(),
            filter: NodeChannelFunctionFilter::none(),
            mode_master: NodeChannelFunctionModeMaster::none(),
            mode_from: None,
            mode_to: None,
            channel_sets: HashMap::new(),
        }.test(Some(Name::new_unchecked("Magenta")),
               r#"
            <ChannelFunction Wheel="" Emitter="" Filter="" ModeFrom="" ModeMaster="" ModeTo=""  Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#)
    }

    #[test]
    fn test_min_2() {
        ChannelFunction {
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel::none(),
            emitter: NodeChannelFunctionEmitter::none(),
            filter: NodeChannelFunctionFilter::none(),
            mode_master: NodeChannelFunctionModeMaster::none(),
            mode_from: None,
            mode_to: None,
            channel_sets: HashMap::new(),
        }.test(Some(Name::new_unchecked("Magenta")),
               r#"
            <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#)
    }
}