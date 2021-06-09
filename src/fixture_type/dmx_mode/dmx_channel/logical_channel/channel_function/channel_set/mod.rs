//! Defines the channel sets of the channel function
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
use crate::utils::deparse;
use crate::utils::deparse::{DeparseHashMap, DeparseSingle};
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::name::Name;

#[derive(Debug, PartialEq, Clone)]
/// Defines the channel sets of the channel function
pub struct ChannelSet {
    ///Start DMX value; The end DMX value is calculated as a DMXFrom of the next channel set – 1 or the maximum value of the current channel function
    pub dmx_from: DmxValue,
    ///Physical start value
    pub physical_from: Option<f32>,
    ///Physical start value
    pub physical_to: Option<f32>,
    ///the channel function has a link to a wheel, a corresponding slot index needs to be specified. The wheel slot index results from the order of slots of the wheel which is linked in the channel function
    pub wheel_slot_index: Option<u8>,
}

impl DeparseSingle for ChannelSet {
    type PrimaryKey = Name;
    type Error = GdtfError;

    const NODE_NAME: &'static [u8] = b"ChannelSet";

    fn read_single_from_event(_: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<(Option<Self::PrimaryKey>, Self), GdtfError> where
        Self: Sized {
        let mut name: Name = Default::default();
        let mut dmx_from: DmxValue = DmxValue::new_from_str("1/1").unwrap();
        let mut physical_from: Option<f32> = None;
        let mut physical_to: Option<f32> = None;
        let mut wheel_slot_index: Option<u8> = None;

        for attr in event.attributes().into_iter() {
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
        Ok((Some(name), ChannelSet {
            dmx_from,
            physical_from,
            physical_to,
            wheel_slot_index,
        }))
    }
}

impl DeparseHashMap for ChannelSet { const PARENT_NODE_NAME: &'static [u8] = ChannelFunction::NODE_NAME; }

#[cfg(test)]
impl TestDeparseSingle for ChannelSet {}

#[cfg(test)]
impl TestDeparseHashMap for ChannelSet {}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet as T;
    use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
    use crate::utils::errors::GdtfError;
    use crate::utils::units::dmx_value::DmxValue;
    use crate::utils::units::name::Name;

    #[test]
    fn test_deparse_single() -> Result<(), GdtfError> {
        assert_eq!(channel_set_testdata(1), T::read_single_from_xml(&channel_set_testdata_xml(1))?);
        assert_eq!(channel_set_testdata(2), T::read_single_from_xml(&channel_set_testdata_xml(2))?);
        assert_eq!(channel_set_testdata(3), T::read_single_from_xml(&channel_set_testdata_xml(3))?);
        assert_eq!(channel_set_testdata(4), T::read_single_from_xml(&channel_set_testdata_xml(4))?);
        assert_eq!(channel_set_testdata(5), T::read_single_from_xml(&channel_set_testdata_xml(5))?);
        assert_eq!(channel_set_testdata(6), T::read_single_from_xml(&channel_set_testdata_xml(6))?);
        assert_eq!(channel_set_testdata(7), T::read_single_from_xml(&channel_set_testdata_xml(7))?);
        assert_eq!(channel_set_testdata(8), T::read_single_from_xml(&channel_set_testdata_xml(8))?);
        assert_eq!(channel_set_testdata(9), T::read_single_from_xml(&channel_set_testdata_xml(9))?);
        assert_eq!(channel_set_testdata(10), T::read_single_from_xml(&channel_set_testdata_xml(10))?);
        Ok(())
    }

    #[test]
    fn test_deparse_hash_map() -> Result<(), GdtfError> {
        assert_eq!(channel_set_testdata_group(), T::read_hash_map_from_xml(&channel_set_testdata_xml_group())?);
        Ok(())
    }

    ///Returns 10 ChannelSet for testing
    pub fn channel_set_testdata(i: u8) -> (Option<Name>, T) {
        match i {
            1 => (Some(Name::new("Closed").unwrap()), T { dmx_from: DmxValue::new_from_str("0/1").unwrap(), physical_from: None, physical_to: None, wheel_slot_index: None }),
            2 => (Some(Name::new("Open").unwrap()), T { dmx_from: DmxValue::new_from_str("5/1s").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            3 => (Some(Name::new("Slow").unwrap()), T { dmx_from: DmxValue::new_from_str("10/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            4 => (Some(Name::new("WSI").unwrap()), T { dmx_from: DmxValue::new_from_str("11/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            5 => (Some(Name::new("Wired DMX").unwrap()), T { dmx_from: DmxValue::new_from_str("10/1").unwrap(), wheel_slot_index: Some(0), physical_from: Some(0.000012), physical_to: Some(12.040001) }),
            6 => (Some(Name::new("Slow").unwrap()), T { dmx_from: DmxValue::new_from_str("55/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            7 => (Some(Name::new("STH").unwrap()), T { dmx_from: DmxValue::new_from_str("56/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            8 => (Some(Name::new("Fast").unwrap()), T { dmx_from: DmxValue::new_from_str("79/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
            9 => (Some(Name::new("").unwrap()), T { dmx_from: DmxValue::new_from_str("235/1").unwrap(), wheel_slot_index: Some(1), physical_from: None, physical_to: None }),
            _ => (Some(Name::new("Something").unwrap()), T { dmx_from: DmxValue::new_from_str("236/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None }),
        }
    }

    ///Returns 10 xmls with ChannelSet for Testing
    pub fn channel_set_testdata_xml(i: u8) -> String {
        match i {
            1 => r#"<ChannelSet DMXFrom="0/1" Name="Closed"/>"#.to_string(),
            2 => r#"<ChannelSet DMXFrom="5/1s" Name="Open" WheelSlotIndex="0"/>"#.to_string(),
            3 => r#"<ChannelSet DMXFrom="10/1" Name="Slow" WheelSlotIndex="0"/>"#.to_string(),
            4 => r#"<ChannelSet DMXFrom="11/1" Name="WSI" WheelSlotIndex="0"/>"#.to_string(),
            5 => r#" <ChannelSet DMXFrom="10/1" Name="Wired DMX" PhysicalFrom="0.000012" PhysicalTo="12.040001" WheelSlotIndex="0"/>"#.to_string(),
            6 => r#"<ChannelSet DMXFrom="55/1" Name="Slow" WheelSlotIndex="0"/>"#.to_string(),
            7 => r#"<ChannelSet DMXFrom="56/1" Name="STH" WheelSlotIndex="0"/>"#.to_string(),
            8 => r#"<ChannelSet DMXFrom="79/1" Name="Fast" WheelSlotIndex="0"/>"#.to_string(),
            9 => r#"<ChannelSet DMXFrom="235/1" Name="" WheelSlotIndex="1"/>"#.to_string(),
            _ => r#"<ChannelSet DMXFrom="236/1" Name="Something" WheelSlotIndex="0"/>"#.to_string(),
        }
    }

    ///Returns a HashMap of ChannelSet for testing
    pub fn channel_set_testdata_group() -> HashMap<Name, T> {
        let mut map = HashMap::new();
        map.insert(channel_set_testdata(1).0.unwrap(), channel_set_testdata(1).1);
        map.insert(channel_set_testdata(2).0.unwrap(), channel_set_testdata(2).1);
        map.insert(channel_set_testdata(3).0.unwrap(), channel_set_testdata(3).1);
        map.insert(channel_set_testdata(4).0.unwrap(), channel_set_testdata(4).1);
        map.insert(channel_set_testdata(5).0.unwrap(), channel_set_testdata(5).1);
        map.insert(channel_set_testdata(6).0.unwrap(), channel_set_testdata(6).1);
        map.insert(channel_set_testdata(7).0.unwrap(), channel_set_testdata(7).1);
        map.insert(channel_set_testdata(8).0.unwrap(), channel_set_testdata(8).1);
        map.insert(channel_set_testdata(9).0.unwrap(), channel_set_testdata(9).1);
        map.insert(channel_set_testdata(10).0.unwrap(), channel_set_testdata(10).1);

        map
    }

    pub fn channel_set_testdata_xml_group() -> String {
        format!(
            r#"
            <ChannelFunction>
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
            </ChannelFunction>
            "#,
            channel_set_testdata_xml(1),
            channel_set_testdata_xml(2),
            channel_set_testdata_xml(3),
            channel_set_testdata_xml(4),
            channel_set_testdata_xml(5),
            channel_set_testdata_xml(6),
            channel_set_testdata_xml(7),
            channel_set_testdata_xml(8),
            channel_set_testdata_xml(9),
            channel_set_testdata_xml(10),
        )
    }

    /*
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
    }*/
}