#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::FixtureType;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType CanHaveChildren="Yes" Description="Powerfully smooth, Robe’s VIVA CMY combines brightness of exceptionally clear zero-fringing white beam together with continuous color transitions of CMY mixing. " FixtureTypeID="BEB8B97D-FF49-4FBE-A834-9BE2C7BC689B" LongName="Robin Viva™ CMY" Manufacturer="Robe Lighting" Name="Robin Viva CMY" RefFT="" ShortName="Viva™ CMY" Thumbnail="thumbnail">
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