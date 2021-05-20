#![cfg(test)]

use std::convert::TryInto;

use crate::fixture_type::FixtureType;

pub mod attribute_definitions;
pub mod dmx_mode;

pub fn expect() -> FixtureType {
    FixtureType {
        name: "Robin Viva CMY".try_into().unwrap(),
        short_name: "Viva™ CMY".to_string(),
        long_name: "Robin Viva™ CMY".to_string(),
        attribute_definitions: attribute_definitions::expect(),
        description: "Powerfully smooth, Robe’s VIVA CMY combines brightness of exceptionally clear zero-fringing white beam together with continuous color transitions of CMY mixing. ".to_string(),
        fixture_type_id: "BEB8B97D-FF49-4FBE-A834-9BE2C7BC689B".try_into().unwrap(),
        thumbnail: Some("thumbnail".to_string()),
        manufacturer: "Robe Lighting".to_string(),
        ref_ft: None,
        dmx_modes: vec![]
    }
}