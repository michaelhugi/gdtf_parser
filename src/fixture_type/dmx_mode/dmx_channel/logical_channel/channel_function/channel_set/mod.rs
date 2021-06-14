//! Defines the channel sets of the channel function
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::ChannelFunction;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
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

///Helper struct to deparse from xml
#[derive(Default)]
pub(crate) struct ChannelSetDataHolder {
    ///Start DMX value; The end DMX value is calculated as a DMXFrom of the next channel set – 1 or the maximum value of the current channel function
    pub dmx_from: Option<DmxValue>,
    ///Physical start value
    pub physical_from: Option<f32>,
    ///Physical start value
    pub physical_to: Option<f32>,
    ///the channel function has a link to a wheel, a corresponding slot index needs to be specified. The wheel slot index results from the order of slots of the wheel which is linked in the channel function
    pub wheel_slot_index: Option<u8>,
}


impl ReadGdtf for ChannelSet {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = ChannelSetDataHolder;

    const NODE_NAME: &'static [u8] = b"ChannelSet";
    const PARENT_NODE_NAME: &'static [u8] = ChannelFunction::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;


    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"DMXFrom" => data_holder.dmx_from = Some(DmxValue::new_from_attr(attr)?),
            b"PhysicalFrom" => data_holder.physical_from = read::attr_to_f32_option(attr),
            b"PhysicalTo" => data_holder.physical_to = read::attr_to_f32_option(attr),
            b"WheelSlotIndex" => data_holder.wheel_slot_index = read::attr_to_u8_option(attr),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(_: &mut Self::DataHolder, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            dmx_from: data_holder.dmx_from.ok_or_else(|| Self::attribute_not_found(b"DmxFrom"))?,
            physical_from: data_holder.physical_from,
            physical_to: data_holder.physical_to,
            wheel_slot_index: data_holder.wheel_slot_index,
        })
    }
}

#[cfg(test)]
impl TestReadGdtf for ChannelSet {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("Closed").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("0/1").unwrap(), physical_from: None, physical_to: None, wheel_slot_index: None })),
            (Some(Name::new("Open").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("5/1s").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("Slow").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("10/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("WSI").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("11/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("Wired DMX").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("10/1").unwrap(), wheel_slot_index: Some(0), physical_from: Some(0.000012), physical_to: Some(12.040001) })),
            (Some(Name::new("Slow").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("55/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("STH").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("56/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("Fast").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("79/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
            (Some(Name::new("").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("235/1").unwrap(), wheel_slot_index: Some(1), physical_from: None, physical_to: None })),
            (Some(Name::new("Something").unwrap()), Some(Self { dmx_from: DmxValue::new_from_str("236/1").unwrap(), wheel_slot_index: Some(0), physical_from: None, physical_to: None })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<ChannelSet DMXFrom="0/1" Name="Closed"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="5/1s" Name="Open" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="10/1" Name="Slow" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="11/1" Name="WSI" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="10/1" Name="Wired DMX" PhysicalFrom="0.000012" PhysicalTo="12.040001" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="55/1" Name="Slow" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="56/1" Name="STH" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="79/1" Name="Fast" WheelSlotIndex="0"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="235/1" Name="" WheelSlotIndex="1"/>"#.to_string(),
            r#"<ChannelSet DMXFrom="236/1" Name="Something" WheelSlotIndex="0"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![r#"<ChannelSet Name="Closed"/>"#.to_string()]
    }
}

#[cfg(test)]
pub mod tests {
    use crate::fixture_type::dmx_mode::dmx_channel::logical_channel::channel_function::channel_set::ChannelSet as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}