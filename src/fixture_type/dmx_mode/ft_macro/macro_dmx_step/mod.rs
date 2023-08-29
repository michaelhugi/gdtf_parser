//!Defines a DMX sequence for a macro
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::macro_dmx_value::MacroDmxValue;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

pub mod macro_dmx_value;

///Defines a DMX sequence for a macro
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct MacroDmxStep {
    ///Duration of a step; Default value: 1; Unit: seconds.
    pub duration: f32,
    ///DMX output values for the step
    pub dmx_values: Vec<MacroDmxValue>,
}

///Helper struct to hold temporary data during deparsing
#[derive(Default)]
pub(crate) struct MacroDmxStepDataHolder {
    ///Duration of a step; Default value: 1; Unit: seconds.
    pub duration: Option<f32>,
    ///DMX output values for the step
    pub dmx_values: Vec<MacroDmxValue>,
}

impl ReadGdtf for MacroDmxStep {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = MacroDmxStepDataHolder;

    const NODE_NAME: &'static [u8] = b"MacroDMXStep";
    const PARENT_NODE_NAME: &'static [u8] = b"MacroDMX";
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        if attr.key == b"Duration" {
            data_holder.duration = Some(read::attr_to_f32(attr));
        }
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        if event.name() == MacroDmxValue::NODE_NAME {
            data_holder
                .dmx_values
                .push(MacroDmxValue::read_single_from_event(reader, event, has_children)?.1);
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            duration: data_holder.duration.unwrap_or(1_f32),
            dmx_values: data_holder.dmx_values,
        })
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed");
    }
}

#[cfg(test)]
impl TestReadGdtf for MacroDmxStep {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    duration: 1.302,
                    dmx_values: vec![],
                }),
            ),
            (
                None,
                Some(Self {
                    duration: 12.0,
                    dmx_values: vec![],
                }),
            ),
            (
                None,
                Some(Self {
                    duration: 1_f32,
                    dmx_values: MacroDmxValue::testdata_vec(),
                }),
            ),
            (
                None,
                Some(Self {
                    duration: 17_f32,
                    dmx_values: MacroDmxValue::testdata_vec(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<MacroDMXStep Duration="1.302"/>"#.to_string(),
            r#"<MacroDMXStep Duration="12"></MacroDMXStep>"#.to_string(),
            format!(
                r#"<MacroDMXStep>{}</MacroDMXStep>"#,
                MacroDmxValue::testdata_xml()
            ),
            format!(
                r#"<MacroDMXStep Duration="17">{}</MacroDMXStep>"#,
                MacroDmxValue::testdata_xml()
            ),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::MacroDmxStep;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        MacroDmxStep::execute_tests();
    }
}
