use serde::Deserialize;

use crate::gdtf_v_1::fixture_type::attribute_definitions::AttributeDefinitions;
use crate::gdtf_v_1::fixture_type::dmx_modes::DMXModes;
use crate::gdtf_v_1::fixture_type::ft_presets::FTPresets;
use crate::gdtf_v_1::fixture_type::geometries::Geometries;
use crate::gdtf_v_1::fixture_type::models::Models;
use crate::gdtf_v_1::fixture_type::physical_descriptions::PhysicalDescriptions;
use crate::gdtf_v_1::fixture_type::protocols::Protocols;
use crate::gdtf_v_1::fixture_type::revisions::Revisions;
use crate::gdtf_v_1::fixture_type::wheels::Wheels;
use crate::utils::units::can_have_children::CanHaveChildren;
use crate::utils::units::guid::Guid;
use crate::utils::units::guid_opt::GuidOpt;
use crate::utils::units::name::Name;
use crate::utils::units::resource::Resource;

pub mod attribute_definitions;
pub mod wheels;
pub mod physical_descriptions;
pub mod models;
pub mod dmx_modes;
pub mod revisions;
pub mod ft_presets;
pub mod protocols;
mod geometries;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub struct FixtureType {
    #[serde(rename = "Name")]
    pub name: Name,
    #[serde(rename = "ShortName")]
    pub short_name: String,
    #[serde(rename = "LongName")]
    pub long_name: String,
    #[serde(rename = "Manufacturer")]
    pub manufacturer: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "FixtureTypeID")]
    pub fixture_type_id: Guid,
    #[serde(rename = "Thumbnail")]
    pub thumbnail: Resource,
    #[serde(rename = "RefFT")]
    pub ref_ft: GuidOpt,
    #[serde(rename = "CanHaveChildren", default)]
    pub can_have_children: CanHaveChildren,
    #[serde(rename = "AttributeDefinitions")]
    pub attribute_definitions: AttributeDefinitions,
    #[serde(rename = "Wheels")]
    pub wheels: Option<Wheels>,
    #[serde(rename = "PhysicalDescriptions")]
    pub physical_descriptions: Option<PhysicalDescriptions>,
    #[serde(rename = "Models")]
    pub models: Option<Models>,
    #[serde(rename = "Geometries")]
    pub geometries: Option<Geometries>,
    #[serde(rename = "DMXModes")]
    pub dmx_modes: DMXModes,
    #[serde(rename = "Revisions")]
    pub revisions: Option<Revisions>,
    #[serde(rename = "FTPresets")]
    pub ft_presets: Option<FTPresets>,
    #[serde(rename = "Protocols")]
    pub protocols: Option<Protocols>,
}


#[cfg(test)]
pub(crate) mod test {
    use crate::gdtf_v_1::fixture_type::attribute_definitions::test as attribute_definitions_test;
    use crate::gdtf_v_1::fixture_type::dmx_modes::test as dmx_modes_test;
    use crate::gdtf_v_1::fixture_type::ft_presets::test as ft_presets_test;
    use crate::gdtf_v_1::fixture_type::geometries::test as geometries_test;
    use crate::gdtf_v_1::fixture_type::models::test as models_test;
    use crate::gdtf_v_1::fixture_type::physical_descriptions::test as physical_descriptions_test;
    use crate::gdtf_v_1::fixture_type::protocols::test as protocols_test;
    use crate::gdtf_v_1::fixture_type::revisions::test as revisions_test;
    use crate::gdtf_v_1::fixture_type::wheels::test as wheels_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;

