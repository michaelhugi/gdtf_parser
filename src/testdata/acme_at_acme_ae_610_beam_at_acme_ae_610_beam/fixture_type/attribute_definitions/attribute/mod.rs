#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
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
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" Name="Prism1" PhysicalUnit="None" Pretty="Prism1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Beam.Beam" Name="Prism1Pos" PhysicalUnit="Angle" Pretty="Prism1 Pos"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
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
        //<Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="LuminousIntensity" Pretty="Dim"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="LampControl" PhysicalUnit="None" Pretty="Lamp Ctrl"/>
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
        //<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" MainAttribute="Prism1Pos" Name="Prism1PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate1"/>
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
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="PositionReset" PhysicalUnit="None" Pretty="Pos Reset"/>
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
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="PT Speed" PhysicalUnit="None" Pretty="PT Speed"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="ShutterReset" PhysicalUnit="None" Pretty="Shutter Reset"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Lamp On" PhysicalUnit="None" Pretty="Lamp On"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Lamp Off" PhysicalUnit="None" Pretty="Lamp Off"/>
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
        //<Attribute Feature="Control.Control" Name="ColorMixReset" PhysicalUnit="None" Pretty="Color Mix Reset"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="GoboWheelReset" PhysicalUnit="None" Pretty="G Reset"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="FixtureGlobalReset" PhysicalUnit="None" Pretty="Fixture Global Reset"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Sound" PhysicalUnit="None" Pretty="Sound"/>
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