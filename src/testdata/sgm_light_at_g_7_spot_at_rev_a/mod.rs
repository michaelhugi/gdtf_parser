#![cfg(test)]

use crate::gdtf::GDTF;
use crate::units::data_version::DataVersion;

pub mod fixture_type;

pub fn expect() -> GDTF {
    GDTF {
        data_version: DataVersion::Version1_1,
        fixture_type: fixture_type::expect(),
    }
}