    pub(crate) fn test_acme_ae_610t(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_acme_ae_610t(gdtf)?;
        dmx_modes_test::test_acme_ae_610t(gdtf)?;
        ft_presets_test::test_acme_ae_610t(gdtf)?;
        geometries_test::test_acme_ae_610t(gdtf)?;
        models_test::test_acme_ae_610t(gdtf)?;
        physical_descriptions_test::test_acme_ae_610t(gdtf)?;
        protocols_test::test_acme_ae_610t(gdtf)?;
        revisions_test::test_acme_ae_610t(gdtf)?;
        wheels_test::test_acme_ae_610t(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adb_klemantis(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_adb_klemantis(gdtf)?;
        dmx_modes_test::test_adb_klemantis(gdtf)?;
        ft_presets_test::test_adb_klemantis(gdtf)?;
        geometries_test::test_adb_klemantis(gdtf)?;
        models_test::test_adb_klemantis(gdtf)?;
        physical_descriptions_test::test_adb_klemantis(gdtf)?;
        protocols_test::test_adb_klemantis(gdtf)?;
        revisions_test::test_adb_klemantis(gdtf)?;
        wheels_test::test_adb_klemantis(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adj_mega_tripar(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_adj_mega_tripar(gdtf)?;
        dmx_modes_test::test_adj_mega_tripar(gdtf)?;
        ft_presets_test::test_adj_mega_tripar(gdtf)?;
        geometries_test::test_adj_mega_tripar(gdtf)?;
        models_test::test_adj_mega_tripar(gdtf)?;
        physical_descriptions_test::test_adj_mega_tripar(gdtf)?;
        protocols_test::test_adj_mega_tripar(gdtf)?;
        revisions_test::test_adj_mega_tripar(gdtf)?;
        wheels_test::test_adj_mega_tripar(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_adsi_dataton(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_adsi_dataton(gdtf)?;
        dmx_modes_test::test_adsi_dataton(gdtf)?;
        ft_presets_test::test_adsi_dataton(gdtf)?;
        geometries_test::test_adsi_dataton(gdtf)?;
        models_test::test_adsi_dataton(gdtf)?;
        physical_descriptions_test::test_adsi_dataton(gdtf)?;
        protocols_test::test_adsi_dataton(gdtf)?;
        revisions_test::test_adsi_dataton(gdtf)?;
        wheels_test::test_adsi_dataton(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_china_36x10(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_china_36x10(gdtf)?;
        dmx_modes_test::test_china_36x10(gdtf)?;
        ft_presets_test::test_china_36x10(gdtf)?;
        geometries_test::test_china_36x10(gdtf)?;
        models_test::test_china_36x10(gdtf)?;
        physical_descriptions_test::test_china_36x10(gdtf)?;
        protocols_test::test_china_36x10(gdtf)?;
        revisions_test::test_china_36x10(gdtf)?;
        wheels_test::test_china_36x10(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_jb_lighting_p12(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_jb_lighting_p12(gdtf)?;
        dmx_modes_test::test_jb_lighting_p12(gdtf)?;
        ft_presets_test::test_jb_lighting_p12(gdtf)?;
        geometries_test::test_jb_lighting_p12(gdtf)?;
        models_test::test_jb_lighting_p12(gdtf)?;
        physical_descriptions_test::test_jb_lighting_p12(gdtf)?;
        protocols_test::test_jb_lighting_p12(gdtf)?;
        revisions_test::test_jb_lighting_p12(gdtf)?;
        wheels_test::test_jb_lighting_p12(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_robe_robin_viva_cmy(gdtf)?;
        dmx_modes_test::test_robe_robin_viva_cmy(gdtf)?;
        ft_presets_test::test_robe_robin_viva_cmy(gdtf)?;
        geometries_test::test_robe_robin_viva_cmy(gdtf)?;
        models_test::test_robe_robin_viva_cmy(gdtf)?;
        physical_descriptions_test::test_robe_robin_viva_cmy(gdtf)?;
        protocols_test::test_robe_robin_viva_cmy(gdtf)?;
        revisions_test::test_robe_robin_viva_cmy(gdtf)?;
        wheels_test::test_robe_robin_viva_cmy(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_sgm_g7_spot(gdtf)?;
        dmx_modes_test::test_sgm_g7_spot(gdtf)?;
        ft_presets_test::test_sgm_g7_spot(gdtf)?;
        geometries_test::test_sgm_g7_spot(gdtf)?;
        models_test::test_sgm_g7_spot(gdtf)?;
        physical_descriptions_test::test_sgm_g7_spot(gdtf)?;
        protocols_test::test_sgm_g7_spot(gdtf)?;
        revisions_test::test_sgm_g7_spot(gdtf)?;
        wheels_test::test_sgm_g7_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_p6(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_sgm_p6(gdtf)?;
        dmx_modes_test::test_sgm_p6(gdtf)?;
        ft_presets_test::test_sgm_p6(gdtf)?;
        geometries_test::test_sgm_p6(gdtf)?;
        models_test::test_sgm_p6(gdtf)?;
        physical_descriptions_test::test_sgm_p6(gdtf)?;
        protocols_test::test_sgm_p6(gdtf)?;
        revisions_test::test_sgm_p6(gdtf)?;
        wheels_test::test_sgm_p6(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_shenzhen_mini_led_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_shenzhen_mini_led_spot(gdtf)?;
        dmx_modes_test::test_shenzhen_mini_led_spot(gdtf)?;
        ft_presets_test::test_shenzhen_mini_led_spot(gdtf)?;
        geometries_test::test_shenzhen_mini_led_spot(gdtf)?;
        models_test::test_shenzhen_mini_led_spot(gdtf)?;
        physical_descriptions_test::test_shenzhen_mini_led_spot(gdtf)?;
        protocols_test::test_shenzhen_mini_led_spot(gdtf)?;
        revisions_test::test_shenzhen_mini_led_spot(gdtf)?;
        wheels_test::test_shenzhen_mini_led_spot(gdtf)?;
        return Ok(());
    }

    pub(crate) fn test_stairville_fan_200(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        attribute_definitions_test::test_stairville_fan_200(gdtf)?;
        dmx_modes_test::test_stairville_fan_200(gdtf)?;
        ft_presets_test::test_stairville_fan_200(gdtf)?;
        geometries_test::test_stairville_fan_200(gdtf)?;
        models_test::test_stairville_fan_200(gdtf)?;
        physical_descriptions_test::test_stairville_fan_200(gdtf)?;
        protocols_test::test_stairville_fan_200(gdtf)?;
        revisions_test::test_stairville_fan_200(gdtf)?;
        wheels_test::test_stairville_fan_200(gdtf)?;
        return Ok(());
    }
}