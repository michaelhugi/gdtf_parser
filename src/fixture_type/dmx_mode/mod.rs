//!This section is describes all DMX modes of the device
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::dmx_mode::dmx_channel::DmxChannel;
use crate::fixture_type::dmx_mode::ft_macro::FtMacro;
use crate::fixture_type::dmx_mode::relation::Relation;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

pub mod dmx_channel;

pub mod ft_macro;
pub mod relation;

///Each DMX mode describes logical control a part of the device in a specific mode
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct DmxMode {
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    pub geometry: Name,
    ///Description of all DMX channels used in the mode
    pub dmx_channels: Vec<DmxChannel>,
    ///Description of relations between channels
    pub relations: HashMap<Name, Relation>,
    ///Macros defined by the manufacturer
    pub ft_macros: HashMap<Name, FtMacro>,
}

///Helper struct for temporary data during deparsing
#[derive(Default)]
pub(crate) struct DmxModeDataHolder {
    ///Name of the first geometry in the device; Only top level geometries are allowed to be linked.
    pub geometry: Option<Name>,
    ///Description of all DMX channels used in the mode
    pub dmx_channels: Option<Vec<DmxChannel>>,
    ///Description of relations between channels
    pub relations: HashMap<Name, Relation>,
    ///Macros defined by the manufacturer
    pub ft_macros: HashMap<Name, FtMacro>,
}

impl ReadGdtf for DmxMode {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = DmxModeDataHolder;

    const NODE_NAME: &'static [u8] = b"DMXMode";
    const PARENT_NODE_NAME: &'static [u8] = b"DMXModes";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        if attr.key == b"Geometry" {
            data_holder.geometry = Some(Name::new_from_attr(attr)?);
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
            DmxChannel::PARENT_NODE_NAME => {
                data_holder.dmx_channels = Some(DmxChannel::read_vec_from_event(
                    reader,
                    event,
                    has_children,
                )?)
            }
            Relation::PARENT_NODE_NAME => {
                data_holder.relations =
                    Relation::read_hash_map_from_event(reader, event, has_children)?
            }
            FtMacro::PARENT_NODE_NAME => {
                data_holder.ft_macros =
                    FtMacro::read_hash_map_from_event(reader, event, has_children)?
            }
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            geometry: data_holder
                .geometry
                .ok_or_else(|| Self::attribute_not_found(b"Geometry"))?,
            dmx_channels: data_holder
                .dmx_channels
                .ok_or_else(|| Self::child_not_found(b"DMXChannels"))?,
            relations: data_holder.relations,
            ft_macros: data_holder.ft_macros,
        })
    }

    fn read_primary_key_from_attr(
        attr: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for DmxMode {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                Some(Name::new("Mode1").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry1").unwrap(),
                    dmx_channels: vec![],
                    relations: HashMap::new(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode2").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry1").unwrap(),
                    dmx_channels: vec![],
                    relations: HashMap::new(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode3").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry1").unwrap(),
                    dmx_channels: DmxChannel::testdata_vec(),
                    relations: HashMap::new(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode4").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry2").unwrap(),
                    dmx_channels: DmxChannel::testdata_vec(),
                    relations: HashMap::new(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode5").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry3").unwrap(),
                    dmx_channels: DmxChannel::testdata_vec(),
                    relations: HashMap::new(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode6").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry4").unwrap(),
                    dmx_channels: vec![],
                    relations: Relation::testdata_hash_map(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode7").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry5").unwrap(),
                    dmx_channels: vec![],
                    relations: Relation::testdata_hash_map(),
                    ft_macros: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("Mode8").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry6").unwrap(),
                    dmx_channels: vec![],
                    relations: HashMap::new(),
                    ft_macros: FtMacro::testdata_hash_map(),
                }),
            ),
            (
                Some(Name::new("Mode9").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry7").unwrap(),
                    dmx_channels: vec![],
                    relations: HashMap::new(),
                    ft_macros: FtMacro::testdata_hash_map(),
                }),
            ),
            (
                Some(Name::new("Mode10").unwrap()),
                Some(Self {
                    geometry: Name::new("Geometry8").unwrap(),
                    dmx_channels: DmxChannel::testdata_vec(),
                    relations: Relation::testdata_hash_map(),
                    ft_macros: FtMacro::testdata_hash_map(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            format!(r#"<DMXMode Name="Mode1" Geometry="Geometry1"><DMXChannels/></DMXMode>"#),
            format!(
                r#"<DMXMode Name="Mode2" Geometry="Geometry1"><DMXChannels></DMXChannels></DMXMode>"#
            ),
            format!(
                r#"<DMXMode Name="Mode3" Geometry="Geometry1"><DMXChannels>{}</DMXChannels></DMXMode>"#,
                DmxChannel::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode4" Geometry="Geometry2"><DMXChannels>{}</DMXChannels><Relations></Relations><FTMacros></FTMacros></DMXMode>"#,
                DmxChannel::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode5" Geometry="Geometry3"><DMXChannels>{}</DMXChannels><Relations/><FTMacros/></DMXMode>"#,
                DmxChannel::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode6" Geometry="Geometry4"><DMXChannels></DMXChannels><Relations>{}</Relations><FTMacros></FTMacros></DMXMode>"#,
                Relation::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode7" Geometry="Geometry5"><DMXChannels/><Relations>{}</Relations><FTMacros/></DMXMode>"#,
                Relation::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode8" Geometry="Geometry6"><DMXChannels></DMXChannels><Relations></Relations><FTMacros>{}</FTMacros></DMXMode>"#,
                FtMacro::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode9" Geometry="Geometry7"><DMXChannels/><Relations/><FTMacros>{}</FTMacros></DMXMode>"#,
                FtMacro::testdata_xml()
            ),
            format!(
                r#"<DMXMode Name="Mode10" Geometry="Geometry8"><DMXChannels>{}</DMXChannels><Relations>{}</Relations><FTMacros>{}</FTMacros></DMXMode>"#,
                DmxChannel::testdata_xml(),
                Relation::testdata_xml(),
                FtMacro::testdata_xml()
            ),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<DMXMode Name="Mode1" Geometry="Geometry1"></DMXMode>"#.to_string(),
            r#"<DMXMode Name="Mode1" Geometry="Geometry1"/>"#.to_string(),
            format!(
                r#"<DMXMode Name="Name1"><DMXChannels>{}</DMXChannels></DMXMode>"#,
                DmxChannel::testdata_xml()
            ),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::DmxMode;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        DmxMode::execute_tests();
    }
}
