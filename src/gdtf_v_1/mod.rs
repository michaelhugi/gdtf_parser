use std::fs::File;
use std::io::Read;
use std::path::Path;

use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::FixtureType;
use crate::utils::errors::GdtfError;
use crate::utils::units::data_version::DataVersion;

pub mod fixture_type;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct GdtfV1 {
    #[serde(rename = "FixtureType")]
    pub fixture_type: FixtureType,
    #[serde(rename = "DataVersion")]
    pub data_version: DataVersion,
}

impl GdtfV1 {
    pub fn open_file(file_path: &Path) -> Result<Self, GdtfError> {
        let mut archive = zip::ZipArchive::new(File::open(file_path)?)?;
        let mut description_xml = String::new();
        archive
            .by_name("description.xml")?
            .read_to_string(&mut description_xml)?;
        //Gdtf::deserialize()
        let raw_model = quick_xml::de::from_str(&description_xml)?;
        Ok(raw_model)
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::gdtf_v_1::fixture_type::fixture_type_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_acme_ae_610t() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_0));
        fixture_type_test::test_acme_ae_610_beam(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_jb_12_spot() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/JB-Lighting@P12_Spot_HP@V_1.15.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_jb_12_spot_hp(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_robe_robin_viva_cmy() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/Robe_Lighting@Robin_Viva_CMY@13042021.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_robe_robin_viva_cmy(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_sgm_g7_spot() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/SGM_Light@G-7_Spot@Rev_A.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_sgm_g7_spot(&gdtf)?;
        return Ok(());
    }
}