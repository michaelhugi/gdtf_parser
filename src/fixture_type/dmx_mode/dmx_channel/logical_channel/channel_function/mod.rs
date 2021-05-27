//! Contains ChannelFunction and it's children

use std::convert::TryInto;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::test::assert_eq_allow_empty::AssertEqAllowEmpty;
use crate::utils::units::dmx_value::DMXValue;
use crate::utils::units::name::Name;
use crate::utils::units::node::node_channel_function_attribute::NodeChannelFunctionAttribute;
use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;
use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;
use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;

pub mod channel_set;

///The Fixture Type Attribute is assinged to a Channel Function and defines the function of its DMX Range
#[derive(Debug)]
pub struct ChannelFunction {
    ///Unique name; Default value: Name of attribute and number of channel function.
    pub name: Name,
    ///Link to attribute; Starting point is the attributes node_2. Default value: “NoFeature”.
    pub attribute: NodeChannelFunctionAttribute,
    ///The manufacturer’s original name of the attribute; Default: empty
    pub original_attribute: String,
    ///Start DMX value; The end DMX value is calculated as a DMXFrom of the next channel function – 1 or the maximum value of the DMX channel. Default value: "0/1".
    pub dmx_from: DMXValue,
    ///Default DMX value of channel function when activated by the control system.
    pub default: DMXValue,
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
    pub mode_from: Option<DMXValue>,
    ///Only used together with ModeMaster; DMX end value; Default value: 0/1
    pub mode_to: Option<DMXValue>,
    //A list of channel sets for the channel function
    pub channel_sets: Vec<ChannelSet>,
}

const DEFAULT_DMX_FROM: DMXValue = DMXValue {
    initial_value: 0,
    n: 1,
    is_byte_shifting: false,
};

const DEFAULT_DMX_DEFAULT: DMXValue = DMXValue {
    initial_value: 0,
    n: 1,
    is_byte_shifting: false,
};

impl DeparseSingle for ChannelFunction {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut attribute: NodeChannelFunctionAttribute = Default::default();
        let mut original_attribute: String = String::new();
        let mut dmx_from: DMXValue = DEFAULT_DMX_FROM;
        let mut default: DMXValue = DEFAULT_DMX_DEFAULT;
        let mut physical_from: f32 = 0.;
        let mut physical_to: f32 = 0.;
        let mut real_fade: f32 = 0.;
        let mut real_acceleration: f32 = 0.;
        let mut wheel: NodeChannelFunctionWheel = Default::default();
        let mut emitter: NodeChannelFunctionEmitter = Default::default();
        let mut filter: NodeChannelFunctionFilter = Default::default();
        let mut mode_master: NodeChannelFunctionModeMaster = Default::default();
        let mut mode_from: Option<DMXValue> = None;
        let mut mode_to: Option<DMXValue> = None;
        let mut channel_sets: Vec<ChannelSet> = vec![];
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Attribute" => attribute = attr.into(),
                b"OriginalAttribute" => original_attribute = deparse::attr_to_string(&attr),
                b"DMXFrom" => dmx_from = attr.try_into().unwrap_or_else(|_| DEFAULT_DMX_FROM),
                b"Default" => default = attr.try_into().unwrap_or_else(|_| DEFAULT_DMX_DEFAULT),
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
                        channel_sets.push(ChannelSet::single_from_event(reader, e)?);
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

        Ok(
            ChannelFunction {
                name,
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
            }
        )
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"ChannelFunction"
    }

    fn single_event_name() -> String {
        "ChannelFunction".to_string()
    }

    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty_no_log(&compare.name)
    }
}

impl AssertEqAllowEmpty for ChannelFunction {
    fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
        self.name.is_eq_allow_empty(&other.name) &&
            self.attribute.is_eq_allow_empty(&other.attribute) &&
            self.original_attribute == other.original_attribute &&
            self.dmx_from == other.dmx_from &&
            self.default == other.default &&
            self.physical_from == other.physical_from &&
            self.physical_to == other.physical_to &&
            self.real_fade == other.real_fade &&
            self.real_acceleration == other.real_acceleration &&
            self.wheel.is_eq_allow_empty(&other.wheel) &&
            self.emitter.is_eq_allow_empty(&other.emitter) &&
            self.filter.is_eq_allow_empty(&other.filter) &&
            self.mode_master.is_eq_allow_empty(&other.mode_master) &&
            self.mode_from == other.mode_from &&
            self.mode_to == other.mode_to &&
            ChannelSet::is_vec_eq(&self.channel_sets, &other.channel_sets)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
    use crate::utils::deparse::DeparseSingle;
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
    use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;
    use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;
    use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;

