//!defines the ambient operating temperature range
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

///defines the ambient operating temperature range
#[derive(Debug, PartialEq, Clone)]
pub struct OperatingTemperature {
    ///Lowest temperature the device can be operated. Unit: °C. Default value: 0
    pub low: f32,
    ///Highest temperature the device can be operated. Unit: °C. Default value: 40
    pub high: f32,
}

///Default for low is 0 degrees Celsius, default for high is 40 degrees Celsius
///```rust
///
/// use gdtf_parser::fixture_type::physical_descriptions::properties::operating_temperature::OperatingTemperature;
/// assert_eq!(OperatingTemperature::default(), OperatingTemperature { low: 0.0, high: 40.0 })
/// ```
impl Default for OperatingTemperature {
    fn default() -> Self {
        Self { low: 0.0, high: 40.0 }
    }
}

///Deparsing from description xml in GDTF
impl ReadGdtf for OperatingTemperature {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = OperatingTemperature;
    const NODE_NAME: &'static [u8] = b"OperatingTemperature";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"Low" => data_holder.low = read::attr_to_f32(attr),
            b"High" => data_holder.high = read::attr_to_f32(attr),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(_: &mut Self::DataHolder, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(_: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for OperatingTemperature {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(OperatingTemperature { low: 0.0, high: 40.0 })),
            (None, Some(OperatingTemperature { low: -34.2, high: 40.0 })),
            (None, Some(OperatingTemperature { low: 0.0, high: -322.1 })),
            (None, Some(OperatingTemperature { low: 234.1, high: 478.321 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<OperatingTemperature/>"#.to_string(),
            r#"<OperatingTemperature Low="-34.2"/>"#.to_string(),
            r#"<OperatingTemperature High="-322.1"/>"#.to_string(),
            r#"<OperatingTemperature Low="234.1" High="478.321"></OperatingTemperature>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::properties::operating_temperature::OperatingTemperature;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        OperatingTemperature::execute_tests()
    }
}