use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::DMXChannels;
use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::FTMacros;
use crate::gdtf_v_1::fixture_type::dmx_modes::relations::Relations;

pub mod relations;
pub mod ft_macros;
pub mod dmx_channels;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXModes {
    #[serde(rename = "DMXMode")]
    pub items: Option<Option<Vec<DMXMode>>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXMode {
    #[serde(rename = "DMXChannels")]
    pub dmx_channels: DMXChannels,
    #[serde(rename = "Relations")]
    pub relations: Option<Relations>,
    #[serde(rename = "FTMacros")]
    pub ft_macros: Option<FTMacros>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;




    use crate::gdtf_v_1::fixture_type::dmx_modes::relations::test as relations_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::test as dmx_channels_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::test as ft_macros_test;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_acme_ae_610t(gdtf)?;
        dmx_channels_test::test_acme_ae_610t(gdtf)?;
        ft_macros_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_adb_klemantis(gdtf)?;
        dmx_channels_test::test_adb_klemantis(gdtf)?;
        ft_macros_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_adj_mega_tripar(gdtf)?;
        dmx_channels_test::test_adj_mega_tripar(gdtf)?;
        ft_macros_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_adsi_dataton(gdtf)?;
        dmx_channels_test::test_adsi_dataton(gdtf)?;
        ft_macros_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_china_36x10(gdtf)?;
        dmx_channels_test::test_china_36x10(gdtf)?;
        ft_macros_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_jb_lighting_p12(gdtf)?;
        dmx_channels_test::test_jb_lighting_p12(gdtf)?;
        ft_macros_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_robe_robin_viva_cmy(gdtf)?;
        dmx_channels_test::test_robe_robin_viva_cmy(gdtf)?;
        ft_macros_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_sgm_g7_spot(gdtf)?;
        dmx_channels_test::test_sgm_g7_spot(gdtf)?;
        ft_macros_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_sgm_p6(gdtf)?;
        dmx_channels_test::test_sgm_p6(gdtf)?;
        ft_macros_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_shenzhen_mini_led_spot(gdtf)?;
        dmx_channels_test::test_shenzhen_mini_led_spot(gdtf)?;
        ft_macros_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        relations_test::test_stairville_fan_200(gdtf)?;
        dmx_channels_test::test_stairville_fan_200(gdtf)?;
        ft_macros_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}