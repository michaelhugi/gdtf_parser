//! Holds the GDTF FixtureType and it's children
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

#[cfg(test)]
use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
#[cfg(test)]
use crate::fixture_type::attribute_definitions::attribute::Attribute;
#[cfg(test)]
use crate::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use crate::fixture_type::attribute_definitions::AttributeDefinitions;
use crate::fixture_type::dmx_mode::DmxMode;
use crate::fixture_type::physical_descriptions::PhysicalDescriptions;
use crate::fixture_type::wheel::Wheel;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::guid::Guid;
use crate::utils::units::name::Name;
use crate::utils::units::resource::Resource;
use crate::Gdtf;

pub mod attribute_definitions;
pub mod dmx_mode;
pub mod physical_descriptions;
pub mod wheel;

///The FixtureType node_2 is the starting point of the description of the fixture type
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
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
    pub thumbnail: Option<Resource>,
    ///GUID of the referenced fixture type
    pub ref_ft: Option<Guid>,
    ///Describes if it is possible to mount other devices to this device. Value: “Yes”, “No”. Default value: “Yes”
    pub can_have_children: CanHaveChildren,
    ///This section defines all attributes that are used in the fixture type.
    pub attribute_definitions: AttributeDefinitions,
    ///Defines the physical or virtual color wheels, gobo wheels, media server content and others.
    pub wheels: Option<HashMap<Name, Wheel>>,
    //Contains additional physical descriptions.
    pub physical_descriptions: Option<PhysicalDescriptions>,
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

#[derive(Default)]
pub(crate) struct FixtureTypeDataHolder {
    pub name: Option<Name>,
    pub short_name: Option<String>,
    pub long_name: Option<String>,
    pub manufacturer: Option<String>,
    pub description: Option<String>,
    pub fixture_type_id: Option<Guid>,
    pub thumbnail: Option<Resource>,
    pub ref_ft: Option<Guid>,
    pub can_have_children: Option<CanHaveChildren>,
    pub attribute_definitions: Option<AttributeDefinitions>,
    pub dmx_modes: Option<HashMap<Name, DmxMode>>,
    pub wheels: Option<HashMap<Name, Wheel>>,
    pub physical_descriptions: Option<PhysicalDescriptions>,
}

impl ReadGdtf for FixtureType {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = FixtureTypeDataHolder;
    const NODE_NAME: &'static [u8] = b"FixtureType";
    const PARENT_NODE_NAME: &'static [u8] = Gdtf::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: quick_xml::events::attributes::Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Name" => data_holder.name = Some(Name::new_from_attr(attr)?),
            b"ShortName" => data_holder.short_name = Some(read::attr_to_string(attr)),
            b"LongName" => data_holder.long_name = Some(read::attr_to_string(attr)),
            b"Manufacturer" => data_holder.manufacturer = Some(read::attr_to_string(attr)),
            b"Description" => data_holder.description = Some(read::attr_to_string(attr)),
            b"FixtureTypeID" => data_holder.fixture_type_id = Some(Guid::new_from_attr(attr)?),
            b"Thumbnail" => data_holder.thumbnail = Some(Resource::new_from_attr(attr)),
            b"RefFT" => data_holder.ref_ft = Guid::new_from_attr(attr).ok(),
            b"CanHaveChildren" => {
                data_holder.can_have_children = Some(CanHaveChildren::new_from_attr(attr))
            }
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        match event.name() {
            DmxMode::PARENT_NODE_NAME => {
                data_holder.dmx_modes = Some(DmxMode::read_hash_map_from_event(
                    reader,
                    event,
                    has_children,
                )?)
            }
            AttributeDefinitions::NODE_NAME => {
                data_holder.attribute_definitions = Some(
                    AttributeDefinitions::read_single_from_event(reader, event, has_children)?.1,
                )
            }
            Wheel::PARENT_NODE_NAME => {
                data_holder.wheels = Some(Wheel::read_hash_map_from_event(
                    reader,
                    event,
                    has_children,
                )?)
            }
            PhysicalDescriptions::NODE_NAME => {
                data_holder.physical_descriptions = Some(
                    PhysicalDescriptions::read_single_from_event(reader, event, has_children)?.1,
                )
            }
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            name: data_holder
                .name
                .ok_or_else(|| Self::attribute_not_found(b"Name"))?,
            short_name: data_holder
                .short_name
                .ok_or_else(|| Self::attribute_not_found(b"ShortName"))?,
            long_name: data_holder
                .long_name
                .ok_or_else(|| Self::attribute_not_found(b"LongName"))?,
            manufacturer: data_holder
                .manufacturer
                .ok_or_else(|| Self::attribute_not_found(b"Manufacturer"))?,
            description: data_holder
                .description
                .ok_or_else(|| Self::attribute_not_found(b"Description"))?,
            fixture_type_id: data_holder
                .fixture_type_id
                .ok_or_else(|| Self::attribute_not_found(b"FixtureTypeId"))?,
            thumbnail: data_holder.thumbnail,
            ref_ft: data_holder.ref_ft,
            can_have_children: data_holder.can_have_children.unwrap_or_default(),
            attribute_definitions: data_holder
                .attribute_definitions
                .ok_or_else(|| Self::attribute_not_found(b"AttributeDefinitons"))?,
            dmx_modes: data_holder
                .dmx_modes
                .ok_or_else(|| Self::attribute_not_found(b"DmxModes"))?,
            wheels: data_holder.wheels,
            physical_descriptions: data_holder.physical_descriptions,
        })
    }

    fn read_primary_key_from_attr(
        _: quick_xml::events::attributes::Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed!");
    }
}

