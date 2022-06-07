use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::geometries::axis::Axis;
use crate::gdtf_v_1::fixture_type::geometries::beam::Beam;
use crate::gdtf_v_1::fixture_type::geometries::display::Display;
use crate::gdtf_v_1::fixture_type::geometries::filter_beam::FilterBeam;
use crate::gdtf_v_1::fixture_type::geometries::filter_color::FilterColor;
use crate::gdtf_v_1::fixture_type::geometries::filter_shaper::FilterShaper;
use crate::gdtf_v_1::fixture_type::geometries::geometry::Geometry;
use crate::gdtf_v_1::fixture_type::geometries::geometry_reference::GeometryReference;
use crate::gdtf_v_1::fixture_type::geometries::media_server_camera::MediaServerCamera;
use crate::gdtf_v_1::fixture_type::geometries::media_server_layer::MediaServerLayer;
use crate::gdtf_v_1::fixture_type::geometries::media_server_master::MediaServerMaster;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FilterGobo {
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