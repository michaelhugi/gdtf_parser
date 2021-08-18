//!Contains Color Rendering Indexes (CRI) for a single color temperature
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::cris::cri::Cri;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

pub mod cri;

///Contains Color Rendering Indexes (CRI) for a single color temperature
#[derive(Debug, PartialEq, Clone)]
pub struct CriGroup {
    ///Color temperature; Default value: 6 000; Unit: Kelvin
    pub color_temperature: f32,
    ///Each defines the CRI for one of the 99 color samples
    pub cris: Vec<Cri>,
}

///```rust
/// use gdtf_parser::fixture_type::physical_descriptions::cris::CriGroup;
/// assert_eq!(CriGroup::default(),CriGroup{color_temperature:6000.0,cris:vec![]})
/// ```
impl Default for CriGroup {
    fn default() -> Self {
        Self {
            color_temperature: 6000.0,
            cris: vec![],
        }
    }
}

impl ReadGdtf for CriGroup {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = CriGroup;
    const NODE_NAME: &'static [u8] = b"CRIGroup";
    const PARENT_NODE_NAME: &'static [u8] = b"CRIs";
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        if let b"ColorTemperature" = attr.key {
            data_holder.color_temperature = read::attr_to_f32(attr)
        }
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        if let Cri::NODE_NAME = event.name() {
            data_holder
                .cris
                .push(Cri::read_single_from_event(reader, event, has_children)?.1)
        }
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
impl TestReadGdtf for CriGroup {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(CriGroup {
                    color_temperature: 2345.4,
                    cris: vec![],
                }),
            ),
            (
                None,
                Some(CriGroup {
                    color_temperature: 6000.0,
                    cris: vec![],
                }),
            ),
            (
                None,
                Some(CriGroup {
                    color_temperature: 2345.4,
                    cris: Cri::testdata_vec(),
                }),
            ),
            (
                None,
                Some(CriGroup {
                    color_temperature: 6000.0,
                    cris: Cri::testdata_vec(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<CRIGroup ColorTemperature="2345.4"/>"#.to_string(),
            r#"<CRIGroup/>"#.to_string(),
            format!(
                r#"<CRIGroup ColorTemperature="2345.4">{}</CRIGroup>"#,
                Cri::testdata_xml()
            ),
            format!(r#"<CRIGroup>{}</CRIGroup>"#, Cri::testdata_xml()),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::cris::CriGroup;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        CriGroup::execute_tests()
    }
}