#[cfg(test)]
impl TestReadGdtf for FixtureType {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    name: Name::new("P12 Spot HP").unwrap(),
                    short_name: "P12SPHP".to_string(),
                    long_name: "P12 Spot HP".to_string(),
                    manufacturer: "JB-Lighting".to_string(),
                    description: "P12 Spot HP (High Power) 640W".to_string(),
                    fixture_type_id: Guid::new_from_str("807DC00C-18D5-4133-B781-1A003FA988FA")
                        .unwrap(),
                    thumbnail: Some(Resource::new_from_str("P12 dunkel")),
                    ref_ft: Some(
                        Guid::new_from_str("807DC00C-18D5-4133-B781-1A003FA988FB").unwrap(),
                    ),
                    can_have_children: CanHaveChildren::Yes,
                    attribute_definitions: AttributeDefinitions {
                        feature_groups: FeatureGroup::testdata_hash_map(),
                        attributes: Attribute::testdata_hash_map(),
                        activation_groups: ActivationGroup::testdata_primary_key_vec(),
                    },
                    dmx_modes: DmxMode::testdata_hash_map(),
                    wheels: None,
                    physical_descriptions: Some(PhysicalDescriptions::testdata_vec()[0].clone()),
                }),
            ),
            (
                None,
                Some(Self {
                    name: Name::new("P12 Spot HP").unwrap(),
                    short_name: "P12SPHP".to_string(),
                    long_name: "P12 Spot HP".to_string(),
                    manufacturer: "JB-Lighting".to_string(),
                    description: "P12 Spot HP (High Power) 640W".to_string(),
                    fixture_type_id: Guid::new_from_str("807DC00C-18D5-4133-B781-1A003FA988FA")
                        .unwrap(),
                    thumbnail: None,
                    ref_ft: None,
                    can_have_children: CanHaveChildren::No,
                    attribute_definitions: AttributeDefinitions {
                        feature_groups: FeatureGroup::testdata_hash_map(),
                        attributes: Attribute::testdata_hash_map(),
                        activation_groups: ActivationGroup::testdata_primary_key_vec(),
                    },
                    dmx_modes: DmxMode::testdata_hash_map(),
                    wheels: None,
                    physical_descriptions: None,
                }),
            ),
            (
                None,
                Some(Self {
                    name: Name::new("P12 Spot HP").unwrap(),
                    short_name: "P12SPHP".to_string(),
                    long_name: "P12 Spot HP".to_string(),
                    manufacturer: "JB-Lighting".to_string(),
                    description: "P12 Spot HP (High Power) 640W".to_string(),
                    fixture_type_id: Guid::new_from_str("807DC00C-18D5-4133-B781-1A003FA988FA")
                        .unwrap(),
                    thumbnail: None,
                    ref_ft: None,
                    can_have_children: CanHaveChildren::No,
                    attribute_definitions: AttributeDefinitions {
                        feature_groups: FeatureGroup::testdata_hash_map(),
                        attributes: Attribute::testdata_hash_map(),
                        activation_groups: ActivationGroup::testdata_primary_key_vec(),
                    },
                    dmx_modes: DmxMode::testdata_hash_map(),
                    wheels: Some(Wheel::testdata_hash_map()),
                    physical_descriptions: None,
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            format!(
                r#"<FixtureType CanHaveChildren="Yes" Description="P12 Spot HP (High Power) 640W" FixtureTypeID="807DC00C-18D5-4133-B781-1A003FA988FA" LongName="P12 Spot HP" Manufacturer="JB-Lighting" Name="P12 Spot HP" RefFT="807DC00C-18D5-4133-B781-1A003FA988FB" ShortName="P12SPHP" Thumbnail="P12 dunkel"><AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions><DMXModes>{}</DMXModes>{}</FixtureType>"#,
                FeatureGroup::testdata_xml(),
                Attribute::testdata_xml(),
                ActivationGroup::testdata_xml(),
                DmxMode::testdata_xml(),
                PhysicalDescriptions::testdatas_xml()[0]
            ),
            format!(
                r#"<FixtureType CanHaveChildren="No" Description="P12 Spot HP (High Power) 640W" FixtureTypeID="807DC00C-18D5-4133-B781-1A003FA988FA" LongName="P12 Spot HP" Manufacturer="JB-Lighting" Name="P12 Spot HP" RefFT="" ShortName="P12SPHP"><AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions><DMXModes>{}</DMXModes></FixtureType>"#,
                FeatureGroup::testdata_xml(),
                Attribute::testdata_xml(),
                ActivationGroup::testdata_xml(),
                DmxMode::testdata_xml()
            ),
            format!(
                r#"<FixtureType CanHaveChildren="No" Description="P12 Spot HP (High Power) 640W" FixtureTypeID="807DC00C-18D5-4133-B781-1A003FA988FA" LongName="P12 Spot HP" Manufacturer="JB-Lighting" Name="P12 Spot HP" RefFT="" ShortName="P12SPHP"><AttributeDefinitions><FeatureGroups>{}</FeatureGroups><Attributes>{}</Attributes><ActivationGroups>{}</ActivationGroups></AttributeDefinitions><Wheels>{}</Wheels><DMXModes>{}</DMXModes></FixtureType>"#,
                FeatureGroup::testdata_xml(),
                Attribute::testdata_xml(),
                ActivationGroup::testdata_xml(),
                Wheel::testdata_xml(),
                DmxMode::testdata_xml()
            ),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of CanHaveChildren
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///Describes if it is possible to mount other devices to this device. Value: “Yes”, “No”. Default value: “Yes”
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CanHaveChildren {
    Yes,
    No,
}

impl CanHaveChildren {
    ///Parses a string provided by gdtf-xml-description to CanHaveChildren
    /// ```rust
    /// use gdtf_parser::fixture_type::CanHaveChildren;
    /// assert_eq!(CanHaveChildren::No, CanHaveChildren::new_from_str("No"));
    /// assert_eq!(CanHaveChildren::Yes, CanHaveChildren::new_from_str("Yes"));
    /// assert_eq!(CanHaveChildren::Yes, CanHaveChildren::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(value: &str) -> Self {
        match value {
            "No" => Self::No,
            _ => Self::Yes,
        }
    }
    ///Parses a quick-xml-attribute provided by gdtf-xml-description to CanHaveChildren
    /// ```rust
    /// use gdtf_parser::fixture_type::CanHaveChildren;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(CanHaveChildren::No, CanHaveChildren::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"No")}));
    /// assert_eq!(CanHaveChildren::Yes, CanHaveChildren::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Yes")}));
    /// assert_eq!(CanHaveChildren::Yes, CanHaveChildren::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Anything else")}));
    /// ```
    pub fn new_from_attr(attr: quick_xml::events::attributes::Attribute<'_>) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

///```rust
/// use gdtf_parser::fixture_type::CanHaveChildren;
/// assert_eq!(CanHaveChildren::Yes, Default::default());
/// ```
impl Default for CanHaveChildren {
    fn default() -> Self {
        Self::Yes
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of CanHaveChildren
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::fixture_type::{CanHaveChildren, FixtureType};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::testdata;

    #[test]
    pub fn test_deparse() {
        FixtureType::execute_tests();
    }

    #[test]
    pub fn test_can_have_children_new_from_str() {
        assert_eq!(CanHaveChildren::No, CanHaveChildren::new_from_str("No"));
        assert_eq!(CanHaveChildren::Yes, CanHaveChildren::new_from_str("Yes"));
        assert_eq!(
            CanHaveChildren::Yes,
            CanHaveChildren::new_from_str("Anything else")
        );
    }

    #[test]
    pub fn test_can_have_children_new_from_attr_owned() {
        assert_eq!(
            CanHaveChildren::No,
            CanHaveChildren::new_from_attr(testdata::to_attr_owned(b"No"))
        );
        assert_eq!(
            CanHaveChildren::Yes,
            CanHaveChildren::new_from_attr(testdata::to_attr_owned(b"Yes"))
        );
        assert_eq!(
            CanHaveChildren::Yes,
            CanHaveChildren::new_from_attr(testdata::to_attr_owned(b"Anything else"))
        );
    }

    #[test]
    pub fn test_can_have_children_new_from_attr_borrowed() {
        assert_eq!(
            CanHaveChildren::No,
            CanHaveChildren::new_from_attr(testdata::to_attr_borrowed(b"No"))
        );
        assert_eq!(
            CanHaveChildren::Yes,
            CanHaveChildren::new_from_attr(testdata::to_attr_borrowed(b"Yes"))
        );
        assert_eq!(
            CanHaveChildren::Yes,
            CanHaveChildren::new_from_attr(testdata::to_attr_borrowed(b"Anything else"))
        );
    }

    #[test]
    pub fn test_can_have_children_default() {
        assert_eq!(CanHaveChildren::Yes, Default::default());
    }
}
