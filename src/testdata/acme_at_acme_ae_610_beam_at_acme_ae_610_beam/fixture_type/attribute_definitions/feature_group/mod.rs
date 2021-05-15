#![cfg(test)]

use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;
use std::convert::TryInto;
use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;


pub fn expect() -> Vec<FeatureGroup> {
    vec![
        FeatureGroup{
            name:"Position".try_into().unwrap(),
            pretty:"Position".to_string(),
            features:vec![
                Feature{
                    name:"PanTilt".try_into().unwrap()
                }
            ]
        },
        FeatureGroup{
            name:"Gobo".try_into().unwrap(),
            pretty:"Gobo".to_string(),
            features:vec![
                Feature{
                    name:"Gobo".try_into().unwrap()
                }
            ]
        },
        FeatureGroup{
            name:"Beam".try_into().unwrap(),
            pretty:"Beam".to_string(),
            features:vec![
                Feature{
                    name:"Beam".try_into().unwrap()
                }
            ]
        },
        FeatureGroup{
            name:"Dimmer".try_into().unwrap(),
            pretty:"Dimmer".to_string(),
            features:vec![
                Feature{
                    name:"Dimmer".try_into().unwrap()
                }
            ]
        },
        FeatureGroup{
            name:"Control".try_into().unwrap(),
            pretty:"Control".to_string(),
            features:vec![
                Feature{
                    name:"Control".try_into().unwrap()
                }
            ]
        },
        FeatureGroup{
            name:"Color".try_into().unwrap(),
            pretty:"Color".to_string(),
            features:vec![
                Feature{
                    name:"Color".try_into().unwrap()
                }
            ]
        }
    ]

}