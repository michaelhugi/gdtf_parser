#![cfg(test)]

use std::convert::TryInto;

use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::utils::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
        Attribute {
            name: "Pan".try_into().unwrap(),
            pretty: "P".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Tilt".try_into().unwrap(),
            pretty: "T".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Control1".try_into().unwrap(),
            pretty: "Ctrl1".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "DimmerMode".try_into().unwrap(),
            pretty: "Dim Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Shutter1".try_into().unwrap(),
            pretty: "Sh1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Dimmer".try_into().unwrap(),
            pretty: "Dim".to_string(),
            activation_group: None,
            feature: "Dimmer.Dimmer".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Focus1".try_into().unwrap(),
            pretty: "Focus1".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Zoom".try_into().unwrap(),
            pretty: "Zoom".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Focus1Distance".try_into().unwrap(),
            pretty: "Focus1 Distance".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Length,
            color: None,
        },
        Attribute {
            name: "Focus1Adjust".try_into().unwrap(),
            pretty: "Focus1 Adjust".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Iris".try_into().unwrap(),
            pretty: "Iris".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Gobo1".try_into().unwrap(),
            pretty: "G1".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Gobo1SelectShake".try_into().unwrap(),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Gobo1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "Gobo1Pos".try_into().unwrap(),
            pretty: "G1 &lt;&gt;".to_string(),
            activation_group: Some("Gobo1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: "Gobo1PosRotate".try_into().unwrap(),
            pretty: "Rotate".to_string(),
            activation_group: Some("Gobo1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "Gobo2".try_into().unwrap(),
            pretty: "G2".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Gobo2SelectShake".try_into().unwrap(),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Gobo2WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: "Gobo2Pos".try_into().unwrap(),
            pretty: "G2 &lt;&gt;".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: "Gobo2PosRotate".try_into().unwrap(),
            pretty: "Rotate".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: "Color1".try_into().unwrap(),
            pretty: "C1".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Color1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: "CTO".try_into().unwrap(),
            pretty: "CTO".to_string(),
            activation_group: None,
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Sparkle".try_into().unwrap(),
            pretty: "Sparkle".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },

        //
        Attribute {
            name: "SparkleSpeed".try_into().unwrap(),
            pretty: "SparkleSpeed".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Frequency,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },

        //
        Attribute {
            name: "Prism1Pos".try_into().unwrap(),
            pretty: "Prism1 Pos".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: "Prism1PosRotate".try_into().unwrap(),
            pretty: "Rotate1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism1Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: "Prism2".try_into().unwrap(),
            pretty: "Prism2".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Prism1".try_into().unwrap(),
            pretty: "Prism1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Prism2Pos".try_into().unwrap(),
            pretty: "Prism2 Pos".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: "Frost1".try_into().unwrap(),
            pretty: "Frost1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Frost2".try_into().unwrap(),
            pretty: "Frost2".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Effects1".try_into().unwrap(),
            pretty: "FX1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "PositionMSpeed".try_into().unwrap(),
            pretty: "Pos MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "GlobalMSpeed".try_into().unwrap(),
            pretty: "Global MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "BlackoutMode".try_into().unwrap(),
            pretty: "Blackout Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Function".try_into().unwrap(),
            pretty: "Function".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "LEDFrequency".try_into().unwrap(),
            pretty: "LED Frequency".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "FixtureGlobalReset".try_into().unwrap(),
            pretty: "Fixture Global Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1Strobe".try_into().unwrap(),
            pretty: "Strobe1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobePulseOpen".try_into().unwrap(),
            pretty: "Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobePulseClose".try_into().unwrap(),
            pretty: "Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobePulse".try_into().unwrap(),
            pretty: "Pulse1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobeRandom".try_into().unwrap(),
            pretty: "Random1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobeRandomPulseClose".try_into().unwrap(),
            pretty: "Random Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Shutter1StrobeRandomPulseOpen".try_into().unwrap(),
            pretty: "Random Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: "Color1WheelIndex".try_into().unwrap(),
            pretty: "Wheel Index".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: "ColorSub_R".try_into().unwrap(),
            pretty: "R".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: "ColorSub_G".try_into().unwrap(),
            pretty: "G".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: "ColorSub_B".try_into().unwrap(),
            pretty: "B".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: "Prism2PosRotate".try_into().unwrap(),
            pretty: "Rotate2".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism2Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: "AnimationWheel1".try_into().unwrap(),
            pretty: "Anim1".to_string(),
            activation_group: Some("AnimationWheel1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: "AnimationWheel1PosRotate".try_into().unwrap(),
            pretty: "Anim Rotate".to_string(),
            activation_group: Some("AnimationWheel1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        }
    ]
}