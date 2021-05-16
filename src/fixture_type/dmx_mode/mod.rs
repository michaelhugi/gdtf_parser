//!This section is describes all DMX modes of the device


use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub mod dmx_channel;

///Each DMX mode describes logical control a part of the device in a specific mode
#[derive(Debug)]
pub struct DMXMode {
    ///The unique name of the DMX mode
    name: Name,
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    geometry: Name,
}

impl DeparseSingle for DMXMode {
    fn single_from_event_unchecked(_reader: &mut Reader<&[u8]>, _e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        todo!()
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"DMXMode"
    }

    fn single_event_name() -> String {
        todo!()
    }

    #[cfg(test)]
    fn is_single_eq_no_log(&self, _other: &Self) -> bool {
        todo!()
    }

    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name == compare.name
    }
}