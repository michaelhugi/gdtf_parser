use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::geometries::axis::Axis;
use crate::gdtf_v_1::fixture_type::geometries::beam::Beam;
use crate::gdtf_v_1::fixture_type::geometries::filter_beam::FilterBeam;
use crate::gdtf_v_1::fixture_type::geometries::filter_color::FilterColor;
use crate::gdtf_v_1::fixture_type::geometries::filter_gobo::FilterGobo;
use crate::gdtf_v_1::fixture_type::geometries::filter_shaper::FilterShaper;
use crate::gdtf_v_1::fixture_type::geometries::geometry::Geometry;
use crate::gdtf_v_1::fixture_type::geometries::geometry_reference::GeometryReference;
use crate::gdtf_v_1::fixture_type::geometries::media_server_camera::MediaServerCamera;
use crate::gdtf_v_1::fixture_type::geometries::media_server_layer::MediaServerLayer;
use crate::gdtf_v_1::fixture_type::geometries::media_server_master::MediaServerMaster;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct Display {
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