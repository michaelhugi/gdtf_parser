//! Module for DmxValue for a step in a manufacurer defined macro

use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::MacroDmxStep;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::dmx_value::DmxValue;
use crate::utils::units::node::Node;

///Defines a dmx value for a step in a macro
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MacroDmxValue {
    ///Value of the DMX channel
    pub value: DmxValue,
    ///Link to a DMX channel. Starting node DMX Channel Collect.
    pub dmx_channel: Node,
}

///Helper struct for temporary data during deparse
#[derive(Default)]
pub(crate) struct MacroDmxValueDataHolder {
    ///Value of the DMX channel
    pub value: Option<DmxValue>,
    ///Link to a DMX channel. Starting node DMX Channel Collect.
    pub dmx_channel: Option<Node>,
}

impl ReadGdtf for MacroDmxValue {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = MacroDmxValueDataHolder;

    const NODE_NAME: &'static [u8] = b"MacroDMXValue";
    const PARENT_NODE_NAME: &'static [u8] = MacroDmxStep::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Value" => data_holder.value = Some(DmxValue::new_from_attr(attr)?),
            b"DMXChannel" => data_holder.dmx_channel = Node::new_from_attr(attr)?,
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        _: &mut Self::DataHolder,
        _: &mut Reader<&[u8]>,
        _: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            value: data_holder
                .value
                .ok_or_else(|| Self::attribute_not_found(b"Value"))?,
            dmx_channel: data_holder
                .dmx_channel
                .ok_or_else(|| Self::attribute_not_found(b"DMXChannel"))?,
        })
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed");
    }
}

#[cfg(test)]
impl TestReadGdtf for MacroDmxValue {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    value: DmxValue {
                        initial_value: 12,
                        n: 1,
                        is_byte_shifting: false,
                    },
                    dmx_channel: Node::new_from_str("Channel1").unwrap().unwrap(),
                }),
            ),
            (
                None,
                Some(Self {
                    value: DmxValue {
                        initial_value: 13,
                        n: 2,
                        is_byte_shifting: true,
                    },
                    dmx_channel: Node::new_from_str("Channel2").unwrap().unwrap(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<MacroDMXValue Value="12/1" DMXChannel="Channel1"/>"#.to_string(),
            r#"<MacroDMXValue Value="13/2s" DMXChannel="Channel2"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<MacroDMXValue DMXChannel="Channel1"/>"#.to_string(),
            r#"<MacroDMXValue Value="13/2s"/>"#.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::macro_dmx_value::MacroDmxValue;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        MacroDmxValue::execute_tests();
    }
}
