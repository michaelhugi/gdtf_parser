#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::FixtureType;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    //<FixtureType Description="ACME AE-610 BEAM" FixtureTypeID="E62F2ECF-2A08-491D-BEEC-F5C491B89784" LongName="ACME AE 610 BEAM" Manufacturer="ACME" Name="ACME AE-610 BEAM" RefFT="8F54E11C-4C91-11E9-80BC-F1DFE217E634" ShortName="ACME AE 610 BEAM" Thumbnail="AE-610 BEAM">
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