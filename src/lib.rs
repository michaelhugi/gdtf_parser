//! A fast and well tested GDTF-parser
//!
//! 🚧️ **gdtf-parser is in pre-release state. Any breaking changes may be implemented without further notice!** 🚧️
//!
//! ⚠️ **Implementation of Matrix and Rotation may be wrong** ⚠️
//!
//! ## Description
//! GDTF stands for [`General Device Type Format`] and describes the hierarchical and logical structure and controls of any type of controllable device (e.g. luminaires, fog machines, etc.) in the lighting and entertainment industry.
//!
//! With gdtf-parser you can simply parse a .gdtf file in an struct that is similarly organized like the GDTF description.xml but with lighter hierarchy.
//!
//! gdtf-parser supports GDTF 1.0 and 1.1 accorting to DIN SPEC 15800:2020-07
//!
//! [`General Device Type Format`]: https://www.gdtf-share.com
//!
//! ## Usage
//! The main struct `GDTF` implements the trait `TryFrom<&Path>` so usage is straight forward:
//!
//! ### Example try from
//!
//!```rust
//! use std::convert::TryFrom;
//! use std::path::Path;
//! use gdtf_parser::Gdtf;
//! use gdtf_parser::utils::errors::GdtfError;
//! use gdtf_parser::DataVersion;
//! use gdtf_parser::utils::units::name::Name;
//! use gdtf_parser::utils::units::attribute_name::AttributeName;
//! use gdtf_parser::utils::units::physical_unit::PhysicalUnit;
//!
//! fn main() -> Result<(),GdtfError>{
//!     let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
//!     let gdtf: Gdtf = Gdtf::try_from(path)?;
//!
//!     assert_eq!(gdtf.data_version, DataVersion::Version1_0);
//!     assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
//!     assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
//!     Ok(())
//! }
//! ```
//!
//! ### Example try into
//!
//! ```rust
//! use std::convert::TryInto;
//! use std::path::Path;
//! use gdtf_parser::Gdtf;
//! use gdtf_parser::utils::errors::GdtfError;
//! use gdtf_parser::DataVersion;
//! use gdtf_parser::utils::units::name::Name;
//! use gdtf_parser::utils::units::attribute_name::AttributeName;
//! use gdtf_parser::utils::units::physical_unit::PhysicalUnit;
//!
//! fn main() -> Result<(),GdtfError> {
//!     let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
//!     let gdtf: Gdtf = path.try_into()?;
//!
//!     assert_eq!(gdtf.data_version, DataVersion::Version1_0);
//!     assert_eq!(gdtf.fixture_type.name, Name::new("ACME AE-610 BEAM")?);
//!     assert_eq!(gdtf.fixture_type.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
//!     Ok(())
//! }
//! ```
//!
//!
use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use quick_xml::events::{BytesStart, Event};
use quick_xml::events::attributes::Attribute;
use quick_xml::Reader;

use crate::fixture_type::FixtureType;
use crate::utils::deparse;
use crate::utils::deparse::DeparseSingle;
use crate::utils::deparse::GdtfDeparseError;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;

pub mod fixture_type;
pub mod utils;

///Describes the hierarchical and logical structure and controls of any type of controllable device (e.g. luminaires, fog machines, etc.) in the lighting and entertainment industry.
#[derive(Debug, PartialEq, Clone)]
pub struct Gdtf {
    pub data_version: DataVersion,
    pub fixture_type: FixtureType,
}

