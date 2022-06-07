use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::physical_descriptions::color_space::ColorSpace;
use crate::gdtf_v_1::fixture_type::physical_descriptions::connectors::Connectors;
use crate::gdtf_v_1::fixture_type::physical_descriptions::cris::CRIs;
use crate::gdtf_v_1::fixture_type::physical_descriptions::dmx_profiles::DMXProfiles;
use crate::gdtf_v_1::fixture_type::physical_descriptions::emitters::Emitters;
use crate::gdtf_v_1::fixture_type::physical_descriptions::filters::Filters;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::Properties;

pub mod dmx_profiles;
pub mod color_space;
pub mod emitters;
pub mod filters;
pub mod cris;
pub mod connectors;
pub mod properties;
pub mod measurement;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct PhysicalDescriptions {
    #[serde(rename = "Emitters")]
    pub emitters: Option<Emitters>,
    #[serde(rename = "Filters")]
    pub filters: Option<Filters>,
    #[serde(rename = "ColorSpace")]
    pub color_space: Option<ColorSpace>,
    #[serde(rename = "DMXProfiles")]
    pub dmx_profiles: Option<DMXProfiles>,
    #[serde(rename = "CRIs")]
    pub cris: Option<CRIs>,
    #[serde(rename = "Connectors")]
    pub connectors: Option<Connectors>,
    #[serde(rename = "Properties")]
    pub properties: Option<Properties>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::fixture_type::physical_descriptions::color_space::test as color_space_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::connectors::test as connectors_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::cris::test as cris_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::dmx_profiles::test as dmx_profiles_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::emitters::test as emitters_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::filters::test as filters_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::measurement::test as measurement_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::test as properties_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;





    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_acme_ae_610t(gdtf)?;
        color_space_test::test_acme_ae_610t(gdtf)?;
        connectors_test::test_acme_ae_610t(gdtf)?;
        cris_test::test_acme_ae_610t(gdtf)?;
        dmx_profiles_test::test_acme_ae_610t(gdtf)?;
        emitters_test::test_acme_ae_610t(gdtf)?;
        filters_test::test_acme_ae_610t(gdtf)?;
        measurement_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_adb_klemantis(gdtf)?;
        color_space_test::test_adb_klemantis(gdtf)?;
        connectors_test::test_adb_klemantis(gdtf)?;
        cris_test::test_adb_klemantis(gdtf)?;
        dmx_profiles_test::test_adb_klemantis(gdtf)?;
        emitters_test::test_adb_klemantis(gdtf)?;
        filters_test::test_adb_klemantis(gdtf)?;
        measurement_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_adj_mega_tripar(gdtf)?;
        color_space_test::test_adj_mega_tripar(gdtf)?;
        connectors_test::test_adj_mega_tripar(gdtf)?;
        cris_test::test_adj_mega_tripar(gdtf)?;
        dmx_profiles_test::test_adj_mega_tripar(gdtf)?;
        emitters_test::test_adj_mega_tripar(gdtf)?;
        filters_test::test_adj_mega_tripar(gdtf)?;
        measurement_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_adsi_dataton(gdtf)?;
        color_space_test::test_adsi_dataton(gdtf)?;
        connectors_test::test_adsi_dataton(gdtf)?;
        cris_test::test_adsi_dataton(gdtf)?;
        dmx_profiles_test::test_adsi_dataton(gdtf)?;
        emitters_test::test_adsi_dataton(gdtf)?;
        filters_test::test_adsi_dataton(gdtf)?;
        measurement_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_china_36x10(gdtf)?;
        color_space_test::test_china_36x10(gdtf)?;
        connectors_test::test_china_36x10(gdtf)?;
        cris_test::test_china_36x10(gdtf)?;
        dmx_profiles_test::test_china_36x10(gdtf)?;
        emitters_test::test_china_36x10(gdtf)?;
        filters_test::test_china_36x10(gdtf)?;
        measurement_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_jb_lighting_p12(gdtf)?;
        color_space_test::test_jb_lighting_p12(gdtf)?;
        connectors_test::test_jb_lighting_p12(gdtf)?;
        cris_test::test_jb_lighting_p12(gdtf)?;
        dmx_profiles_test::test_jb_lighting_p12(gdtf)?;
        emitters_test::test_jb_lighting_p12(gdtf)?;
        filters_test::test_jb_lighting_p12(gdtf)?;
        measurement_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_robe_robin_viva_cmy(gdtf)?;
        color_space_test::test_robe_robin_viva_cmy(gdtf)?;
        connectors_test::test_robe_robin_viva_cmy(gdtf)?;
        cris_test::test_robe_robin_viva_cmy(gdtf)?;
        dmx_profiles_test::test_robe_robin_viva_cmy(gdtf)?;
        emitters_test::test_robe_robin_viva_cmy(gdtf)?;
        filters_test::test_robe_robin_viva_cmy(gdtf)?;
        measurement_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_sgm_g7_spot(gdtf)?;
        color_space_test::test_sgm_g7_spot(gdtf)?;
        connectors_test::test_sgm_g7_spot(gdtf)?;
        cris_test::test_sgm_g7_spot(gdtf)?;
        dmx_profiles_test::test_sgm_g7_spot(gdtf)?;
        emitters_test::test_sgm_g7_spot(gdtf)?;
        filters_test::test_sgm_g7_spot(gdtf)?;
        measurement_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_sgm_p6(gdtf)?;
        color_space_test::test_sgm_p6(gdtf)?;
        connectors_test::test_sgm_p6(gdtf)?;
        cris_test::test_sgm_p6(gdtf)?;
        dmx_profiles_test::test_sgm_p6(gdtf)?;
        emitters_test::test_sgm_p6(gdtf)?;
        filters_test::test_sgm_p6(gdtf)?;
        measurement_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_shenzhen_mini_led_spot(gdtf)?;
        color_space_test::test_shenzhen_mini_led_spot(gdtf)?;
        connectors_test::test_shenzhen_mini_led_spot(gdtf)?;
        cris_test::test_shenzhen_mini_led_spot(gdtf)?;
        dmx_profiles_test::test_shenzhen_mini_led_spot(gdtf)?;
        emitters_test::test_shenzhen_mini_led_spot(gdtf)?;
        filters_test::test_shenzhen_mini_led_spot(gdtf)?;
        measurement_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        properties_test::test_stairville_fan_200(gdtf)?;
        color_space_test::test_stairville_fan_200(gdtf)?;
        connectors_test::test_stairville_fan_200(gdtf)?;
        cris_test::test_stairville_fan_200(gdtf)?;
        dmx_profiles_test::test_stairville_fan_200(gdtf)?;
        emitters_test::test_stairville_fan_200(gdtf)?;
        filters_test::test_stairville_fan_200(gdtf)?;
        measurement_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}