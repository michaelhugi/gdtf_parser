use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::geometries::axis::Axis;
use crate::gdtf_v_1::fixture_type::geometries::beam::Beam;
use crate::gdtf_v_1::fixture_type::geometries::display::Display;
use crate::gdtf_v_1::fixture_type::geometries::filter_beam::FilterBeam;
use crate::gdtf_v_1::fixture_type::geometries::filter_color::FilterColor;
use crate::gdtf_v_1::fixture_type::geometries::filter_gobo::FilterGobo;
use crate::gdtf_v_1::fixture_type::geometries::filter_shaper::FilterShaper;
use crate::gdtf_v_1::fixture_type::geometries::geometry::Geometry;
use crate::gdtf_v_1::fixture_type::geometries::geometry_reference::GeometryReference;
use crate::gdtf_v_1::fixture_type::geometries::media_server_camera::MediaServerCamera;
use crate::gdtf_v_1::fixture_type::geometries::media_server_layer::MediaServerLayer;
use crate::gdtf_v_1::fixture_type::geometries::media_server_master::MediaServerMaster;

mod axis;
mod beam;
mod display;
mod filter_beam;
mod filter_color;
mod filter_gobo;
mod filter_shaper;
mod geometry;
mod geometry_reference;
mod media_server_camera;
mod media_server_layer;
mod media_server_master;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Geometries {
    #[serde(rename = "Geometry")]
    pub geometries: Option<Vec<Geometry>>,
    #[serde(rename = "Axis")]
    pub axis: Option<Vec<Axis>>,
    #[serde(rename = "FilterBeam")]
    pub filter_beams: Option<Vec<FilterBeam>>,
    #[serde(rename = "FilterColor")]
    pub filter_colors: Option<Vec<FilterColor>>,
    #[serde(rename = "FilterGobo")]
    pub filter_gobos: Option<Vec<FilterGobo>>,
    #[serde(rename = "FilterShaper")]
    pub filter_shapers: Option<Vec<FilterShaper>>,
    #[serde(rename = "Beam")]
    pub beams: Option<Vec<Beam>>,
    #[serde(rename = "MediaServerLayer")]
    pub media_server_layers: Option<Vec<MediaServerLayer>>,
    #[serde(rename = "MediaServerCamera")]
    pub media_server_cameras: Option<Vec<MediaServerCamera>>,
    #[serde(rename = "MediaServerMaster")]
    pub media_server_masters: Option<Vec<MediaServerMaster>>,
    #[serde(rename = "Display")]
    pub displays: Option<Vec<Display>>,
    #[serde(rename = "GeometryReference")]
    pub geometry_references: Option<Vec<GeometryReference>>,
}

