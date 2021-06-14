//!This section is describes all DMX modes of the device
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
use crate::utils::deparse::{DeparseHashMap, DeparseSingle};
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
use crate::utils::units::name::Name;

pub mod dmx_channel;

pub mod relation;
pub mod ft_macro;

///Each DMX mode describes logical control a part of the device in a specific mode
#[derive(Debug, PartialEq, Clone)]
pub struct DmxMode {
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    pub geometry: Name,
    ///Description of all DMX channels used in the mode
    pub dmx_channels: Vec<DmxChannel>,

    //TODO relations

    //TODO ftmacros
}

impl DeparseSingle for DmxMode {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME_DS: &'static [u8] = b"DMXMode";

    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut geometry: Name = Default::default();
        let mut dmx_channels: Vec<DmxChannel> = Vec::new();

        for attr in event.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"Geometry" => geometry = Name::new_from_attr(attr)?,
                _ => {}
            }
        }

        if has_children {
            let mut buf: Vec<u8> = Vec::new();
            let mut tree_down = 0;
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                        match e.name() {
                            b"DMXChannels" => dmx_channels = DmxChannel::read_vec_from_event(reader, e, true)?,
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
        }

        Ok((Some(name), Self {
            geometry,
            dmx_channels,
        }))
    }
}

impl DeparseHashMap for DmxMode {
    const PARENT_NODE_NAME: &'static [u8] = b"DMXModes";
}

#[cfg(test)]
impl TestDeparseHashMap for DmxMode {}

#[cfg(test)]
impl TestDeparseSingle for DmxMode {}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::dmx_channel::{DmxBreak, DmxChannel, Offset};
    use crate::fixture_type::dmx_mode::DmxMode;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::Node;

    #[test]
    fn test_normal() -> Result<(), GdtfError> {
        DmxMode {
            geometry: Name::new("Base")?,
            dmx_channels: vec![
                DmxChannel {
                    dmx_break: DmxBreak::Overwrite,
                    offset: Some(Offset::new(vec![1, 2])),
                    initial_function: Node::new_from_str("M").unwrap(),
                    highlight: None,
                    geometry: Name::new("Yoke")?,
                    logical_channels: vec![],
                }, DmxChannel {
                    dmx_break: DmxBreak::Value(1),
                    offset: Some(Offset::new(vec![3, 4])),
                    initial_function: Node::new_from_str("N").unwrap(),
                    highlight: None,
                    geometry: Name::new("Head")?,
                    logical_channels: vec![],
                }
            ],
        }.compare_to_primary_key_and_xml(Some(Name::new("Mode 1 12 DMX")?),
                                         r#"
          <DMXMode Geometry="Base" Name="Mode 1 12 DMX">
            <DMXChannels>
              <DMXChannel DMXBreak="Overwrite" Default="32768/2" Geometry="Yoke" InitialFunction="M" Highlight="None" Offset="1,2">
              </DMXChannel>
              <DMXChannel DMXBreak="1" Default="32767/2" Geometry="Head" InitialFunction="N" Highlight="None" Offset="3,4">
              </DMXChannel>
            </DMXChannels>
           </DMXMode>
                "#,
        );
        Ok(())
    }
}