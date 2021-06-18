//! Represents a slot on a wheel
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::wheel::slot::animation_system::AnimationSystem;
use crate::fixture_type::wheel::slot::facet::Facet;
use crate::fixture_type::wheel::Wheel;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::color_cie::{COLOR_CIE_WHITE, ColorCie};
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;
use crate::utils::units::resource::Resource;

pub mod facet;
pub mod animation_system;

/// Represents a slot on a wheel
#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    /// Color of the wheel slot, Default value: {0.3127, 0.3290, 100.0} (white) For Y give relative value compared to overall output defined in property Luminous Flux of related Beam Geometry (transmissive case)
    pub color: ColorCie,
    /// Optional link to filter in the physical description; Do not define color if filter is used; Starting point: Filter Collect
    pub filter: Option<Node>,
    /// Optional; PNG file name without extension containing image for specific gobos etc.
    /// — Maximum resolution of picture: 1 024x1 024;
    /// — Recommended resolution of gobo: 256x256;
    /// — Recommended resolution of animation wheel: 256x256
    /// These resource files are located in a folder called ./wheels in the zip archive. Default value: empty.
    pub media_file_name: Option<Resource>,
    /// Used if the wheel slot has a prism
    pub prism_facets: Option<Vec<Facet>>,
    /// Used if the wheel slot has an AnimationWheel
    pub animation_wheel: Option<AnimationSystem>,
}

/// Helper struct to hold temporary data during deparsing
#[derive(Default)]
pub struct SlotDataHolder {
    /// Color of the wheel slot, Default value: {0.3127, 0.3290, 100.0} (white) For Y give relative value compared to overall output defined in property Luminous Flux of related Beam Geometry (transmissive case)
    pub color: Option<ColorCie>,
    /// Optional link to filter in the physical description; Do not define color if filter is used; Starting point: Filter Collect
    pub filter: Option<Node>,
    /// Optional; PNG file name without extension containing image for specific gobos etc.
    /// — Maximum resolution of picture: 1 024x1 024;
    /// — Recommended resolution of gobo: 256x256;
    /// — Recommended resolution of animation wheel: 256x256
    /// These resource files are located in a folder called ./wheels in the zip archive. Default value: empty.
    pub media_file_name: Option<Resource>,
    /// Used if the wheel slot has a prism
    pub prism_facets: Vec<Facet>,
    /// Used if the wheel slot has an AnimationWheel
    pub animation_wheel: Option<AnimationSystem>,
}

///White
const DEFAULT_COLOR: ColorCie = COLOR_CIE_WHITE;

impl ReadGdtf for Slot {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = SlotDataHolder;
    const NODE_NAME: &'static [u8] = b"Slot";
    const PARENT_NODE_NAME: &'static [u8] = Wheel::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"Color" => data_holder.color = Some(ColorCie::new_from_attr(attr)?),
            b"Filter" => data_holder.filter = Node::new_from_attr(attr)?,
            b"MediaFileName" => data_holder.media_file_name = Some(Resource::new_from_attr(attr)),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(data_holder: &mut Self::DataHolder, reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(), Self::Error> {
        match event.name() {
            Facet::NODE_NAME => data_holder.prism_facets.push(Facet::read_single_from_event(reader, event, has_children)?.1),
            AnimationSystem::NODE_NAME => data_holder.animation_wheel = Some(AnimationSystem::read_single_from_event(reader, event, has_children)?.1),
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        let media_file_name = if let Some(media_file_name) = data_holder.media_file_name {
            if media_file_name.0 == "empty" { None } else { Some(media_file_name) }
        } else {
            None
        };
        let prism_facets = if data_holder.prism_facets.is_empty() { None } else { Some(data_holder.prism_facets) };
        Ok(Self {
            color: data_holder.color.unwrap_or(DEFAULT_COLOR),
            filter: data_holder.filter,
            media_file_name,
            prism_facets,
            animation_wheel: data_holder.animation_wheel,
        })
    }

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for Slot {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("Open").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.329, Y: 100.0 }, filter: None, media_file_name: None, animation_wheel: None, prism_facets: None })),
            (Some(Name::new("Closed").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.329, Y: 100.0 }, filter: None, media_file_name: None, animation_wheel: None, prism_facets: None })),
            (Some(Name::new("Something").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.234001, Y: 99.000001 }, filter: None, media_file_name: None, animation_wheel: None, prism_facets: None })),
            (Some(Name::new("Open").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.234001, Y: 99.000001 }, filter: None, media_file_name: None, animation_wheel: None, prism_facets: Some(Facet::testdata_vec()) })),
            (Some(Name::new("Open").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.234001, Y: 99.000001 }, filter: Some(Node(vec![Name::new("MyFilter").unwrap(), Name::new("MyFilter2").unwrap()])), media_file_name: None, animation_wheel: AnimationSystem::testdatas().get(0).unwrap().1.clone(), prism_facets: None })),
            (Some(Name::new("Open").unwrap()), Some(Self { color: ColorCie { x: 0.3127, y: 0.234001, Y: 99.000001 }, filter: None, media_file_name: Some(Resource("media.jpg".to_string())), animation_wheel: AnimationSystem::testdatas().get(1).unwrap().1.clone(), prism_facets: Some(Facet::testdata_vec()) }))
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Slot Color="0.312700,0.329000,100.000000" Name="Open"/>"#.to_string(),
            r#"<Slot Name="Closed"/>"#.to_string(),
            r#"<Slot Color="0.312700,0.234001,99.000001" Name="Something"></Slot>"#.to_string(),
            format!(r#"<Slot Color="0.312700,0.234001,99.000001" Name="Open">{}</Slot>"#, Facet::testdata_xml()),
            format!(r#"<Slot Color="0.312700,0.234001,99.000001" Name="Open" Filter="MyFilter.MyFilter2" MediaFileName="empty">{}</Slot>"#, AnimationSystem::testdatas_xml().get(0).unwrap()),
            format!(r#"<Slot Color="0.312700,0.234001,99.000001" Name="Open" MediaFileName="media.jpg">{}{}</Slot>"#, Facet::testdata_xml(), AnimationSystem::testdatas_xml().get(1).unwrap()),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            //r#"<Slot Color="0.312700,0.234001,99.000001"></Slot>"#.to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::wheel::slot::{DEFAULT_COLOR, Slot};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::units::color_cie::ColorCie;

    #[test]
    fn test_deparse() {
        Slot::execute_tests();
    }

    #[test]
    fn test_default_color() {
        assert_eq!(DEFAULT_COLOR, ColorCie { x: 0.3127, y: 0.3290, Y: 100.0 });
    }
}