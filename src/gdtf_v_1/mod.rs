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

    use crate::gdtf_v_1::fixture_type::test as fixture_type_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::data_version::DataVersion;

    #[test]
    fn test_acme_ae_610t() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ACME@ACME_AE-610_BEAM@ACME_AE-610_BEAM.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_0));
        fixture_type_test::test_acme_ae_610t(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_adb_klemantis() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ADB@Klemantis_AS_1000@Revision_1.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_0));
        fixture_type_test::test_adb_klemantis(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_adj_mega_tripar() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ADJ@Mega_TriPar_Profile_Plus@28122021.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_adj_mega_tripar(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_adsi_dataton() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/ADSI_DATATON@WO_ADSI@V0.1.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_adsi_dataton(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_china_36x10() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/China@36x10_Wash_LED_Moving_Light@2022-02-23-3.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_china_36x10(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_jb_lighting_p12() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/JB-Lighting@P12_Spot_HP@V_1.15.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_jb_lighting_p12(&gdtf)?;
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

    #[test]
    fn test_sgm_p6() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/SGM_Light@P-6@Rev_B.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_sgm_p6(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_shenzhen_mini_led_spot() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/Shenzhen@Mini_Led_Spot_60W@REV1.0TEST.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_shenzhen_mini_led_spot(&gdtf)?;
        return Ok(());
    }

    #[test]
    fn test_stairville_fan_200() -> Result<(), GdtfError> {
        let path: &Path = Path::new("test/StairVille@Stairville_Fan-200_DMX@Init_-_Rev1.gdtf");
        let gdtf = GdtfV1::open_file(path)?;
        assert!(matches!(gdtf.data_version,DataVersion::Version1_1));
        fixture_type_test::test_stairville_fan_200(&gdtf)?;
        return Ok(());
    }
}
