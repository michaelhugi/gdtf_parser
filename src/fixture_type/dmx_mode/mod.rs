//!This section is describes all DMX modes of the device
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::DMXChannel;
use crate::utils::deparse::{DeparseSingle, DeparseVec};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
use crate::utils::units::name::Name;

pub mod dmx_channel;

///Each DMX mode describes logical control a part of the device in a specific mode
#[derive(Debug)]
pub struct DMXMode {
    ///The unique name of the DMX mode
    pub name: Name,
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    pub geometry: Name,
    ///Description of all DMX channels used in the mode
    pub dmx_channels: Vec<DMXChannel>,

    //TODO relations

    //TODO ftmacros
}

impl DeparseSingle for DMXMode {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut geometry: Name = Default::default();
        let mut dmx_channels: Vec<DMXChannel> = Vec::new();

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Geometry" => geometry = attr.into(),
                _ => {}
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        b"DMXChannels" => dmx_channels = DMXChannel::vec_from_event(reader, e)?,
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
                _ => {}
            }
        }
        buf.clear();

        Ok(Self {
            name,
            geometry,
            dmx_channels,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXMode"
    }

    fn single_event_name() -> String {
        "DMXMode".to_string()
    }
}

#[cfg(test)]
impl PartialEqAllowEmpty for DMXMode {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        self.name.is_eq_allow_empty(&other.name, log) &&
            self.geometry.is_eq_allow_empty(&other.geometry, log) &&
            DMXChannel::is_vec_eq_unordered(&self.dmx_channels, &other.dmx_channels)

        //TODO add todo fields
    }
}

#[cfg(test)]
impl TestDeparseSingle for DMXMode {
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty(&compare.name, false)
    }
}


impl DeparseVec for DMXMode {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXModes"
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::DMXChannel;
    use crate::fixture_type::dmx_mode::DMXMode;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::units::dmx_break::DMXBreak;
    use crate::utils::units::highlight::Highlight;
    use crate::utils::units::name::Name;
    use crate::utils::units::offset::Offset;

    #[test]
    fn test_normal() {
        DMXMode {
            name: Name::Name("Mode 1 12 DMX".to_string()),
            geometry: Name::Name("Base".to_string()),
            dmx_channels: vec![
                DMXChannel {
                    dmx_break: DMXBreak::Overwrite,
                    offset: Offset::Value(vec![1, 2]),
                    initial_function: Default::default(),
                    highlight: Highlight::None,
                    geometry: "Yoke".try_into().unwrap(),
                    logical_channels: vec![],
                }, DMXChannel {
                    dmx_break: DMXBreak::Value(1),
                    offset: Offset::Value(vec![3, 4]),
                    initial_function: Default::default(),
                    highlight: Highlight::None,
                    geometry: "Head".try_into().unwrap(),
                    logical_channels: vec![],
                }
            ],
        }.test(
            r#"
      <DMXMode Geometry="Base" Name="Mode 1 12 DMX">
        <DMXChannels>
          <DMXChannel DMXBreak="Overwrite" Default="32768/2" Geometry="Yoke" Highlight="None" Offset="1,2">
          </DMXChannel>
          <DMXChannel DMXBreak="1" Default="32767/2" Geometry="Head" Highlight="None" Offset="3,4">
          </DMXChannel>
        </DMXChannels>
       </DMXMode>
            "#
        );
    }
}