    #[test]
    fn test_normal() {
        ChannelFunction {
            name: "Magenta".try_into().unwrap(),
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 0.000000,
            real_acceleration: 0.000000,
            wheel: NodeChannelFunctionWheel(None),
            emitter: NodeChannelFunctionEmitter(None),
            filter: NodeChannelFunctionFilter(Some("Magenta".try_into().unwrap())),
            mode_master: NodeChannelFunctionModeMaster(Some("Base_ColorMacro1".try_into().unwrap())),
            mode_from: Some("0/1".try_into().unwrap()),
            mode_to: Some("0/1".try_into().unwrap()),
            channel_sets: vec![
                ChannelSet {
                    name: "min".try_into().unwrap(),
                    dmx_from: "0/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    name: "".try_into().unwrap(),
                    dmx_from: "1/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
                ChannelSet {
                    name: "max".try_into().unwrap(),
                    dmx_from: "255/1".try_into().unwrap(),
                    physical_from: None,
                    physical_to: None,
                    wheel_slot_index: Some(0),
                },
            ],
        }.test(
            r#"
            <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
                <ChannelSet DMXFrom="0/1" Name="min" WheelSlotIndex="0"/>
                <ChannelSet DMXFrom="1/1" Name="" WheelSlotIndex="0"/>
                <ChannelSet DMXFrom="255/1" Name="max" WheelSlotIndex="0"/>
              </ChannelFunction>
            "#)
    }

    #[test]
    fn test_max() {
        ChannelFunction {
            name: "Magenta".try_into().unwrap(),
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel(Some("Wheel1".try_into().unwrap())),
            emitter: NodeChannelFunctionEmitter(Some("Emitter1".try_into().unwrap())),
            filter: NodeChannelFunctionFilter(Some("Magenta".try_into().unwrap())),
            mode_master: NodeChannelFunctionModeMaster(Some("Base_ColorMacro1".try_into().unwrap())),
            mode_from: Some("0/1".try_into().unwrap()),
            mode_to: Some("0/1".try_into().unwrap()),
            channel_sets: vec![],
        }.test(
            r#"
            <ChannelFunction Wheel="Wheel1" Emitter="Emitter1" Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#)
    }

    #[test]
    fn test_min_1() {
        ChannelFunction {
            name: "Magenta".try_into().unwrap(),
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel(None),
            emitter: NodeChannelFunctionEmitter(None),
            filter: NodeChannelFunctionFilter(None),
            mode_master: NodeChannelFunctionModeMaster(None),
            mode_from: None,
            mode_to: None,
            channel_sets: vec![],
        }.test(
            r#"
            <ChannelFunction Wheel="" Emitter="" Filter="" ModeFrom="" ModeMaster="" ModeTo=""  Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#)
    }

    #[test]
    fn test_min_2() {
        ChannelFunction {
            name: "Magenta".try_into().unwrap(),
            attribute: "ColorSub_M".try_into().unwrap(),
            original_attribute: "orig".to_string(),
            dmx_from: "0/1".try_into().unwrap(),
            default: "0/1".try_into().unwrap(),
            physical_from: 0.000000,
            physical_to: 1.000000,
            real_fade: 3.000000,
            real_acceleration: 4.001000,
            wheel: NodeChannelFunctionWheel(None),
            emitter: NodeChannelFunctionEmitter(None),
            filter: NodeChannelFunctionFilter(None),
            mode_master: NodeChannelFunctionModeMaster(None),
            mode_from: None,
            mode_to: None,
            channel_sets: vec![],
        }.test(
            r#"
            <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Name="Magenta" OriginalAttribute="orig" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="4.001000" RealFade="3.000000">
            </ChannelFunction>
            "#)
    }
}