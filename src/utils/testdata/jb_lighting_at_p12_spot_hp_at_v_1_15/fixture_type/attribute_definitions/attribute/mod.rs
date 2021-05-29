#![cfg(test)]

use std::convert::TryInto;

use crate::fixture_type::attribute_definitions::attribute::Attribute;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::name::Name;
use crate::utils::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
        Attribute {
            name: AttributeName::Pan,
            pretty: "P".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: AttributeName::Tilt,
            pretty: "T".to_string(),
            activation_group: Some("PanTilt".to_string()),
            feature: "Position.PanTilt".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: AttributeName::Control_n_(1),
            pretty: "Ctrl1".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::DimmerMode,
            pretty: "Dim Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Shutter_n_(1),
            pretty: "Sh1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Dimmer,
            pretty: "Dim".to_string(),
            activation_group: None,
            feature: "Dimmer.Dimmer".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Focus_n_(1),
            pretty: "Focus1".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Zoom,
            pretty: "Zoom".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: AttributeName::Focus_n_Distance(1),
            pretty: "Focus1 Distance".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Length,
            color: None,
        },
        Attribute {
            name: AttributeName::Focus_n_Adjust(1),
            pretty: "Focus1 Adjust".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Iris,
            pretty: "Iris".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_(1),
            pretty: "G1".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_SelectShake(1),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_WheelSpin(1),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_Pos(1),
            pretty: "G1 &lt;&gt;".to_string(),
            activation_group: Some("Gobo1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_PosRotate(1),
            pretty: "Rotate".to_string(),
            activation_group: Some("Gobo1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_(2),
            pretty: "G2".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: AttributeName::Gobo_n_SelectShake(2),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Gobo_n_WheelSpin(2),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Gobo_n_Pos(2),
            pretty: "G2 &lt;&gt;".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Gobo_n_PosRotate(2),
            pretty: "Rotate".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Color_n_(1),
            pretty: "C1".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Color_n_WheelSpin(1),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::CTO,
            pretty: "CTO".to_string(),
            activation_group: None,
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("Sparkle")),
            pretty: "Sparkle".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },

        //
        Attribute {
            name: AttributeName::UserDefined(Name::new_unchecked("SparkleSpeed")),
            pretty: "SparkleSpeed".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Frequency,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },

        //
        Attribute {
            name: AttributeName::Prism_n_Pos(1),
            pretty: "Prism1 Pos".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Prism_n_PosRotate(1),
            pretty: "Rotate1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism1Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Prism_n_(2),
            pretty: "Prism2".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Prism_n_(1),
            pretty: "Prism1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Prism_n_Pos(2),
            pretty: "Prism2 Pos".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Frost_n_(1),
            pretty: "Frost1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Frost_n_(2),
            pretty: "Frost2".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Effects_n_(1),
            pretty: "FX1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::PositionMSpeed,
            pretty: "Pos MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::GlobalMSpeed,
            pretty: "Global MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::BlackoutMode,
            pretty: "Blackout Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Function,
            pretty: "Function".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::LEDFrequency,
            pretty: "LED Frequency".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::FixtureGlobalReset,
            pretty: "Fixture Global Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_Strobe(1),
            pretty: "Strobe1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobePulseOpen(1),
            pretty: "Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobePulseClose(1),
            pretty: "Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobePulse(1),
            pretty: "Pulse1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobeRandom(1),
            pretty: "Random1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobeRandomPulseClose(1),
            pretty: "Random Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Shutter_n_StrobeRandomPulseOpen(1),
            pretty: "Random Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Color_n_WheelIndex(1),
            pretty: "Wheel Index".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::ColorSub_R,
            pretty: "R".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::ColorSub_G,
            pretty: "G".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::ColorSub_B,
            pretty: "B".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::Prism_n_PosRotate(2),
            pretty: "Rotate2".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism2Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::AnimationWheel_n_(1),
            pretty: "Anim1".to_string(),
            activation_group: Some("AnimationWheel1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

        //
        Attribute {
            name: AttributeName::AnimationWheel_n_PosRotate(1),
            pretty: "Anim Rotate".to_string(),
            activation_group: Some("AnimationWheel1Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        }
    ]
}