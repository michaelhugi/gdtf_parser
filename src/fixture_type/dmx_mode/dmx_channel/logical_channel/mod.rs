//! Contains LogicalChannel and it's children
use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::master::Master;
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;
use crate::utils::units::snap::Snap;

pub mod channel_function;

///The Fixture Type Attribute is assinged to a LogicalChannel and defines the function of the LogicalChannel. All logical channels that are children of the same DMX channel are mutually exclusive. In a DMX mode, only one logical channel with the same attribute can reference the same geometry at a time.
#[derive(Debug)]
pub struct ChannelFunction {
    ///Link to the attribute; The starting point is the Attribute Collect
    pub attribute: Node,
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

impl DeparseSingle for ChannelFunction {
    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut attribute: Node = Node::default();
        let mut snap: Snap = Snap::default();
        let mut master: Master = Master::default();
        let mut mib_fade: f32 = 0.;
        let mut dmx_change_time_limit: f32 = 0.;
        let mut channel_functions: Vec<ChannelFunction> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Attribute" => attribute = deparse::attr_to_str(&attr)?.try_into()?,
                b"Snap" => snap = deparse::attr_to_str(&attr)?.into(),
                b"Master" => master = deparse::attr_to_str(&attr)?.into(),
                b"MibFade" => mib_fade = deparse::attr_to_f32(&attr)?,
                b"DMXChangeTimeLimit" => dmx_change_time_limit = deparse::attr_to_f32(&attr)?,
                _ => {}
            }
        }

        unimplemented!()
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"LogicalChannel"
    }

    fn single_event_name() -> String {
        "LogicalChannel".to_string()
    }

    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.attribute == other.attribute &&
            self.snap == other.snap &&
            self.master == other.master &&
            self.mib_fade == other.mib_fade &&
            self.dmx_change_time_limit == other.dmx_change_time_limit &&
            ChannelFunction::is_vec_eq(&self.channel_functions, &other.channel_functions)
    }

    fn is_same_item_identifier(&self, other: &Self) -> bool {
        self.attribute == other.attribute
    }
}