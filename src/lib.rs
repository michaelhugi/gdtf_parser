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
//!     assert_eq!(gdtf.fixture_type_old.name, Name::new("ACME AE-610 BEAM")?);
//!     assert_eq!(gdtf.fixture_type_old.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
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
//!     assert_eq!(gdtf.fixture_type_old.name, Name::new("ACME AE-610 BEAM")?);
//!     assert_eq!(gdtf.fixture_type_old.attribute_definitions.attributes.get(&AttributeName::Gobo_n_WheelSpin(1)).unwrap().physical_unit, PhysicalUnit::AngularSpeed);
//!     Ok(())
//! }
//! ```
//!
//!
extern crate core;

use std::convert::TryFrom;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use crate::gdtf_v_1::GdtfV1;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::units::data_version::DataVersion;

pub mod utils;
pub mod gdtf_v_1;

///Describes the hierarchical and logical structure and controls of any type of controllable device (e.g. luminaires, fog machines, etc.) in the lighting and entertainment industry.
#[derive(Debug, PartialEq, Clone, Deserialize)]
#[serde(tag = "DataVersion")]
pub enum Gdtf {
    #[serde(rename(deserialize = "1.0"))]
    Version1_0(GdtfV1),
    #[serde(rename(deserialize = "1.1"))]
    Version1_1(GdtfV1),
}

impl Gdtf {
    pub fn open_file(file_path: &Path) -> Result<Self, GdtfError> {
        let mut archive = zip::ZipArchive::new(File::open(file_path)?)?;
        let mut description_xml = String::new();
        archive
            .by_name("description.xml")?
            .read_to_string(&mut description_xml)?;
        //Gdtf::deserialize()
        let raw_model = quick_xml::de::from_str(&description_xml);
        Ok(raw_model?)
    }
    pub fn data_version(&self) -> DataVersion {
        match self {
            Gdtf::Version1_0(_) => DataVersion::Version1_0,
            Gdtf::Version1_1(_) => DataVersion::Version1_1
        }
    }
}


#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::path::Path;

    use quick_xml::DeError;

    use crate::{DataVersion, Gdtf, GdtfError, GdtfV1};
    use crate::gdtf_v_1::tests;

    fn unwrap_v_1(gdtf: Gdtf) -> Result<GdtfV1, GdtfError> {
        match gdtf {
            Gdtf::Version1_0(gdtf) => Ok(gdtf),
            Gdtf::Version1_1(gdtf) => Ok(gdtf),
        }
    }

    #[test]
    fn test_acme_ae_610t() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
        let gdtf = Gdtf::open_file(path)?;
        assert!(matches!(gdtf.data_version(),DataVersion::Version1_0));
        let gdtf = unwrap_v_1(gdtf)?;
        tests::test_acme_ae_610_beam(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_jb_12_spot() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/JB-Lighting@P12_Spot_HP@V_1.15.gdtf");
        let gdtf = Gdtf::open_file(path)?;
        assert!(matches!(gdtf.data_version(),DataVersion::Version1_1));
        let gdtf = unwrap_v_1(gdtf)?;
        tests::test_jb_12_spot_hp(&gdtf)?;
        return Ok(());
    }


    #[test]
    fn test_robe_robin_viva_cmy() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/Robe_Lighting@Robin_Viva_CMY@13042021.gdtf");
        let gdtf = Gdtf::open_file(path)?;
        assert!(matches!(gdtf.data_version(),DataVersion::Version1_1));
        let gdtf = unwrap_v_1(gdtf)?;
        tests::test_robe_robin_viva_cmy(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_sgm_g7_spot() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/SGM_Light@G-7_Spot@Rev_A.gdtf");
        let gdtf = Gdtf::open_file(path)?;
        assert!(matches!(gdtf.data_version(),DataVersion::Version1_1));
        let gdtf = unwrap_v_1(gdtf)?;
        tests::test_sgm_g7_spot(&gdtf)?;
        return Ok(());
    }
}