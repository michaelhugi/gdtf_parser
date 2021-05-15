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
        //<Attribute Feature="Control.Control" Name="Control1" PhysicalUnit="None" Pretty="Ctrl1"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Control.Control" Name="DimmerMode" PhysicalUnit="None" Pretty="Dim Mode"/>
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
        //<Attribute Feature="Focus.Focus" Name="Focus1" PhysicalUnit="None" Pretty="Focus1"/>
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
        //<Attribute Feature="Focus.Focus" Name="Focus1Distance" PhysicalUnit="Length" Pretty="Focus1 Distance"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute Feature="Focus.Focus" Name="Focus1Adjust" PhysicalUnit="None" Pretty="Focus1 Adjust"/>
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
        //<Attribute ActivationGroup="Gobo1Pos" Feature="Gobo.Gobo" Name="Gobo1Pos" PhysicalUnit="Angle" Pretty="G1 &lt;&gt;"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo1Pos" Feature="Gobo.Gobo" MainAttribute="Gobo1Pos" Name="Gobo1PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate"/>
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
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2SelectShake" PhysicalUnit="Frequency" Pretty="Select Shake"/>
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        //<Attribute ActivationGroup="Gobo2" Feature="Gobo.Gobo" MainAttribute="Gobo2" Name="Gobo2WheelSpin" PhysicalUnit="AngularSpeed" Pretty="Wheel Spin"/>
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
        //<Attribute ActivationGroup="Gobo2Pos" Feature="Gobo.Gobo" Name="Gobo2Pos" PhysicalUnit="Angle" Pretty="G2 &lt;&gt;"/>
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
        //<Attribute ActivationGroup="Gobo2Pos" Feature="Gobo.Gobo" MainAttribute="Gobo2Pos" Name="Gobo2PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate"/>
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
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.Color" Name="Color1" PhysicalUnit="None" Pretty="C1"/>
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
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Beam.Beam" Name="Sparkle" PhysicalUnit="None" Pretty="Sparkle"/>
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
        //<Attribute Color="0.312700,0.329000,100.000000" Feature="Beam.Beam" Name="SparkleSpeed" PhysicalUnit="Frequency" Pretty="SparkleSpeed"/>
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
        //<Attribute Feature="Beam.Beam" Name="Prism1Pos" PhysicalUnit="Angle" Pretty="Prism1 Pos"/>
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
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" MainAttribute="Prism1Pos" Name="Prism1PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate1"/>
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
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" Name="Prism2" PhysicalUnit="None" Pretty="Prism2"/>
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
        //<Attribute Feature="Beam.Beam" Name="Prism2Pos" PhysicalUnit="Angle" Pretty="Prism2 Pos"/>
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
        },
        //<Attribute Feature="Beam.Beam" Name="Frost2" PhysicalUnit="None" Pretty="Frost2"/>
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
        //<Attribute Feature="Beam.Beam" Name="Effects1" PhysicalUnit="None" Pretty="FX1"/>
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
        //<Attribute Feature="Control.Control" Name="PositionMSpeed" PhysicalUnit="None" Pretty="Pos MSpeed"/>
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
        //<Attribute Feature="Control.Control" Name="GlobalMSpeed" PhysicalUnit="None" Pretty="Global MSpeed"/>
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
        //<Attribute Feature="Control.Control" Name="BlackoutMode" PhysicalUnit="None" Pretty="Blackout Mode"/>
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
        //<Attribute Feature="Control.Control" Name="Function" PhysicalUnit="None" Pretty="Function"/>
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
        //<Attribute Feature="Control.Control" Name="LEDFrequency" PhysicalUnit="Frequency" Pretty="LED Frequency"/>
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
        //<Attribute Feature="Control.Control" Name="FixtureGlobalReset" PhysicalUnit="None" Pretty="Fixture Global Reset"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1Strobe" PhysicalUnit="Frequency" Pretty="Strobe1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseOpen" PhysicalUnit="Frequency" Pretty="Pulse Open1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulseClose" PhysicalUnit="Frequency" Pretty="Pulse Close1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobePulse" PhysicalUnit="Frequency" Pretty="Pulse1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeRandom" PhysicalUnit="Frequency" Pretty="Random1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeRandomPulseClose" PhysicalUnit="Frequency" Pretty="Random Pulse Close1"/>
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
        //<Attribute Feature="Beam.Beam" MainAttribute="Shutter1" Name="Shutter1StrobeRandomPulseOpen" PhysicalUnit="Frequency" Pretty="Random Pulse Open1"/>
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
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_R" PhysicalUnit="ColorComponent" Pretty="R"/>
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
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_G" PhysicalUnit="ColorComponent" Pretty="G"/>
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
        //<Attribute ActivationGroup="ColorRGB" Feature="Color.RGB" Name="ColorSub_B" PhysicalUnit="ColorComponent" Pretty="B"/>
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
        //<Attribute ActivationGroup="Prism" Feature="Beam.Beam" MainAttribute="Prism2Pos" Name="Prism2PosRotate" PhysicalUnit="AngularSpeed" Pretty="Rotate2"/>
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
        //<Attribute ActivationGroup="AnimationWheel1" Feature="Gobo.Gobo" Name="AnimationWheel1" PhysicalUnit="None" Pretty="Anim1"/>
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
        //<Attribute ActivationGroup="AnimationWheel1Pos" Feature="Gobo.Gobo" Name="AnimationWheel1PosRotate" PhysicalUnit="AngularSpeed" Pretty="Anim Rotate"/>
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