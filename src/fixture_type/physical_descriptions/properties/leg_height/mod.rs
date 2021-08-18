use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

///defines the height of the legs
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LegHeight {
    ///Defines height of the legs – distance between the floor and the bottom base plate. Unit: meter. Default value: 0
    pub value: f32,
}

impl ReadGdtf for LegHeight {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = LegHeight;
    const NODE_NAME: &'static [u8] = b"LegHeight";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        if let b"Value" = attr.key {
            data_holder.value = read::attr_to_f32(attr);
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
impl TestReadGdtf for LegHeight {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(LegHeight { value: 0.012 })),
            (None, Some(LegHeight { value: 0.234 })),
            (None, Some(LegHeight { value: 0.0 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<LegHeight Value="0.012000"/>"#.to_string(),
            r#"<LegHeight Value="0.2340"></LegHeight>"#.to_string(),
            r#"<LegHeight/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::properties::leg_height::LegHeight;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        LegHeight::execute_tests();
    }
}