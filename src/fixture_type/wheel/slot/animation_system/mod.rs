//! Module for defining animation system's behaviour
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::wheel::slot::Slot;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::pixel::Pixel;
use crate::utils::units::pixel_array::PixelArray;

///Defines the animation system disk and it describes the animation system behavior
#[derive(Debug, PartialEq, Clone)]
pub struct AnimationSystem {
    ///First Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p1: PixelArray,
    ///Second Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p2: PixelArray,
    ///Third Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p3: PixelArray,
    ///Radius of the circle that defines the section of the Animation system which will be shown in the beam
    pub radius: Pixel,
}

///Helper struct to hold temporary data during deparsing
#[derive(Default)]
pub struct AnimationSystemDataHolder {
    ///First Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p1: Option<PixelArray>,
    ///Second Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p2: Option<PixelArray>,
    ///Third Point of the Spline describing the path of animation system in the beam in relation to the middle of the Media File
    pub p3: Option<PixelArray>,
    ///Radius of the circle that defines the section of the Animation system which will be shown in the beam
    pub radius: Option<Pixel>,
}

impl ReadGdtf for AnimationSystem {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = AnimationSystemDataHolder;
    const NODE_NAME: &'static [u8] = b"AnimationSystem";
    const PARENT_NODE_NAME: &'static [u8] = Slot::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"P1" => data_holder.p1 = Some(PixelArray::new_from_attr(attr)?),
            b"P2" => data_holder.p2 = Some(PixelArray::new_from_attr(attr)?),
            b"P3" => data_holder.p3 = Some(PixelArray::new_from_attr(attr)?),
            b"Radius" => data_holder.radius = Some(Pixel::new_from_attr(attr)?),
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
            p1: data_holder
                .p1
                .ok_or_else(|| Self::attribute_not_found(b"P1"))?,
            p2: data_holder
                .p2
                .ok_or_else(|| Self::attribute_not_found(b"P2"))?,
            p3: data_holder
                .p3
                .ok_or_else(|| Self::attribute_not_found(b"P3"))?,
            radius: data_holder
                .radius
                .ok_or_else(|| Self::attribute_not_found(b"Radius"))?,
        })
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for AnimationSystem {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(AnimationSystem {
                    p1: PixelArray(Pixel(-0.7), Pixel(0.0)),
                    p2: PixelArray(Pixel(0.0), Pixel(0.7)),
                    p3: PixelArray(Pixel(1.4), Pixel(0.7)),
                    radius: Pixel(0.5),
                }),
            ),
            (
                None,
                Some(AnimationSystem {
                    p1: PixelArray(Pixel(-0.7), Pixel(0.0)),
                    p2: PixelArray(Pixel(0.0), Pixel(0.7)),
                    p3: PixelArray(Pixel(1.4), Pixel(0.7)),
                    radius: Pixel(0.5),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<AnimationSystem P1="-0.700000,0.000000" P2="0.000000,0.700000" P3="1.400000,0.700000" Radius="0.500000"/>"#.to_string(),
            r#"<AnimationSystem P1="-0.700000,0.000000" P2="0.000000,0.700000" P3="1.400000,0.700000" Radius="0.500000"></AnimationSystem>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<AnimationSystem P1="-0.700000" P2="0.000000,0.700000" P3="1.400000,0.700000" Radius="0.500000"/>"#.to_string(),
            r#"<AnimationSystem P1="-0.700000,0.000000" P2="0.000000,0.700000" P3="1.400000,0.700000"/>"#.to_string(),
            r#"<AnimationSystem P1="-0.700000,0.000000" P2="0.000000,0.700000" Radius="0.500000"/>"#.to_string(),
            r#"<AnimationSystem P1="-0.700000,0.000000" P3="1.400000,0.700000" Radius="0.500000"/>"#.to_string(),
            r#"<AnimationSystem P2="0.000000,0.700000" P3="1.400000,0.700000" Radius="0.500000"/>"#.to_string()
        ]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::wheel::slot::animation_system::AnimationSystem;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        AnimationSystem::execute_tests();
    }
}
