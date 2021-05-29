use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
#[cfg(test)]
use crate::utils::partial_eq_allow_empty::PartialEqAllowEmpty;
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
    fn single_from_event(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut dmx_from: DMXValue = "1/1".try_into().unwrap();
        let mut physical_from: Option<f32> = None;
        let mut physical_to: Option<f32> = None;
        let mut wheel_slot_index: Option<u8> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"DMXFrom" => dmx_from = deparse::attr_to_str(&attr).try_into()?,
                b"PhysicalFrom" => physical_from = deparse::attr_to_f32_option(&attr),
                b"PhysicalTo" => physical_to = deparse::attr_to_f32_option(&attr),
                b"WheelSlotIndex" => wheel_slot_index = deparse::attr_to_u8_option(&attr),
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
}

#[cfg(test)]
impl PartialEqAllowEmpty for ChannelSet {
    fn is_eq_allow_empty_impl(&self, other: &Self, log: bool) -> bool {
        if self.dmx_from != other.dmx_from {
            return Self::print_structs_not_equal(&self.dmx_from, &other.dmx_from, log);
        }
        if self.physical_from != other.physical_from {
            return Self::print_structs_not_equal(&self.physical_from, &other.physical_from, log);
        }
        if self.physical_to != other.physical_to {
            return Self::print_structs_not_equal(&self.physical_to, &other.physical_to, log);
        }
        if self.wheel_slot_index != other.wheel_slot_index {
            return Self::print_structs_not_equal(&self.wheel_slot_index, &other.wheel_slot_index, log);
        }
        self.name.is_eq_allow_empty(&other.name, log)
    }
}


#[cfg(test)]
impl TestDeparseSingle for ChannelSet {
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty(&compare.name, false)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::utils::deparse::TestDeparseSingle;
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
}