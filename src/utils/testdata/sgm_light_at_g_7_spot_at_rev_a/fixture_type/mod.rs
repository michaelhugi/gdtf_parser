#![cfg(test)]


use std::convert::TryInto;

use crate::fixture_type::FixtureType;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType CanHaveChildren="Yes" >
    //
    FixtureType {
        name: "G-7 Spot".try_into().unwrap(),
        short_name: "G-7 Spot".to_string(),
        long_name: "G-7 Spot".to_string(),
        attribute_definitions: attribute_definitions::expect(),
        description: "The G-7 Spot is the quintessence of IP-rated moving heads. A fast, compact, and lightweight mid-sized moving head spot with high-output and low power consumption. Thanks to its white LED engine and CMY color mixing, the G-7 Spot is the perfect moving head for those who need maximum light output inside an easy-to-move luminaire. The G-7 Spot gives you a solid construction, a high-quality beam, and an optimal projection in a very flexible assembly. A fixture born to rock night after night.".to_string(),
        fixture_type_id: "14030EC0-9085-4756-8B19-8B08369E06B9".try_into().unwrap(),
        thumbnail: Some("G-7_RAL_black_small".to_string()),
        manufacturer: "SGM Light".to_string(),
        ref_ft: None,
    }
}