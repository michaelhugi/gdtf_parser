#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
        //<Attribute Feature="Beam.Beam" Name="Shutter1" PhysicalUnit="None" Pretty="Sh1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="None" Pretty="Dim"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" Name="Color1" PhysicalUnit="None" Pretty="C1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="Angle" Pretty="T"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1WheelIndex" PhysicalUnit="Angle" Pretty="Wheel Index"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Focus.Focus" Name="Zoom" PhysicalUnit="Angle" Pretty="Zoom"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="Function" PhysicalUnit="None" Pretty="Function"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Reserved" PhysicalUnit="None" Pretty="Reserved"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeEffect" PhysicalUnit="Frequency" Pretty="Effect1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1Strobe" PhysicalUnit="Frequency" Pretty="Strobe1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseOpen" PhysicalUnit="Frequency" Pretty="Pulse Open1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeRandom" PhysicalUnit="Frequency" Pretty="Random1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseClose" PhysicalUnit="Frequency" Pretty="Pulse Close1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulse" PhysicalUnit="Frequency" Pretty="Pulse1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="NoFeature" PhysicalUnit="None" Pretty="NoFeature"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelIndex" PhysicalUnit="Angle" Pretty="Wheel Index"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorIndirect" Feature="Color.Indirect" Name="ColorRGB_Cyan" PhysicalUnit="None" Pretty="C"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorIndirect" Feature="Color.Indirect" Name="ColorRGB_Magenta" PhysicalUnit="None" Pretty="M"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="ColorIndirect" Feature="Color.Indirect" Name="ColorRGB_Yellow" PhysicalUnit="None" Pretty="Y"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Color.Color" Name="CTO" PhysicalUnit="None" Pretty="CTO"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" Name="Gobo2" PhysicalUnit="None" Pretty="G2"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Focus.Focus" Name="Focus1" PhysicalUnit="None" Pretty="Focus1"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" Name="Iris" PhysicalUnit="None" Pretty="Iris"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisPulseOpen" PhysicalUnit="Frequency" Pretty="Pulse Open"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisPulseClose" PhysicalUnit="Frequency" Pretty="Pulse Close"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisRandomPulseOpen" PhysicalUnit="Frequency" Pretty="Random Pulse Open"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" Name="Prism1" PhysicalUnit="None" Pretty="Prism1"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" Name="Frost1" PhysicalUnit="None" Pretty="Frost1"/>
        //
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }
    ]
}