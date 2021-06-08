use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;

#[derive(Debug, PartialEq, Clone)]
pub struct ChannelSet {
    pub dmx_from: DmxValue,
    pub physical_from: Option<f32>,
    pub physical_to: Option<f32>,
    pub wheel_slot_index: Option<u8>,
}

impl DeparseSingle for ChannelSet {
    type PrimaryKey = Name;
    type Error = GdtfError;
    const NODE_NAME: &'static [u8] = b"ChannelSet";

    fn read_single_from_event(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut dmx_from: DmxValue = DmxValue::new_from_str("1/1").unwrap();
        let mut physical_from: Option<f32> = None;
        let mut physical_to: Option<f32> = None;
        let mut wheel_slot_index: Option<u8> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"DMXFrom" => dmx_from = DmxValue::new_from_attr(attr)?,
                b"PhysicalFrom" => physical_from = deparse::attr_to_f32_option(&attr),
                b"PhysicalTo" => physical_to = deparse::attr_to_f32_option(&attr),
                b"WheelSlotIndex" => wheel_slot_index = deparse::attr_to_u8_option(&attr),
                _ => {}
            }
        }
        Ok((ChannelSet {
            dmx_from,
            physical_from,
            physical_to,
            wheel_slot_index,
        }, Some(name)))
    }

}

#[cfg(test)]
impl TestDeparseSingle for ChannelSet {}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;

    #[test]
    fn test_max() -> Result<(), GdtfError> {
        ChannelSet {
            dmx_from: DmxValue {
                is_byte_shifting: false,
                initial_value: 55,
                n: 1,
            },
            physical_from: Some(23.1),
            physical_to: Some(23.4),
            wheel_slot_index: Some(0),
        }.compare_to_primary_key_and_xml(Some(Name::new("Slow")?),
                                         r#"<ChannelSet DMXFrom="55/1" Name="Slow" WheelSlotIndex="0" PhysicalFrom="23.1" PhysicalTo="23.4" />"#);
        Ok(())
    }

    #[test]
    fn test_min() -> Result<(), GdtfError> {
        ChannelSet {
            dmx_from: DmxValue {
                is_byte_shifting: true,
                initial_value: 55,
                n: 2,
            },
            physical_from: None,
            physical_to: None,
            wheel_slot_index: None,
        }.compare_to_primary_key_and_xml(Some(Name::new("Slow")?),
                                         r#"<ChannelSet DMXFrom="55/2s" Name="Slow"/>"#);
        Ok(())
    }

    #[test]
    fn test_min_2() -> Result<(), GdtfError> {
        ChannelSet {
            dmx_from: DmxValue {
                is_byte_shifting: true,
                initial_value: 55,
                n: 2,
            },
            physical_from: None,
            physical_to: None,
            wheel_slot_index: None,
        }.compare_to_primary_key_and_xml(Some(Name::new("Slow")?),
                                         r#"<ChannelSet DMXFrom="55/2s" Name="Slow" WheelSlotIndex="" PhysicalFrom="" PhysicalTo=""/>"#);
        Ok(())
    }

    #[test]
    fn test_invald() {
        assert!(ChannelSet::read_single_from_xml(r#"<ChnnelSet DMXFrom="55/2s" Name="Slow" WheelSlotIndex="" PhysicalFrom="" PhysicalTo=""/>"#).is_err());
    }
}