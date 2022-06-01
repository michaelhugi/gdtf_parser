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
pub(crate) mod fixture_type_test {
    use crate::gdtf_v_1::fixture_type::attribute_definitions::attribute_definitions_test;
    use crate::gdtf_v_1::GdtfV1;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::can_have_children::CanHaveChildren;
    use crate::utils::units::guid::Guid;
    use crate::utils::units::guid_opt::GuidOpt;
    use crate::utils::units::name::Name;
    use crate::utils::units::resource::Resource;

    pub(crate) fn test_acme_ae_610_beam(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        let ft = &gdtf.fixture_type;
        assert_eq!(ft.name, Name::new("ACME AE-610 BEAM").unwrap());
        assert_eq!(ft.short_name, "ACME AE 610 BEAM".to_string());
        assert_eq!(ft.long_name, "ACME AE 610 BEAM".to_string());
        assert_eq!(ft.manufacturer, "ACME".to_string());
        assert_eq!(ft.description, "ACME AE-610 BEAM".to_string());
        assert_eq!(ft.fixture_type_id, Guid::new_from_str("E62F2ECF-2A08-491D-BEEC-F5C491B89784").unwrap());
        assert_eq!(ft.thumbnail, Resource::new_from_str("AE-610 BEAM"));
        assert_eq!(ft.ref_ft, GuidOpt(Some(Guid::new_from_str("8F54E11C-4C91-11E9-80BC-F1DFE217E634").unwrap())));
        assert_eq!(ft.can_have_children, CanHaveChildren::Yes);
        attribute_definitions_test::test_acme_ae_610_beam(&gdtf.fixture_type.attribute_definitions)?;
        return Ok(());
    }

    pub(crate) fn test_jb_12_spot_hp(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        let ft = &gdtf.fixture_type;
        assert_eq!(ft.name, Name::new("P12 Spot HP").unwrap());
        assert_eq!(ft.short_name, "P12SPHP".to_string());
        assert_eq!(ft.long_name, "P12 Spot HP".to_string());
        assert_eq!(ft.manufacturer, "JB-Lighting".to_string());
        assert_eq!(ft.description, "P12 Spot HP (High Power) 640W".to_string());
        assert_eq!(ft.fixture_type_id, Guid::new_from_str("807DC00C-18D5-4133-B781-1A003FA988FA").unwrap());
        assert_eq!(ft.thumbnail, Resource::new_from_str("P12 dunkel"));
        assert_eq!(ft.ref_ft, GuidOpt(None));
        assert_eq!(ft.can_have_children, CanHaveChildren::Yes);
        attribute_definitions_test::test_jb_12_spot_hp(&gdtf.fixture_type.attribute_definitions)?;
        return Ok(());
    }


    pub(crate) fn test_robe_robin_viva_cmy(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        let ft = &gdtf.fixture_type;
        assert_eq!(ft.name, Name::new("Robin Viva CMY").unwrap());
        assert_eq!(ft.short_name, "Viva™ CMY".to_string());
        assert_eq!(ft.long_name, "Robin Viva™ CMY".to_string());
        assert_eq!(ft.manufacturer, "Robe Lighting".to_string());
        assert_eq!(ft.description, "Powerfully smooth, Robe’s VIVA CMY combines brightness of exceptionally clear zero-fringing white beam together with continuous color transitions of CMY mixing. ".to_string());
        assert_eq!(ft.fixture_type_id, Guid::new_from_str("BEB8B97D-FF49-4FBE-A834-9BE2C7BC689B").unwrap());
        assert_eq!(ft.thumbnail, Resource::new_from_str("thumbnail"));
        assert_eq!(ft.ref_ft, GuidOpt(None));
        assert_eq!(ft.can_have_children, CanHaveChildren::Yes);
        attribute_definitions_test::test_robe_robin_viva_cmy(&gdtf.fixture_type.attribute_definitions)?;
        return Ok(());
    }

    pub(crate) fn test_sgm_g7_spot(gdtf: &GdtfV1) -> Result<(), GdtfError> {
        let ft = &gdtf.fixture_type;
        assert_eq!(ft.name, Name::new("G-7 Spot").unwrap());
        assert_eq!(ft.short_name, "G-7 Spot".to_string());
        assert_eq!(ft.long_name, "G-7 Spot".to_string());
        assert_eq!(ft.manufacturer, "SGM Light".to_string());
        assert_eq!(ft.description, "The G-7 Spot is the quintessence of IP-rated moving heads. A fast, compact, and lightweight mid-sized moving head spot with high-output and low power consumption. Thanks to its white LED engine and CMY color mixing, the G-7 Spot is the perfect moving head for those who need maximum light output inside an easy-to-move luminaire. The G-7 Spot gives you a solid construction, a high-quality beam, and an optimal projection in a very flexible assembly. A fixture born to rock night after night.".to_string());
        assert_eq!(ft.fixture_type_id, Guid::new_from_str("14030EC0-9085-4756-8B19-8B08369E06B9").unwrap());
        assert_eq!(ft.thumbnail, Resource::new_from_str("G-7_RAL_black_small"));
        assert_eq!(ft.ref_ft, GuidOpt(None));
        assert_eq!(ft.can_have_children, CanHaveChildren::Yes);
        attribute_definitions_test::test_sgm_g7_spot(&gdtf.fixture_type.attribute_definitions)?;
        return Ok(());
    }
}