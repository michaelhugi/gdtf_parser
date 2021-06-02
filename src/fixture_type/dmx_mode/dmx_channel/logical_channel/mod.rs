//! Contains LogicalChannel and it's children
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::master::Master;
use crate::utils::units::node::node_logical_channel_attribute::NodeLogicalChannelAttribute;
use crate::utils::units::snap::Snap;

pub mod channel_function;

///The Fixture Type Attribute is assinged to a LogicalChannel and defines the function of the LogicalChannel. All logical channels that are children of the same DMX channel are mutually exclusive. In a DMX mode, only one logical channel with the same attribute can reference the same geometry at a time.
#[derive(Debug, PartialEq, Clone)]
pub struct LogicalChannel {
    ///Link to the attribute; The starting point is the Attribute Collect
    pub attribute: NodeLogicalChannelAttribute,
    ///If snap is enabled, the logical channel will not fade between values. Instead, it will jump directly to the new value.; Value: “Yes”, “No”, “On”, “Off”. Default value: “No”
    pub snap: Snap,
    ///Defines if all the subordinate channel functions react to a Group Control defined by the control system. Values: “None”, “Grand”, “Group”; Default value: “None”.
    pub master: Master,
    ///Minimum fade time for moves in black action. MibFade is defined for the complete DMX range. Default value: 0; Unit: second
    pub mib_fade: f32,
    ///Minimum fade time for the subordinate channel functions to change DMX values by the control system. DMXChangeTimeLimit is defined for the complete DMX range. Default value: 0; Unit: second
    pub dmx_change_time_limit: f32,
    ///A list of channel functions
    pub channel_functions: Vec<ChannelFunction>,
}

impl DeparseSingle for LogicalChannel {
    type PrimaryKey = ();

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut attribute: NodeLogicalChannelAttribute = Default::default();
        let mut snap: Snap = Snap::default();
        let mut master: Master = Master::default();
        let mut mib_fade: f32 = 0.;
        let mut dmx_change_time_limit: f32 = 0.;
        let mut channel_functions: Vec<ChannelFunction> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Attribute" => attribute = attr.into(),
                b"Snap" => snap = deparse::attr_to_str(&attr).into(),
                b"Master" => master = deparse::attr_to_str(&attr).into(),
                b"MibFade" => mib_fade = deparse::attr_to_f32(&attr),
                b"DMXChangeTimeLimit" => dmx_change_time_limit = deparse::attr_to_f32(&attr),
                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"ChannelFunction" {
                        channel_functions.push(ChannelFunction::single_from_event(reader, e)?.0);
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

        Ok((
            LogicalChannel {
                attribute,
                snap,
                master,
                mib_fade,
                dmx_change_time_limit,
                channel_functions,
            }
            , None))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"LogicalChannel"
    }

