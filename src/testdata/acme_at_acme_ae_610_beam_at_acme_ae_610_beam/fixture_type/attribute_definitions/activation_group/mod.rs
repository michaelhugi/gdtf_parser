#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::activation_group::ActivationGroup;

pub fn expect() -> Vec<ActivationGroup> {
    vec![
        ActivationGroup { name: "PanTilt".try_into().unwrap() },
        ActivationGroup { name: "Gobo1".try_into().unwrap() },
        ActivationGroup { name: "Prism".try_into().unwrap() },
        ActivationGroup { name: "ColorRGB".try_into().unwrap() }
    ]
}