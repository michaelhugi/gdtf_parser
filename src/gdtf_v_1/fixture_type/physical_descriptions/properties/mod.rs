use serde::Deserialize;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::leg_height::LegHeight;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::operating_temperature::OperatingTemperature;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::power_consumption::PowerConsumption;
use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::weight::Weight;

pub mod leg_height;
pub mod operating_temperature;
pub mod power_consumption;
pub mod weight;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Properties {
    #[serde(rename = "OperatingTemperature")]
    pub operating_temperature: Option<OperatingTemperature>,
    #[serde(rename = "Weight")]
    pub weight: Option<Weight>,
    #[serde(rename = "PowerConsumption")]
    pub power_consumption: Option<Vec<PowerConsumption>>,
    #[serde(rename = "LegHeight")]
    pub leg_height: Option<LegHeight>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;




    use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::leg_height::test as leg_height_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::operating_temperature::test as operating_temperature_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::power_consumption::test as power_consumption_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::properties::weight::test as weight_test;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_acme_ae_610t(gdtf)?;
        operating_temperature_test::test_acme_ae_610t(gdtf)?;
        power_consumption_test::test_acme_ae_610t(gdtf)?;
        weight_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_adb_klemantis(gdtf)?;
        operating_temperature_test::test_adb_klemantis(gdtf)?;
        power_consumption_test::test_adb_klemantis(gdtf)?;
        weight_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_adj_mega_tripar(gdtf)?;
        operating_temperature_test::test_adj_mega_tripar(gdtf)?;
        power_consumption_test::test_adj_mega_tripar(gdtf)?;
        weight_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_adsi_dataton(gdtf)?;
        operating_temperature_test::test_adsi_dataton(gdtf)?;
        power_consumption_test::test_adsi_dataton(gdtf)?;
        weight_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_china_36x10(gdtf)?;
        operating_temperature_test::test_china_36x10(gdtf)?;
        power_consumption_test::test_china_36x10(gdtf)?;
        weight_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_jb_lighting_p12(gdtf)?;
        operating_temperature_test::test_jb_lighting_p12(gdtf)?;
        power_consumption_test::test_jb_lighting_p12(gdtf)?;
        weight_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_robe_robin_viva_cmy(gdtf)?;
        operating_temperature_test::test_robe_robin_viva_cmy(gdtf)?;
        power_consumption_test::test_robe_robin_viva_cmy(gdtf)?;
        weight_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_sgm_g7_spot(gdtf)?;
        operating_temperature_test::test_sgm_g7_spot(gdtf)?;
        power_consumption_test::test_sgm_g7_spot(gdtf)?;
        weight_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_sgm_p6(gdtf)?;
        operating_temperature_test::test_sgm_p6(gdtf)?;
        power_consumption_test::test_sgm_p6(gdtf)?;
        weight_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_shenzhen_mini_led_spot(gdtf)?;
        operating_temperature_test::test_shenzhen_mini_led_spot(gdtf)?;
        power_consumption_test::test_shenzhen_mini_led_spot(gdtf)?;
        weight_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        leg_height_test::test_stairville_fan_200(gdtf)?;
        operating_temperature_test::test_stairville_fan_200(gdtf)?;
        power_consumption_test::test_stairville_fan_200(gdtf)?;
        weight_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}