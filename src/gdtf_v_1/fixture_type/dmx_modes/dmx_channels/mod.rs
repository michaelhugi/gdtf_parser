use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::logical_channel::LogicalChannel;

pub mod channel_function;
pub mod channel_set;
pub mod logical_channel;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXChannels {
    #[serde(rename = "DMXChannel")]
    pub items: Option<Vec<DMXChannel>>,
}

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct DMXChannel {
    #[serde(rename = "LogicalChannel")]
    pub logical_channels: Option<Vec<LogicalChannel>>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;




    use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::channel_function::test as channel_function_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::channel_set::test as channel_set_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::dmx_channels::logical_channel::test as logical_channel_test;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_acme_ae_610t(gdtf)?;
        channel_set_test::test_acme_ae_610t(gdtf)?;
        logical_channel_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_adb_klemantis(gdtf)?;
        channel_set_test::test_adb_klemantis(gdtf)?;
        logical_channel_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_adj_mega_tripar(gdtf)?;
        channel_set_test::test_adj_mega_tripar(gdtf)?;
        logical_channel_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_adsi_dataton(gdtf)?;
        channel_set_test::test_adsi_dataton(gdtf)?;
        logical_channel_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_china_36x10(gdtf)?;
        channel_set_test::test_china_36x10(gdtf)?;
        logical_channel_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_jb_lighting_p12(gdtf)?;
        channel_set_test::test_jb_lighting_p12(gdtf)?;
        logical_channel_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_robe_robin_viva_cmy(gdtf)?;
        channel_set_test::test_robe_robin_viva_cmy(gdtf)?;
        logical_channel_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_sgm_g7_spot(gdtf)?;
        channel_set_test::test_sgm_g7_spot(gdtf)?;
        logical_channel_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_sgm_p6(gdtf)?;
        channel_set_test::test_sgm_p6(gdtf)?;
        logical_channel_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_shenzhen_mini_led_spot(gdtf)?;
        channel_set_test::test_shenzhen_mini_led_spot(gdtf)?;
        logical_channel_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        channel_function_test::test_stairville_fan_200(gdtf)?;
        channel_set_test::test_stairville_fan_200(gdtf)?;
        logical_channel_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}