    fn single_event_name() -> String {
        "LogicalChannel".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for LogicalChannel {}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::LogicalChannel;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::master::Master;
    use crate::utils::units::node::node_channel_function_emitter::NodeChannelFunctionEmitter;
    use crate::utils::units::node::node_channel_function_filter::NodeChannelFunctionFilter;
    use crate::utils::units::node::node_channel_function_mode_master::NodeChannelFunctionModeMaster;
    use crate::utils::units::node::node_channel_function_wheel::NodeChannelFunctionWheel;
    use crate::utils::units::snap::Snap;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        LogicalChannel {
            attribute: "ColorSub_M".try_into().unwrap(),
            snap: Snap::Yes,
            master: Master::Grand,
            mib_fade: 0.1,
            dmx_change_time_limit: 0.0,
            channel_functions: vec![
                ChannelFunction {
                    name: "Magenta".try_into().unwrap(),
                    attribute: "ColorSub_M".try_into().unwrap(),
                    original_attribute: "".to_string(),
                    dmx_from: "0/1".try_into().unwrap(),
                    default: "0/1".try_into().unwrap(),
                    physical_from: 0.0,
                    physical_to: 1.0,
                    real_fade: 0.0,
                    real_acceleration: 0.0,
                    wheel: NodeChannelFunctionWheel::none(),
                    emitter: NodeChannelFunctionEmitter::none(),
                    filter: NodeChannelFunctionFilter::new_from_str("Magenta")?,
                    mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
                    mode_from: Some("0/1".try_into().unwrap()),
                    mode_to: Some("0/1".try_into().unwrap()),
                    channel_sets: vec![],
                },
                ChannelFunction {
                    name: "NoFeature".try_into().unwrap(),
                    attribute: "NoFeature".try_into().unwrap(),
                    original_attribute: "".to_string(),
                    dmx_from: "0/1".try_into().unwrap(),
                    default: "0/1".try_into().unwrap(),
                    physical_from: 0.0,
                    physical_to: 1.0,
                    real_fade: 0.0,
                    real_acceleration: 0.0,
                    wheel: NodeChannelFunctionWheel::none(),
                    emitter: NodeChannelFunctionEmitter::none(),
                    filter: NodeChannelFunctionFilter::none(),
                    mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
                    mode_from: Some("1/1".try_into().unwrap()),
                    mode_to: Some("255/1".try_into().unwrap()),
                    channel_sets: vec![],
                }
            ],
        }.test(None,
               r#"
            <LogicalChannel Attribute="ColorSub_M" DMXChangeTimeLimit="0.000000" Master="Grand" MibFade="0.100000" Snap="Yes">
              <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
              </ChannelFunction>
              <ChannelFunction Attribute="NoFeature" DMXFrom="0/1" Default="0/1" ModeFrom="1/1" ModeMaster="Base_ColorMacro1" ModeTo="255/1" Name="NoFeature" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000"/>
            </LogicalChannel>
            "#);
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        LogicalChannel {
            attribute: "".try_into().unwrap(),
            snap: Snap::No,
            master: Master::None,
            mib_fade: 0.0,
            dmx_change_time_limit: 0.0,
            channel_functions: vec![
                ChannelFunction {
                    name: "Magenta".try_into().unwrap(),
                    attribute: "ColorSub_M".try_into().unwrap(),
                    original_attribute: "".to_string(),
                    dmx_from: "0/1".try_into().unwrap(),
                    default: "0/1".try_into().unwrap(),
                    physical_from: 0.0,
                    physical_to: 1.0,
                    real_fade: 0.0,
                    real_acceleration: 0.0,
                    wheel: NodeChannelFunctionWheel::none(),
                    emitter: NodeChannelFunctionEmitter::none(),
                    filter: NodeChannelFunctionFilter::new_from_str("Magenta")?,
                    mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
                    mode_from: Some("0/1".try_into().unwrap()),
                    mode_to: Some("0/1".try_into().unwrap()),
                    channel_sets: vec![],
                },
                ChannelFunction {
                    name: "NoFeature".try_into().unwrap(),
                    attribute: "NoFeature".try_into().unwrap(),
                    original_attribute: "".to_string(),
                    dmx_from: "0/1".try_into().unwrap(),
                    default: "0/1".try_into().unwrap(),
                    physical_from: 0.0,
                    physical_to: 1.0,
                    real_fade: 0.0,
                    real_acceleration: 0.0,
                    wheel: NodeChannelFunctionWheel::none(),
                    emitter: NodeChannelFunctionEmitter::none(),
                    filter: NodeChannelFunctionFilter::none(),
                    mode_master: NodeChannelFunctionModeMaster::new_from_str("Base_ColorMacro1")?,
                    mode_from: Some("1/1".try_into().unwrap()),
                    mode_to: Some("255/1".try_into().unwrap()),
                    channel_sets: vec![],
                }
            ],
        }.test(None,
               r#"
            <LogicalChannel Attribute="" DMXChangeTimeLimit="" Master="" MibFade="" Snap="">
              <ChannelFunction Attribute="ColorSub_M" DMXFrom="0/1" Default="0/1" Filter="Magenta" ModeFrom="0/1" ModeMaster="Base_ColorMacro1" ModeTo="0/1" Name="Magenta" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000">
              </ChannelFunction>
              <ChannelFunction Attribute="NoFeature" DMXFrom="0/1" Default="0/1" ModeFrom="1/1" ModeMaster="Base_ColorMacro1" ModeTo="255/1" Name="NoFeature" OriginalAttribute="" PhysicalFrom="0.000000" PhysicalTo="1.000000" RealAcceleration="0.000000" RealFade="0.000000"/>
            </LogicalChannel>
            "#);
        Ok(())
    }
}