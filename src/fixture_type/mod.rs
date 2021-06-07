//! Holds the GDTF FixtureType and it's children
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

use crate::fixture_type::attribute_definitions::AttributeDefinitions;
use crate::fixture_type::dmx_mode::DmxMode;
use crate::utils::deparse;
use crate::utils::deparse::{DeparseHashMap, DeparseSingle};
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::errors::GdtfError::QuickXmlError;
use crate::utils::units::guid::Guid;
use crate::utils::units::name::Name;

pub mod attribute_definitions;
pub mod dmx_mode;

///The FixtureType node_2 is the starting point of the description of the fixture type
#[derive(Debug, PartialEq, Clone)]
pub struct FixtureType {
    ///Name of the fixture type.
    pub name: Name,
    /// Shortened name of the fixture type.
    pub short_name: String,
    ///Detailed name of the fixture type.
    pub long_name: String,
    ///Manufacturer of the fixture type.
    pub manufacturer: String,
    /// Description of the fixture type.
    pub description: String,
    ///Unique number of the fixture type.
    pub fixture_type_id: Guid,
    /// File name without extension containing description of the thumbnail.Use the following as a resource file:
    pub thumbnail: Option<String>,
    ///GUID of the referenced fixture type
    pub ref_ft: Option<Guid>,
    ///This section defines all attributes that are used in the fixture type.
    pub attribute_definitions: AttributeDefinitions,
    //Defines the physical or virtual color wheels, gobo wheels, media server content and others.
    // pub wheels: Option<Wheels>,
    //Contains additional physical descriptions.
    // pub physical_descriptions: Option<PhysicalDescriptions>,
    //Contains models of physically separated parts of the device.
    // pub models: Option<Models>,
    //Describes physically separated parts of the device.
    //pub geometries: Geometries,
    ///Contains descriptions of the DMX modes.
    pub dmx_modes: HashMap<Name, DmxMode>,
    //Describe the history of the fixture type.
    //pub revisions: Option<Revisions>,
    //Is used to transfer user - defined and fixture type specific presets to other show files.
    //pub ft_presets: Option<FTPresets>,
    //Specifies supported protocols.
    //pub protocols: Option<Protocols>,
}

impl DeparseSingle for FixtureType {
    type PrimaryKey = ();

    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name = Name::default();
        let mut short_name = String::new();
        let mut long_name = String::new();
        let mut manufacturer = String::new();
        let mut description = String::new();
        let mut fixture_type_id = Guid::dummy();
        let mut thumbnail = None;
        let mut ref_ft: Option<Guid> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = Name::new_from_attr(attr)?,
                b"ShortName" => short_name = deparse::attr_to_string(&attr),
                b"LongName" => long_name = deparse::attr_to_string(&attr),
                b"Manufacturer" => manufacturer = deparse::attr_to_string(&attr),
                b"Description" => description = deparse::attr_to_string(&attr),
                b"FixtureTypeID" => fixture_type_id = Guid::new_from_attr(attr)?,
                b"Thumbnail" => thumbnail = deparse::attr_to_string_option(&attr),
                b"RefFT" => ref_ft = match Guid::new_from_attr(attr) {
                    Ok(guid) => Some(guid),
                    Err(_) => None
                },

