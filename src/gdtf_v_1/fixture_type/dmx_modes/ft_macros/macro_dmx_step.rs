use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::dmx_modes::ft_macros::macro_dmx_value::MacroDMXValue;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct MacroDMXStep {
    #[serde(rename = "MacroDMXValue")]
    pub dmx_values: Option<Vec<MacroDMXValue>>,
}

#[cfg(test)]
pub(crate) mod test {

    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;





    pub(crate) fn test_acme_ae_610t(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_china_36x10(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(_gdtf: &GdtfV1) -> Result<(), GdtfError> {
        return Ok(());
    }
}