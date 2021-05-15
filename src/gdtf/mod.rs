use std::borrow::Borrow;
use std::convert::TryFrom;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::deparse::DeparseSingle;
use crate::errors::GdtfError;
use crate::gdtf::fixture_type::FixtureType;
use crate::units::data_version::DataVersion;

pub mod fixture_type;

#[derive(Debug)]
pub struct GDTF {
    pub data_version: DataVersion,
    pub fixture_type: FixtureType,
}

impl DeparseSingle for GDTF {
    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut data_version = DataVersion::Unknown;
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"DataVersion" => data_version = std::str::from_utf8(attr.value.borrow())?.into(),
                _ => {}
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if FixtureType::is_single_event_name(e.name()) {
                        return Ok(
                            GDTF {
                                fixture_type: FixtureType::single_from_event(reader, e)?,
                                data_version,
                            }
                        );
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
        buf.clear();

        Err(GdtfError::RequiredValueNotFoundError(format!("Could not find {}", FixtureType::single_event_name())))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"GDTF"
    }

    fn single_event_name() -> String {
        "GDTF".to_string()
    }

    fn is_single_eq(&self, other: &Self) -> bool {
        self.data_version == other.data_version
    }
}

impl TryFrom<&Path> for GDTF {
    type Error = GdtfError;

    fn try_from(file_path: &Path) -> Result<Self, Self::Error> {
        let mut archive = zip::ZipArchive::new(File::open(file_path)?)?;
        let mut description_xml = String::new();
        archive.by_name("description.xml")?.read_to_string(&mut description_xml)?;


        let mut reader = Reader::from_str(&description_xml);
        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == b"GDTF" {
                        return Ok(
                            GDTF::single_from_event(&mut reader, e)?
                        );
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            };
        }
        buf.clear();
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;
    use std::path::Path;

    use crate::gdtf::GDTF;

    #[test]
    fn test_sgm() {
        let gdtf: GDTF = Path::new("test/SGM_Light@G-7_Spot@Rev_A.gdtf").try_into().unwrap();
        println!("{:?}", gdtf);
    }
}