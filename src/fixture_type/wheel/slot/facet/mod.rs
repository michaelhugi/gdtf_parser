//! Contains information about PrismFacet for a wheel slot

use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::wheel::slot::Slot;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::color_cie::{ColorCie, COLOR_CIE_WHITE};
use crate::utils::units::rotation::Rotation;

/// Contains information about PrismFacet for a wheel slot
#[derive(Debug, PartialEq, Clone)]
pub struct Facet {
    ///Color of prism facet, Default value: {0.3127, 0.3290, 100.0} (white)
    pub color: ColorCie,
    ///Specify the rotation, translation and scaling for the facet.
    pub rotation: Rotation,
}

///Helper struct to hold temporary data during deparsing
#[derive(Default)]
pub(crate) struct FacetDataHolder {
    ///Color of prism facet, Default value: {0.3127, 0.3290, 100.0} (white)
    pub color: Option<ColorCie>,
    ///Specify the rotation, translation and scaling for the facet.
    pub rotation: Option<Rotation>,
}

///White
const DEFAULT_COLOR: ColorCie = COLOR_CIE_WHITE;

impl ReadGdtf for Facet {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = FacetDataHolder;
    const NODE_NAME: &'static [u8] = b"Facet";
    const PARENT_NODE_NAME: &'static [u8] = Slot::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Color" => data_holder.color = Some(ColorCie::new_from_attr(attr)?),
            b"Rotation" => data_holder.rotation = Some(Rotation::new_from_attr(attr)?),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        _: &mut Self::DataHolder,
        _: &mut Reader<&[u8]>,
        _: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            color: data_holder.color.unwrap_or(DEFAULT_COLOR),
            rotation: data_holder
                .rotation
                .ok_or_else(|| Self::attribute_not_found(b"Rotation"))?,
        })
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for Facet {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    color: ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 100.000000,
                    },
                    rotation: Rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 1.0]]),
                }),
            ),
            (
                None,
                Some(Self {
                    color: ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 100.000000,
                    },
                    rotation: Rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 1.0]]),
                }),
            ),
            (
                None,
                Some(Self {
                    color: ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 78.000001,
                    },
                    rotation: Rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 1.0]]),
                }),
            ),
            (
                None,
                Some(Self {
                    color: ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 100.000000,
                    },
                    rotation: Rotation([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [2.0, 0.0, 1.0]]),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Facet Color="0.312700,0.329000,100.000000" Rotation="{1.000000,0.000000,0.000000}{0.000000,1.000000,0.000000}{2.000000,0.000000,1.000000}"/>"#.to_string(),
            r#"<Facet Color="0.312700,0.329000,100.000000" Rotation="{1.000000,0.000000,0.000000}{0.000000,1.000000,0.000000}{2.000000,0.000000,1.000000}"></Facet>"#.to_string(),
            r#"<Facet Color="0.312700,0.329000,78.000001" Rotation="{1.000000,0.000000,0.000000}{0.000000,1.000000,0.000000}{2.000000,0.000000,1.000000}"/>"#.to_string(),
            r#"<Facet Rotation="{1.000000,0.000000,0.000000}{0.000000,1.000000,0.000000}{2.000000,0.000000,1.000000}"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<Facet Color="0.312700,0.329000,100.000000" Rotation="{0.000000,0.000000}{0.000000,1.000000,0.000000}{2.000000,0.000000,1.000000}"/>"#.to_string(),
            r#"<Facet Color="0.312700,0.329000,78.000001"/>"#.to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::wheel::slot::facet::{Facet, DEFAULT_COLOR};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::units::color_cie::ColorCie;

    #[test]
    fn test_deparse() {
        Facet::execute_tests();
    }

    #[test]
    fn test_default_color() {
        assert_eq!(
            DEFAULT_COLOR,
            ColorCie {
                x: 0.3127,
                y: 0.3290,
                Y: 100.0
            }
        );
    }
}