impl DeparseSingle for Gdtf {
    type PrimaryKey = ();
    type Error = GdtfError;
    const SINGLE_EVENT_NAME: &'static [u8] = b"GDTF";


    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut data_version = DataVersion::dummy();
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            if attr.key == b"DataVersion" {
                data_version = DataVersion::new_from_attr(attr);
            }
        }


        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name()==FixtureType::SINGLE_EVENT_NAME {
                        return Ok(
                            (Gdtf {
                                fixture_type: FixtureType::single_from_event(reader, e)?.0,
                                data_version,
                            }, None)
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
        Err(GdtfDeparseError::RequiredValueNotFoundError(FixtureType::single_event_name()))?
    }

    fn single_event_name() -> String {
        "GDTF".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for Gdtf {}

impl TryFrom<&Path> for Gdtf {
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
                            Gdtf::single_from_event(&mut reader, e)?.0
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
        Err(GdtfDeparseError::RequiredValueNotFoundError(Self::single_event_name()))?
    }
}


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of DataVersion
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


///The DataVersion attribute defines the minimal version of compatibility. The Version format is “Major.Minor”, where major and minor is Uint with size 1 byte
#[derive(Debug, PartialEq, Clone)]
pub enum DataVersion {
    ///Enum for GDTF Version 1.0
    Version1_0,
    ///Enum for GDTF Version 1.1
    Version1_1,
    ///Enum for other GDTF Version (most likely not supported yet in this library)
    ///Unknown contains original String
    Unknown(String),
}

impl DataVersion {
    ///Creates a dummy object to initiate if value is not yet defined
    /// ```rust
    /// use gdtf_parser::DataVersion;
    ///
    /// assert_eq!(DataVersion::dummy(), DataVersion::Unknown("?".to_string()));
    /// ```
    pub fn dummy() -> Self {
        Self::Unknown("?".to_string())
    }

    ///Parses a string defined in gdtf-xml-description to DataVersion.
    /// ```rust
    /// use gdtf_parser::DataVersion;
    ///
    /// assert_eq!(DataVersion::new_from_str("1.0"), DataVersion::Version1_0);
    /// assert_eq!(DataVersion::new_from_str("1.1"), DataVersion::Version1_1);
    /// assert_eq!(DataVersion::new_from_str("1.2"), DataVersion::Unknown("1.2".to_string()));
    /// assert_eq!(DataVersion::new_from_str("Something invalid"), DataVersion::Unknown("Something invalid".to_string()));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
        let mut value = s.split('.');
        let major = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        let minor = value.next().unwrap_or("").parse::<i32>().unwrap_or(-1);
        if value.next().is_some() { return Self::Unknown(s.to_string()); }

        match (major, minor) {
            (1, 0) => Self::Version1_0,
            (1, 1) => Self::Version1_1,
            (_, _) => Self::Unknown(s.to_string())
        }
    }
    ///Parses a string defined in gdtf-xml-description to DataVersion.
    /// ```rust
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// use gdtf_parser::DataVersion;
    ///
    /// assert_eq!(DataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.0")}), DataVersion::Version1_0);
    /// assert_eq!(DataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.1")}), DataVersion::Version1_1);
    /// assert_eq!(DataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"1.2")}), DataVersion::Unknown("1.2".to_string()));
    /// assert_eq!(DataVersion::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something invalid")}), DataVersion::Unknown("Something invalid".to_string()));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(deparse::attr_to_str(&attr))
    }
}


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of DataVersion
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{thread, time};
    use std::convert::TryFrom;
    use std::path::Path;
    use std::time::Duration;

    use crate::{DataVersion, Gdtf};
    use crate::utils::testdata;

    #[test]
    fn test_acme() {
        // crate::utils::testdata::acme_at_acme_ae_610_beam_at_acme_ae_610_beam::expect().test_with_result(Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf").try_into());
    }

    #[test]
    fn test_jb() {
        //  crate::utils::testdata::jb_lighting_at_p12_spot_hp_at_v_1_15::expect().test_with_result(Path::new("test/JB-Lighting@P12_Spot_HP@V_1.15.gdtf").try_into());
    }

    #[test]
    fn test_robe() {
        //   crate::utils::testdata::robe_lighting_at_robin_viva_cmy_at_13042021::expect().test_with_result(Path::new("test/Robe_Lighting@Robin_Viva_CMY@13042021.gdtf").try_into());
    }

    #[test]
    fn test_sgm() {
        //   crate::utils::testdata::sgm_light_at_g_7_spot_at_rev_a::expect().test_with_result(Path::new("test/SGM_Light@G-7_Spot@Rev_A.gdtf").try_into());
    }

    #[test]
    fn test_time() {
        thread::sleep(Duration::from_millis(2000));
        let now = time::Instant::now();
        let _ = Gdtf::try_from(Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf")).unwrap();
        println!("Deparsing Acme takes {:?}", now.elapsed());
        let now = time::Instant::now();
        let _ = Gdtf::try_from(Path::new("test/JB-Lighting@P12_Spot_HP@V_1.15.gdtf")).unwrap();
        println!("Deparsing JB takes {:?}", now.elapsed());
        let now = time::Instant::now();
        let _ = Gdtf::try_from(Path::new("test/Robe_Lighting@Robin_Viva_CMY@13042021.gdtf")).unwrap();
        println!("Deparsing Robe takes {:?}", now.elapsed());
        let now = time::Instant::now();
        let _ = Gdtf::try_from(Path::new("test/SGM_Light@G-7_Spot@Rev_A.gdtf")).unwrap();
        println!("Deparsing SGM takes {:?}", now.elapsed());
    }


    #[test]
    fn test_data_version_new_from_str() {
        assert_eq!(DataVersion::Version1_0, DataVersion::new_from_str("1.0"));
        assert_eq!(DataVersion::Version1_1, DataVersion::new_from_str("1.1"));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(DataVersion::Unknown("1.2".to_string()), DataVersion::new_from_str("1.2"));
        assert_eq!(DataVersion::Unknown("something invalid".to_string()), DataVersion::new_from_str("something invalid"));
        assert_eq!(DataVersion::Unknown("1.1.2".to_string()), DataVersion::new_from_str("1.1.2"));
        assert_eq!(DataVersion::Unknown("1.1.".to_string()), DataVersion::new_from_str("1.1."));
        assert_eq!(DataVersion::Unknown(".1.1".to_string()), DataVersion::new_from_str(".1.1"));
        assert_eq!(DataVersion::Unknown(".1".to_string()), DataVersion::new_from_str(".1"));
        assert_eq!(DataVersion::Unknown("1.".to_string()), DataVersion::new_from_str("1."));
    }

    #[test]
    fn test_data_version_new_from_attr_owned() {
        assert_eq!(DataVersion::Version1_0, DataVersion::new_from_attr(testdata::to_attr_owned(b"1.0")));
        assert_eq!(DataVersion::Version1_1, DataVersion::new_from_attr(testdata::to_attr_owned(b"1.1")));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(DataVersion::Unknown("1.2".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b"1.2")));
        assert_eq!(DataVersion::Unknown("something invalid".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b"something invalid")));
        assert_eq!(DataVersion::Unknown("1.1.2".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b"1.1.2")));
        assert_eq!(DataVersion::Unknown("1.1.".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b"1.1.")));
        assert_eq!(DataVersion::Unknown(".1.1".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b".1.1")));
        assert_eq!(DataVersion::Unknown(".1".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b".1")));
        assert_eq!(DataVersion::Unknown("1.".to_string()), DataVersion::new_from_attr(testdata::to_attr_owned(b"1.")));
    }

    #[test]
    fn test_data_version_new_from_attr_borrowed() {
        assert_eq!(DataVersion::Version1_0, DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.0")));
        assert_eq!(DataVersion::Version1_1, DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.1")));
        //Test must be rewritten when 1.2 is introduced
        assert_eq!(DataVersion::Unknown("1.2".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.2")));
        assert_eq!(DataVersion::Unknown("something invalid".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b"something invalid")));
        assert_eq!(DataVersion::Unknown("1.1.2".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.1.2")));
        assert_eq!(DataVersion::Unknown("1.1.".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.1.")));
        assert_eq!(DataVersion::Unknown(".1.1".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b".1.1")));
        assert_eq!(DataVersion::Unknown(".1".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b".1")));
        assert_eq!(DataVersion::Unknown("1.".to_string()), DataVersion::new_from_attr(testdata::to_attr_borrowed(b"1.")));
    }

    #[test]
    fn test_data_version_dummy() {
        assert_eq!(DataVersion::dummy(), DataVersion::Unknown("?".to_string()));
    }
}