//!Describes the physical constitution of the device
use std::collections::HashMap;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::physical_descriptions::color_space::ColorSpace;
use crate::fixture_type::physical_descriptions::connectors::Connector;
use crate::fixture_type::physical_descriptions::cris::CriGroup;
use crate::fixture_type::physical_descriptions::dmx_profiles::DmxProfile;
use crate::fixture_type::physical_descriptions::emitters::Emitter;
use crate::fixture_type::physical_descriptions::filters::Filter;
use crate::fixture_type::physical_descriptions::properties::Properties;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

pub mod color_space;
pub mod connectors;
pub mod cris;
pub mod dmx_profiles;
pub mod emitters;
pub mod filters;
pub mod measurement;
pub mod properties;

///Describes the physical constitution of the device
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct PhysicalDescriptions {
    ///Describes device emitters
    pub emitters: HashMap<Name, Emitter>,
    ///Describes device filters
    pub filters: HashMap<Name, Filter>,
    ///Describes device color space
    pub color_space: Option<ColorSpace>,
    ///Describes nonlinear correlation between DMX input and physical output of a channel.
    pub dmx_profiles: Vec<DmxProfile>,
    ///Describes color rendering according to ANSI/IES TM-30 (99 color samples).
    pub cris: Vec<CriGroup>,
    ///Describes physical connectors of the device.
    pub connectors: HashMap<Name, Connector>,
    ///Describes physical properties of the device.
    pub properties: Option<Properties>,
}

impl ReadGdtf for PhysicalDescriptions {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = PhysicalDescriptions;
    const NODE_NAME: &'static [u8] = b"PhysicalDescriptions";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(_: &mut Self::DataHolder, _: Attribute<'_>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        match event.name() {
            Emitter::PARENT_NODE_NAME => {
                data_holder.emitters =
                    Emitter::read_hash_map_from_event(reader, event, has_children)?
            }
            Filter::PARENT_NODE_NAME => {
                data_holder.filters = Filter::read_hash_map_from_event(reader, event, has_children)?
            }
            ColorSpace::NODE_NAME => {
                data_holder.color_space =
                    Some(ColorSpace::read_single_from_event(reader, event, has_children)?.1)
            }
            DmxProfile::PARENT_NODE_NAME => {
                data_holder.dmx_profiles =
                    DmxProfile::read_vec_from_event(reader, event, has_children)?
            }
            CriGroup::PARENT_NODE_NAME => {
                data_holder.cris = CriGroup::read_vec_from_event(reader, event, has_children)?
            }
            Connector::PARENT_NODE_NAME => {
                data_holder.connectors =
                    Connector::read_hash_map_from_event(reader, event, has_children)?
            }
            Properties::NODE_NAME => {
                data_holder.properties =
                    Some(Properties::read_single_from_event(reader, event, has_children)?.1)
            }
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for PhysicalDescriptions {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![(
            None,
            Some(PhysicalDescriptions {
                emitters: Emitter::testdata_hash_map(),
                filters: Filter::testdata_hash_map(),
                color_space: Some(ColorSpace::testdata_vec()[0].clone()),
                dmx_profiles: DmxProfile::testdata_vec(),
                cris: CriGroup::testdata_vec(),
                connectors: Connector::testdata_hash_map(),
                properties: Some(Properties::testdata_vec()[0].clone()),
            }),
        )]
    }

    fn testdatas_xml() -> Vec<String> {
        let xml = format!(
            r#"<PhysicalDescriptions>
        <Emitters>
            {}
        </Emitters>
        <Filters>
            {}
        </Filters>
        {}
        <DMXProfiles>
            {}
        </DMXProfiles>
        <CRIs>
            {}
        </CRIs>
        <Connectors>
            {}
        </Connectors>
            {}
        </PhysicalDescriptions>"#,
            Emitter::testdata_xml(),
            Filter::testdata_xml(),
            ColorSpace::testdatas_xml()[0],
            DmxProfile::testdata_xml(),
            CriGroup::testdata_xml(),
            Connector::testdata_xml(),
            Properties::testdatas_xml()[0]
        );
        vec![xml]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::PhysicalDescriptions;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        PhysicalDescriptions::execute_tests();
    }
}
