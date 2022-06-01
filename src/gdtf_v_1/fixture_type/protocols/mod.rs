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