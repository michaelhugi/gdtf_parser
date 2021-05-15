#![cfg(test)]

use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::units::physical_unit::PhysicalUnit;
use std::convert::TryInto;

pub fn expect() -> Vec<Attribute> {
    vec![
        //<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Tilt" PhysicalUnit="Angle" Pretty="T"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="PositionMSpeed" PhysicalUnit="None" Pretty="Pos MSpeed"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="DMXInput" PhysicalUnit="None" Pretty="DMX Input"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="DisplayIntensity" PhysicalUnit="None" Pretty="Display Int"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="DimmerCurve" PhysicalUnit="None" Pretty="Dim Curve"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="NoFeature" PhysicalUnit="None" Pretty="NoFeature"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="FanMode" PhysicalUnit="None" Pretty="Fan Mode"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="PositionReset" PhysicalUnit="None" Pretty="Pos Reset"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="GoboWheelReset" PhysicalUnit="None" Pretty="G Reset"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="ZoomReset" PhysicalUnit="None" Pretty="Zoom Reset"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="FixtureGlobalReset" PhysicalUnit="None" Pretty="Fixture Global Reset"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="Fans" PhysicalUnit="None" Pretty="Fans"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_C" PhysicalUnit="ColorComponent" Pretty="C"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_M" PhysicalUnit="ColorComponent" Pretty="M"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_Y" PhysicalUnit="ColorComponent" Pretty="Y"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Color.RGB" Name="ColorMacro1" PhysicalUnit="None" Pretty="Color Macro1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="ColorMixMSpeed" PhysicalUnit="None" Pretty="Color Mix MSpeed"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="ColorWheelSelectMSpeed" PhysicalUnit="None" Pretty="Color Wheel Select MSpeed"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" Name="Effects1Rate" PhysicalUnit="Frequency" Pretty="FX1 Rate"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" Name="Gobo1" PhysicalUnit="None" Pretty="G1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1WheelAudio" PhysicalUnit="None" Pretty="Wheel Audio"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1WheelRandom" PhysicalUnit="Frequency" Pretty="Wheel Random"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" Name="Gobo2" PhysicalUnit="None" Pretty="G2"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2WheelAudio" PhysicalUnit="None" Pretty="Wheel Audio"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2WheelRandom" PhysicalUnit="Frequency" Pretty="Wheel Random"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2Pos" Feature="Gobo.Gobo" Name="Gobo2Pos" PhysicalUnit="Angle" Pretty="G2 &lt;&gt;"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo2Pos" Feature="Gobo.Gobo" MainAttribute="Gobo2Pos" Name="Gobo2PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Prism" Feature="Beam.Beam" Name="Prism1" PhysicalUnit="None" Pretty="Prism1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Prism" Feature="Beam.Beam" MainAttribute="Prism1" Name="Prism1Macro" PhysicalUnit="None" Pretty="Prism1 Macro"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Prism" Feature="Beam.Beam" Name="Prism1PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" Name="Frost1" PhysicalUnit="None" Pretty="Frost1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Focus.Focus" Name="Zoom" PhysicalUnit="Angle" Pretty="Zoom"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Focus.Focus" Name="Focus1" PhysicalUnit="None" Pretty="Focus1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Dimmer.Dimmer" Name="Dimmer" PhysicalUnit="None" Pretty="Dim"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" Name="Color1" PhysicalUnit="None" Pretty="C1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="Control1" PhysicalUnit="None" Pretty="Ctrl1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelAudio" PhysicalUnit="None" Pretty="Wheel Audio"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" MainAttribute="Color1" Name="Color1WheelRandom" PhysicalUnit="Frequency" Pretty="Wheel Random"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="Gobo1" Feature="Gobo.Gobo" MainAttribute="Gobo1" Name="Gobo1SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" Name="Iris" PhysicalUnit="None" Pretty="Iris"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisPulseOpen" PhysicalUnit="Frequency" Pretty="Pulse Open"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisPulseClose" PhysicalUnit="Frequency" Pretty="Pulse Close"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Iris" Name="IrisStrobeRandom" PhysicalUnit="Frequency" Pretty="Random Strobe"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" Name="Shutter1" PhysicalUnit="None" Pretty="Sh1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1Strobe" PhysicalUnit="Frequency" Pretty="Strobe1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseOpen" PhysicalUnit="Frequency" Pretty="Pulse Open1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseClose" PhysicalUnit="Frequency" Pretty="Pulse Close1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeRandom" PhysicalUnit="Frequency" Pretty="Random1"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute ActivationGroup="PanTilt" Feature="Position.PanTilt" Name="Pan" PhysicalUnit="Angle" Pretty="P"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="BlackoutMode" PhysicalUnit="None" Pretty="Blackout Mode"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="RoboSpot" PhysicalUnit="None" Pretty=""/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="PanTiltMode" PhysicalUnit="None" Pretty="PanTilt Mode"/>

        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },//<Attribute Feature="Control.Control" Name="ColorMixReset" PhysicalUnit="None" Pretty="Color Mix Reset"/>
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