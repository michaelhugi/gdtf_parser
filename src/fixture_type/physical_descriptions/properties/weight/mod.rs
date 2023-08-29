//!defines the overall weight of the device
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

///defines the overall weight of the device
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Weight {
    ///Weight of the device including all accessories. Unit: kilogram. Default value: 0
    pub value: f32,
}

impl ReadGdtf for Weight {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = Weight;
    const NODE_NAME: &'static [u8] = b"Weight";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        if let b"Value" = attr.key {
            data_holder.value = read::attr_to_f32(attr)
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
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for Weight {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(Weight { value: 0.0 })),
            (None, Some(Weight { value: 77.0 })),
            (None, Some(Weight { value: 0.37 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Weight/>"#.to_string(),
            r#"<Weight Value="77"/>"#.to_string(),
            r#"<Weight Value="0.37"></Weight>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::properties::weight::Weight;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Weight::execute_tests()
    }
}
