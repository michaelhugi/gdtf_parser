#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::FixtureType;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType CanHaveChildren="Yes">
    //
    FixtureType {
        name: "P12 Spot HP".try_into().unwrap(),
        short_name: "P12SPHP".to_string(),
        long_name: "P12 Spot HP".to_string(),
        attribute_definitions: attribute_definitions::expect(),
        description: "P12 Spot HP (High Power) 640W".to_string(),
        fixture_type_id: "807DC00C-18D5-4133-B781-1A003FA988FA".try_into().unwrap(),
        thumbnail: Some("P12 dunkel".to_string()),
        manufacturer: "JB-Lighting".to_string(),
        ref_ft: None,
    }
}