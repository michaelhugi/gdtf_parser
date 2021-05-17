use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DMXValue;
use crate::utils::units::name::Name;

#[derive(Debug)]
pub struct ChannelSet {
    pub name: Name,
    pub dmx_from: DMXValue,
    pub physical_from: Option<f32>,
    pub physical_to: Option<f32>,
    pub wheel_slot_index: Option<u8>,
}

impl DeparseSingle for ChannelSet {
    fn single_from_event_unchecked(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name: Name = "".try_into().unwrap();
        let mut dmx_from: DMXValue = "1/1".try_into().unwrap();
        let mut physical_from: Option<f32> = None;
        let mut physical_to: Option<f32> = None;
        let mut wheel_slot_index: Option<u8> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Self::attr_to_name(&attr)?,
                b"DMXFrom" => dmx_from = Self::attr_to_str(&attr)?.try_into()?,
                b"PhysicalFrom" => physical_from = Self::attr_to_f32_option(&attr)?,
                b"PhysicalTo" => physical_to = Self::attr_to_f32_option(&attr)?,
                b"WheelSlotIndex" => wheel_slot_index = Self::attr_to_u8_option(&attr)?,
                _ => {}
            }
        }
        Ok(
            ChannelSet {
                name,
                dmx_from,
                physical_from,
                physical_to,
                wheel_slot_index,
            }
        )
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"ChannelSet"
    }

    fn single_event_name() -> String {
        "ChannelSet".to_string()
    }

    #[cfg(test)]
    fn is_single_eq_no_log(&self, compare: &Self) -> bool {
        self.name == compare.name &&
            self.dmx_from == compare.dmx_from &&
            self.physical_from == compare.physical_from &&
            self.physical_to == compare.physical_to &&
            self.wheel_slot_index == compare.wheel_slot_index
    }

    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name == compare.name
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::utils::deparse::DeparseSingle;
    use crate::utils::units::dmx_value::DMXValue;

    #[test]
    fn test_max() {
        ChannelSet {
            name: "Slow".try_into().unwrap(),
            dmx_from: DMXValue {
                is_byte_shifting: false,
                initial_value: 55,
                n: 1,
            },
            physical_from: Some(23.1),
            physical_to: Some(23.4),
            wheel_slot_index: Some(0),
        }.test(
            r#"<ChannelSet DMXFrom="55/1" Name="Slow" WheelSlotIndex="0" PhysicalFrom="23.1" PhysicalTo="23.4" />"#)
    }

    #[test]
    fn test_min() {
        ChannelSet {
            name: "Slow".try_into().unwrap(),
            dmx_from: DMXValue {
                is_byte_shifting: true,
                initial_value: 55,
                n: 2,
            },
            physical_from: None,
            physical_to: None,
            wheel_slot_index: None,
        }.test(
            r#"<ChannelSet DMXFrom="55/2s" Name="Slow"/>"#)
    }

    #[test]
    fn test_min_2() {
        ChannelSet {
            name: "Slow".try_into().unwrap(),
            dmx_from: DMXValue {
                is_byte_shifting: true,
                initial_value: 55,
                n: 2,
            },
            physical_from: None,
            physical_to: None,
            wheel_slot_index: None,
        }.test(
            r#"<ChannelSet DMXFrom="55/2s" Name="Slow" WheelSlotIndex="" PhysicalFrom="" PhysicalTo=""/>"#)
    }

    #[test]
    fn test_invald() {
        match ChannelSet::single_from_xml(r#"<ChnnelSet DMXFrom="55/2s" Name="Slow" WheelSlotIndex="" PhysicalFrom="" PhysicalTo=""/>"#) {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invald_2() {
        match ChannelSet::single_from_xml(r#"<ChannelSet DMXFrom="55/2s" Name="Slow" WheelSlotIndex="invalid" PhysicalFrom="" PhysicalTo=""/>"#) {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }
}