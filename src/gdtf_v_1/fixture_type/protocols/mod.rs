use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::protocols::art_net::ArtNet;
use crate::gdtf_v_1::fixture_type::protocols::citp::CITP;
use crate::gdtf_v_1::fixture_type::protocols::ftrdm::FTRDM;
use crate::gdtf_v_1::fixture_type::protocols::open_sound_control::OpenSoundControl;
use crate::gdtf_v_1::fixture_type::protocols::posi_stage_net::PosiStageNet;
use crate::gdtf_v_1::fixture_type::protocols::sacn::SACN;

pub mod ftrdm;
pub mod art_net;
pub mod sacn;
pub mod posi_stage_net;
pub mod open_sound_control;
pub mod citp;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Protocols {
    pub ftrdm: Option<FTRDM>,
    pub art_net: Option<ArtNet>,
    pub s_acn: Option<SACN>,
    pub posi_stage_net: Option<PosiStageNet>,
    pub open_sound_control: Option<OpenSoundControl>,
    pub citp: Option<CITP>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;




    use crate::gdtf_v_1::fixture_type::protocols::art_net::test as art_net_test;
    use crate::gdtf_v_1::fixture_type::protocols::citp::test as citp_test;
    use crate::gdtf_v_1::fixture_type::protocols::ftrdm::test as ftrdm_test;
    use crate::gdtf_v_1::fixture_type::protocols::open_sound_control::test as open_sound_control_test;
    use crate::gdtf_v_1::fixture_type::protocols::posi_stage_net::test as posi_stage_net_test;
    use crate::gdtf_v_1::fixture_type::protocols::sacn::test as sacn_test;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_acme_ae_610t(gdtf)?;
        citp_test::test_acme_ae_610t(gdtf)?;
        ftrdm_test::test_acme_ae_610t(gdtf)?;
        open_sound_control_test::test_acme_ae_610t(gdtf)?;
        posi_stage_net_test::test_acme_ae_610t(gdtf)?;
        sacn_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_adb_klemantis(gdtf)?;
        citp_test::test_adb_klemantis(gdtf)?;
        ftrdm_test::test_adb_klemantis(gdtf)?;
        open_sound_control_test::test_adb_klemantis(gdtf)?;
        posi_stage_net_test::test_adb_klemantis(gdtf)?;
        sacn_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_adj_mega_tripar(gdtf)?;
        citp_test::test_adj_mega_tripar(gdtf)?;
        ftrdm_test::test_adj_mega_tripar(gdtf)?;
        open_sound_control_test::test_adj_mega_tripar(gdtf)?;
        posi_stage_net_test::test_adj_mega_tripar(gdtf)?;
        sacn_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_adsi_dataton(gdtf)?;
        citp_test::test_adsi_dataton(gdtf)?;
        ftrdm_test::test_adsi_dataton(gdtf)?;
        open_sound_control_test::test_adsi_dataton(gdtf)?;
        posi_stage_net_test::test_adsi_dataton(gdtf)?;
        sacn_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_china_36x10(gdtf)?;
        citp_test::test_china_36x10(gdtf)?;
        ftrdm_test::test_china_36x10(gdtf)?;
        open_sound_control_test::test_china_36x10(gdtf)?;
        posi_stage_net_test::test_china_36x10(gdtf)?;
        sacn_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_jb_lighting_p12(gdtf)?;
        citp_test::test_jb_lighting_p12(gdtf)?;
        ftrdm_test::test_jb_lighting_p12(gdtf)?;
        open_sound_control_test::test_jb_lighting_p12(gdtf)?;
        posi_stage_net_test::test_jb_lighting_p12(gdtf)?;
        sacn_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_robe_robin_viva_cmy(gdtf)?;
        citp_test::test_robe_robin_viva_cmy(gdtf)?;
        ftrdm_test::test_robe_robin_viva_cmy(gdtf)?;
        open_sound_control_test::test_robe_robin_viva_cmy(gdtf)?;
        posi_stage_net_test::test_robe_robin_viva_cmy(gdtf)?;
        sacn_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_sgm_g7_spot(gdtf)?;
        citp_test::test_sgm_g7_spot(gdtf)?;
        ftrdm_test::test_sgm_g7_spot(gdtf)?;
        open_sound_control_test::test_sgm_g7_spot(gdtf)?;
        posi_stage_net_test::test_sgm_g7_spot(gdtf)?;
        sacn_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_sgm_p6(gdtf)?;
        citp_test::test_sgm_p6(gdtf)?;
        ftrdm_test::test_sgm_p6(gdtf)?;
        open_sound_control_test::test_sgm_p6(gdtf)?;
        posi_stage_net_test::test_sgm_p6(gdtf)?;
        sacn_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_shenzhen_mini_led_spot(gdtf)?;
        citp_test::test_shenzhen_mini_led_spot(gdtf)?;
        ftrdm_test::test_shenzhen_mini_led_spot(gdtf)?;
        open_sound_control_test::test_shenzhen_mini_led_spot(gdtf)?;
        posi_stage_net_test::test_shenzhen_mini_led_spot(gdtf)?;
        sacn_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        art_net_test::test_stairville_fan_200(gdtf)?;
        citp_test::test_stairville_fan_200(gdtf)?;
        ftrdm_test::test_stairville_fan_200(gdtf)?;
        open_sound_control_test::test_stairville_fan_200(gdtf)?;
        posi_stage_net_test::test_stairville_fan_200(gdtf)?;
        sacn_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}