#![cfg(test)]

use crate::gdtf::fixture_type::FixtureType;
use std::convert::TryInto;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType CanHaveChildren="Yes" Description="The G-7 Spot is the quintessence of IP-rated moving heads. A fast, compact, and lightweight mid-sized moving head spot with high-output and low power consumption. Thanks to its white LED engine and CMY color mixing, the G-7 Spot is the perfect moving head for those who need maximum light output inside an easy-to-move luminaire. The G-7 Spot gives you a solid construction, a high-quality beam, and an optimal projection in a very flexible assembly. A fixture born to rock night after night." FixtureTypeID="14030EC0-9085-4756-8B19-8B08369E06B9" LongName="G-7 Spot" Manufacturer="SGM Light" Name="G-7 Spot" RefFT="" ShortName="G-7 Spot" Thumbnail="G-7_RAL_black_small">
    //
    FixtureType {
        name: "".try_into().unwrap(),
        short_name: "".to_string(),
        long_name: "".to_string(),
        attribute_definitions: attribute_definitions::expect(),
        description: "".to_string(),
        fixture_type_id: "".try_into().unwrap(),
        thumbnail: None,
        manufacturer: "".to_string(),
        ref_ft: None,
    }
}