#![cfg(test)]

use crate::gdtf::fixture_type::FixtureType;
use std::convert::TryInto;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType CanHaveChildren="Yes" Description="P12 Spot HP (High Power) 640W" FixtureTypeID="807DC00C-18D5-4133-B781-1A003FA988FA" LongName="P12 Spot HP" Manufacturer="JB-Lighting" Name="P12 Spot HP" RefFT="" ShortName="P12SPHP" Thumbnail="P12 dunkel">
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