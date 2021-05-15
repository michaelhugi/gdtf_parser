#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;
use crate::gdtf::fixture_type::attribute_definitions::feature_group::FeatureGroup;

pub fn expect() -> Vec<FeatureGroup> {
    vec![
        //<FeatureGroup Name="Beam" Pretty="Beam">
        //           <Feature Name="Beam"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        //  <FeatureGroup Name="Dimmer" Pretty="Dimmer">
        //           <Feature Name="Dimmer"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        //    <FeatureGroup Name="Color" Pretty="Color">
        //           <Feature Name="Color"/>
        //           <Feature Name="Indirect"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        //  <FeatureGroup Name="Position" Pretty="Position">
        //           <Feature Name="PanTilt"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        //       <FeatureGroup Name="Gobo" Pretty="Gobo">
        //           <Feature Name="Gobo"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        // <FeatureGroup Name="Focus" Pretty="Focus">
        //           <Feature Name="Focus"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        },
        //   <FeatureGroup Name="Control" Pretty="Control">
        //           <Feature Name="Control"/>
        //         </FeatureGroup>
        FeatureGroup {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            features: vec![
                Feature {
                    name: "".try_into().unwrap()
                }
            ],
        }
    ]
}