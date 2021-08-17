use std::str::FromStr;

use lazy_static::lazy_static;
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use regex::Regex;

use crate::fixture_type::physical_descriptions::cris::CRIGroup;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::{ReadGdtf, TestReadGdtf};

///Defines the CRI for one of the 99 color samples
#[derive(Debug, PartialEq, Clone)]
pub struct Cri {
    ///Color sample. The defined values are “CES01”, “CES02”, … “CES99”. Default Value “CES01"
    pub ces: u8,
    ///The color rendering index for this sample. Size: 1 byte; Default value: 100
    pub color_rendering_index: u8,
}

impl Default for Cri {
    fn default() -> Self {
        Self { color_rendering_index: 100, ces: 0 }
    }
}

impl ReadGdtf for Cri {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = Cri;
    const NODE_NAME: &'static [u8] = b"CRI";
    const PARENT_NODE_NAME: &'static [u8] = CRIGroup::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"ColorRenderingIndex" => data_holder.color_rendering_index = read::attr_to_u8_option(attr).ok_or_else(|| Self::attribute_not_found(b"ColorRenderingIndex"))?,
            b"CES" => {
                lazy_static! {
                        static ref REG: Regex = Regex::new(r"CES(\d{2})").unwrap();
                    }
                for cap in REG.captures_iter(read::attr_to_str(&attr)) {
                    data_holder.ces = u8::from_str(&cap[1]).unwrap_or(0);
                }
            }
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
impl TestReadGdtf for Cri {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(Cri { ces: 1, color_rendering_index: 100 })),
            (None, Some(Cri { ces: 99, color_rendering_index: 23 }))
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<CRI CES="CES01"></CRI>"#.to_string(),
            r#"<CRI CES="CES99" ColorRenderingIndex="23"></CRI>"#.to_string()
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}


#[cfg(test)]
pub mod tests {
    use crate::fixture_type::physical_descriptions::cris::cri::Cri;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Cri::execute_tests()
    }
}