#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::fixture_type::geometries::axis::test as axis_test;
    use crate::gdtf_v_1::fixture_type::geometries::beam::test as beam_test;
    use crate::gdtf_v_1::fixture_type::geometries::display::test as display_test;
    use crate::gdtf_v_1::fixture_type::geometries::filter_beam::test as filter_beam_test;
    use crate::gdtf_v_1::fixture_type::geometries::filter_color::test as filter_color_test;
    use crate::gdtf_v_1::fixture_type::geometries::filter_gobo::test as filter_gobo_test;
    use crate::gdtf_v_1::fixture_type::geometries::filter_shaper::test as filter_shaper_tst;
    use crate::gdtf_v_1::fixture_type::geometries::geometry::test as geometry_test;
    use crate::gdtf_v_1::fixture_type::geometries::geometry_reference::test as geometry_reference_test;
    use crate::gdtf_v_1::fixture_type::geometries::media_server_camera::test as media_server_camera_test;
    use crate::gdtf_v_1::fixture_type::geometries::media_server_layer::test as media_server_layer_test;
    use crate::gdtf_v_1::fixture_type::geometries::media_server_master::test as media_server_master_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;





    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_acme_ae_610t(gdtf)?;
        beam_test::test_acme_ae_610t(gdtf)?;
        display_test::test_acme_ae_610t(gdtf)?;
        filter_beam_test::test_acme_ae_610t(gdtf)?;
        filter_color_test::test_acme_ae_610t(gdtf)?;
        filter_gobo_test::test_acme_ae_610t(gdtf)?;
        filter_shaper_tst::test_acme_ae_610t(gdtf)?;
        geometry_test::test_acme_ae_610t(gdtf)?;
        geometry_reference_test::test_acme_ae_610t(gdtf)?;
        media_server_camera_test::test_acme_ae_610t(gdtf)?;
        media_server_layer_test::test_acme_ae_610t(gdtf)?;
        media_server_master_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_adb_klemantis(gdtf)?;
        beam_test::test_adb_klemantis(gdtf)?;
        display_test::test_adb_klemantis(gdtf)?;
        filter_beam_test::test_adb_klemantis(gdtf)?;
        filter_color_test::test_adb_klemantis(gdtf)?;
        filter_gobo_test::test_adb_klemantis(gdtf)?;
        filter_shaper_tst::test_adb_klemantis(gdtf)?;
        geometry_test::test_adb_klemantis(gdtf)?;
        geometry_reference_test::test_adb_klemantis(gdtf)?;
        media_server_camera_test::test_adb_klemantis(gdtf)?;
        media_server_layer_test::test_adb_klemantis(gdtf)?;
        media_server_master_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_adj_mega_tripar(gdtf)?;
        beam_test::test_adj_mega_tripar(gdtf)?;
        display_test::test_adj_mega_tripar(gdtf)?;
        filter_beam_test::test_adj_mega_tripar(gdtf)?;
        filter_color_test::test_adj_mega_tripar(gdtf)?;
        filter_gobo_test::test_adj_mega_tripar(gdtf)?;
        filter_shaper_tst::test_adj_mega_tripar(gdtf)?;
        geometry_test::test_adj_mega_tripar(gdtf)?;
        geometry_reference_test::test_adj_mega_tripar(gdtf)?;
        media_server_camera_test::test_adj_mega_tripar(gdtf)?;
        media_server_layer_test::test_adj_mega_tripar(gdtf)?;
        media_server_master_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_adsi_dataton(gdtf)?;
        beam_test::test_adsi_dataton(gdtf)?;
        display_test::test_adsi_dataton(gdtf)?;
        filter_beam_test::test_adsi_dataton(gdtf)?;
        filter_color_test::test_adsi_dataton(gdtf)?;
        filter_gobo_test::test_adsi_dataton(gdtf)?;
        filter_shaper_tst::test_adsi_dataton(gdtf)?;
        geometry_test::test_adsi_dataton(gdtf)?;
        geometry_reference_test::test_adsi_dataton(gdtf)?;
        media_server_camera_test::test_adsi_dataton(gdtf)?;
        media_server_layer_test::test_adsi_dataton(gdtf)?;
        media_server_master_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_china_36x10(gdtf)?;
        beam_test::test_china_36x10(gdtf)?;
        display_test::test_china_36x10(gdtf)?;
        filter_beam_test::test_china_36x10(gdtf)?;
        filter_color_test::test_china_36x10(gdtf)?;
        filter_gobo_test::test_china_36x10(gdtf)?;
        filter_shaper_tst::test_china_36x10(gdtf)?;
        geometry_test::test_china_36x10(gdtf)?;
        geometry_reference_test::test_china_36x10(gdtf)?;
        media_server_camera_test::test_china_36x10(gdtf)?;
        media_server_layer_test::test_china_36x10(gdtf)?;
        media_server_master_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_jb_lighting_p12(gdtf)?;
        beam_test::test_jb_lighting_p12(gdtf)?;
        display_test::test_jb_lighting_p12(gdtf)?;
        filter_beam_test::test_jb_lighting_p12(gdtf)?;
        filter_color_test::test_jb_lighting_p12(gdtf)?;
        filter_gobo_test::test_jb_lighting_p12(gdtf)?;
        filter_shaper_tst::test_jb_lighting_p12(gdtf)?;
        geometry_test::test_jb_lighting_p12(gdtf)?;
        geometry_reference_test::test_jb_lighting_p12(gdtf)?;
        media_server_camera_test::test_jb_lighting_p12(gdtf)?;
        media_server_layer_test::test_jb_lighting_p12(gdtf)?;
        media_server_master_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_robe_robin_viva_cmy(gdtf)?;
        beam_test::test_robe_robin_viva_cmy(gdtf)?;
        display_test::test_robe_robin_viva_cmy(gdtf)?;
        filter_beam_test::test_robe_robin_viva_cmy(gdtf)?;
        filter_color_test::test_robe_robin_viva_cmy(gdtf)?;
        filter_gobo_test::test_robe_robin_viva_cmy(gdtf)?;
        filter_shaper_tst::test_robe_robin_viva_cmy(gdtf)?;
        geometry_test::test_robe_robin_viva_cmy(gdtf)?;
        geometry_reference_test::test_robe_robin_viva_cmy(gdtf)?;
        media_server_camera_test::test_robe_robin_viva_cmy(gdtf)?;
        media_server_layer_test::test_robe_robin_viva_cmy(gdtf)?;
        media_server_master_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_sgm_g7_spot(gdtf)?;
        beam_test::test_sgm_g7_spot(gdtf)?;
        display_test::test_sgm_g7_spot(gdtf)?;
        filter_beam_test::test_sgm_g7_spot(gdtf)?;
        filter_color_test::test_sgm_g7_spot(gdtf)?;
        filter_gobo_test::test_sgm_g7_spot(gdtf)?;
        filter_shaper_tst::test_sgm_g7_spot(gdtf)?;
        geometry_test::test_sgm_g7_spot(gdtf)?;
        geometry_reference_test::test_sgm_g7_spot(gdtf)?;
        media_server_camera_test::test_sgm_g7_spot(gdtf)?;
        media_server_layer_test::test_sgm_g7_spot(gdtf)?;
        media_server_master_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_sgm_p6(gdtf)?;
        beam_test::test_sgm_p6(gdtf)?;
        display_test::test_sgm_p6(gdtf)?;
        filter_beam_test::test_sgm_p6(gdtf)?;
        filter_color_test::test_sgm_p6(gdtf)?;
        filter_gobo_test::test_sgm_p6(gdtf)?;
        filter_shaper_tst::test_sgm_p6(gdtf)?;
        geometry_test::test_sgm_p6(gdtf)?;
        geometry_reference_test::test_sgm_p6(gdtf)?;
        media_server_camera_test::test_sgm_p6(gdtf)?;
        media_server_layer_test::test_sgm_p6(gdtf)?;
        media_server_master_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_shenzhen_mini_led_spot(gdtf)?;
        beam_test::test_shenzhen_mini_led_spot(gdtf)?;
        display_test::test_shenzhen_mini_led_spot(gdtf)?;
        filter_beam_test::test_shenzhen_mini_led_spot(gdtf)?;
        filter_color_test::test_shenzhen_mini_led_spot(gdtf)?;
        filter_gobo_test::test_shenzhen_mini_led_spot(gdtf)?;
        filter_shaper_tst::test_shenzhen_mini_led_spot(gdtf)?;
        geometry_test::test_shenzhen_mini_led_spot(gdtf)?;
        geometry_reference_test::test_shenzhen_mini_led_spot(gdtf)?;
        media_server_camera_test::test_shenzhen_mini_led_spot(gdtf)?;
        media_server_layer_test::test_shenzhen_mini_led_spot(gdtf)?;
        media_server_master_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        axis_test::test_stairville_fan_200(gdtf)?;
        beam_test::test_stairville_fan_200(gdtf)?;
        display_test::test_stairville_fan_200(gdtf)?;
        filter_beam_test::test_stairville_fan_200(gdtf)?;
        filter_color_test::test_stairville_fan_200(gdtf)?;
        filter_gobo_test::test_stairville_fan_200(gdtf)?;
        filter_shaper_tst::test_stairville_fan_200(gdtf)?;
        geometry_test::test_stairville_fan_200(gdtf)?;
        geometry_reference_test::test_stairville_fan_200(gdtf)?;
        media_server_camera_test::test_stairville_fan_200(gdtf)?;
        media_server_layer_test::test_stairville_fan_200(gdtf)?;
        media_server_master_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}