                _ => {}
            }
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut attribute_definitions: Option<AttributeDefinitions> = None;
        let mut dmx_modes: Option<HashMap<Name, DmxMode>> = None;
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    match e.name() {
                        b"AttributeDefinitions" => attribute_definitions = Some(AttributeDefinitions::single_from_event(reader, e)?.0),
                        b"DMXModes" => {
                            dmx_modes = Some(DmxMode::hash_map_from_event(reader, e)?);
                        }
                        _ => tree_down += 1
                    }
                }
                Ok(Event::End(_)) => {
                    tree_down -= 1;
                    if tree_down <= 0 { break; }
                }
                Ok(Event::Eof) => {
                    break;
                }
                Err(e) => return Err(QuickXmlError(e)),
                _ => {}
            }
        }
        buf.clear();

        if attribute_definitions.is_none() {
            return Err(GdtfError::RequiredValueNotFoundError("AttributeDefinitions not found in FixtureType".to_string()));
        }
        let attribute_definitions = attribute_definitions.unwrap();
        if dmx_modes.is_none() {
            return Err(GdtfError::RequiredValueNotFoundError("Dmx Modes not found".to_string()));
        }
        let dmx_modes = dmx_modes.unwrap();

        Ok((Self {
            name,
            short_name,
            long_name,
            manufacturer,
            description,
            fixture_type_id,
            thumbnail,
            ref_ft,
            dmx_modes,
            attribute_definitions,
        }, None))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"FixtureType"
    }

    fn single_event_name() -> String {
        "FixtureType".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for FixtureType {}

#[cfg(test)]
mod tests {
    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::fixture_type::attribute_definitions::AttributeDefinitions;
    use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
    use crate::fixture_type::dmx_mode::DmxMode;
    use crate::fixture_type::FixtureType;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::testdata;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::guid::Guid;
    use crate::utils::units::name::Name;
    use crate::utils::units::node::node_attribute_feature::NodeAttributeFeature;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_fixture_type() -> Result<(), GdtfError> {
        FixtureType {
            description: "ACME AE-610 BEAM".to_string(),
            fixture_type_id: Guid::new_from_str("E62F2ECF-2A08-491D-BEEC-F5C491B89784")?,
            long_name: "ACME AE 610 BEAM".to_string(),
            manufacturer: "ACME".to_string(),
            name: Name::new("ACME AE-610 BEAM")?,
            ref_ft: Some(Guid::new_from_str("8F54E11C-4C91-11E9-80BC-F1DFE217E634")?),
            short_name: "ACME AE 610 BEAM".to_string(),
            thumbnail: Some("AE-610 BEAM".to_string()),
            attribute_definitions: AttributeDefinitions {
                activation_groups: vec![Name::new("PanTilt")?],
                feature_groups: testdata::vec_to_hash_map(vec![Name::new("Position")?], vec![FeatureGroup {
                    pretty: "PositionP".to_string(),
                    features: vec![Name::new("PanTilt")?],
                }]),
                attributes: testdata::vec_to_hash_map(vec![AttributeName::Pan], vec![
                    Attribute::new(
                        "P",
                        Some("PanTilt"),
                        NodeAttributeFeature::new_from_strs(vec!["Position", "PanTilt"])?,
                        None,
                        PhysicalUnit::Angle,
                        None)
                ]),
            },
            dmx_modes: testdata::vec_to_hash_map(vec![Name::new("Mode 1 12 DMX")?], vec![DmxMode {
                geometry: Name::new("Base")?,
                dmx_channels: vec![],
            }]),

        }.test(None,
               r#"
        <FixtureType Description="ACME AE-610 BEAM" FixtureTypeID="E62F2ECF-2A08-491D-BEEC-F5C491B89784" LongName="ACME AE 610 BEAM" Manufacturer="ACME" Name="ACME AE-610 BEAM" RefFT="8F54E11C-4C91-11E9-80BC-F1DFE217E634" ShortName="ACME AE 610 BEAM" Thumbnail="AE-610 BEAM">
            <AttributeDefinitions>
                    <ActivationGroups>
                        <ActivationGroup Name="PanTilt"/>
                    </ActivationGroups>
                <FeatureGroups>
                    <FeatureGroup Name="Position" Pretty="PositionP">
                        <Feature Name="PanTilt"/>
                    </FeatureGroup>
                </FeatureGroups>
                <Attributes>
                    <Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>
                </Attributes>
            </AttributeDefinitions>
            <DMXModes>
                <DMXMode Geometry="Base" Name="Mode 1 12 DMX">
                    <DMXChannels>

                    </DMXChannels>
                </DMXMode>
            </DMXModes>
        </FixtureType>
    "#);
        Ok(())
    }
}