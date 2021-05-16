#![cfg(test)]

use std::convert::TryInto;

use crate::fixture_type::FixtureType;

pub mod attribute_definitions;

pub fn expect() -> FixtureType {
    FixtureType {
        name: "ACME AE-610 BEAM".try_into().unwrap(),
        short_name: "ACME AE 610 BEAM".to_string(),
        long_name: "ACME AE 610 BEAM".to_string(),
        attribute_definitions: attribute_definitions::expect(),
        description: "ACME AE-610 BEAM".to_string(),
        fixture_type_id: "E62F2ECF-2A08-491D-BEEC-F5C491B89784".try_into().unwrap(),
        thumbnail: Some("AE-610 BEAM".to_string()),
        manufacturer: "ACME".to_string(),
        ref_ft: Some("8F54E11C-4C91-11E9-80BC-F1DFE217E634".try_into().unwrap()),
    }
}