//!Defines DMX profile descriptions.
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

#[derive(Debug, PartialEq, Default, Clone)]
///Defines DMX profile descriptions.
pub struct DmxProfile {}

impl ReadGdtf for DmxProfile {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = DmxProfile;
    const NODE_NAME: &'static [u8] = b"DMXProfile";
    const PARENT_NODE_NAME: &'static [u8] = b"DMXProfiles";
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(_: &mut Self::DataHolder, _: Attribute<'_>) -> Result<(), Self::Error> {
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
impl TestReadGdtf for DmxProfile {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(DmxProfile {}))
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![r#"<DMXProfile/>"#.to_string()]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}


#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::dmx_profiles::DmxProfile;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        DmxProfile::execute_tests();
    }
}