#![cfg(test)]

use crate::fixture_type::attribute_definitions::AttributeDefinitions;

pub mod activation_group;
pub mod feature_group;
pub mod attribute;

pub fn expect() -> AttributeDefinitions {
    AttributeDefinitions {
        attributes: attribute::expect(),
        feature_groups: feature_group::expect(),
        activation_groups: activation_group::expect(),
    }
}