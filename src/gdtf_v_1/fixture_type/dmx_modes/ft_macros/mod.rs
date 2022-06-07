use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx::MacroDmx;

pub mod macro_dmx_step;
pub mod macro_dmx_value;
pub mod macro_dmx;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FTMacros {
    #[serde(rename = "FTMacro")]
    pub items: Option<Vec<FTMacro>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FTMacro {
    #[serde(rename = "MacroDmx")]
    pub macro_dmx: Option<MacroDmx>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx::test as macro_dmx_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx_step::test as macro_dmx_step;
    use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx_value::test as macro_dmx_value;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;





    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_acme_ae_610t(gdtf)?;
        macro_dmx_step::test_acme_ae_610t(gdtf)?;
        macro_dmx_value::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_adb_klemantis(gdtf)?;
        macro_dmx_step::test_adb_klemantis(gdtf)?;
        macro_dmx_value::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_adj_mega_tripar(gdtf)?;
        macro_dmx_step::test_adj_mega_tripar(gdtf)?;
        macro_dmx_value::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_adsi_dataton(gdtf)?;
        macro_dmx_step::test_adsi_dataton(gdtf)?;
        macro_dmx_value::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_china_36x10(gdtf)?;
        macro_dmx_step::test_china_36x10(gdtf)?;
        macro_dmx_value::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_jb_lighting_p12(gdtf)?;
        macro_dmx_step::test_jb_lighting_p12(gdtf)?;
        macro_dmx_value::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_robe_robin_viva_cmy(gdtf)?;
        macro_dmx_step::test_robe_robin_viva_cmy(gdtf)?;
        macro_dmx_value::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_sgm_g7_spot(gdtf)?;
        macro_dmx_step::test_sgm_g7_spot(gdtf)?;
        macro_dmx_value::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_sgm_p6(gdtf)?;
        macro_dmx_step::test_sgm_p6(gdtf)?;
        macro_dmx_value::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_shenzhen_mini_led_spot(gdtf)?;
        macro_dmx_step::test_shenzhen_mini_led_spot(gdtf)?;
        macro_dmx_value::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        macro_dmx_test::test_stairville_fan_200(gdtf)?;
        macro_dmx_step::test_stairville_fan_200(gdtf)?;
        macro_dmx_value::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}