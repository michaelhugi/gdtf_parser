#![cfg(test)]

use std::convert::TryInto;

use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
use crate::units::physical_unit::PhysicalUnit;

pub fn expect() -> Vec<Attribute> {
    vec![
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
            name: "PositionMSpeed".try_into().unwrap(),
            pretty: "Pos MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "DMXInput".try_into().unwrap(),
            pretty: "DMX Input".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "DisplayIntensity".try_into().unwrap(),
            pretty: "Display Int".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "DimmerCurve".try_into().unwrap(),
            pretty: "Dim Curve".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "NoFeature".try_into().unwrap(),
            pretty: "NoFeature".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "FanMode".try_into().unwrap(),
            pretty: "Fan Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "PositionReset".try_into().unwrap(),
            pretty: "Pos Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "GoboWheelReset".try_into().unwrap(),
            pretty: "G Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "ZoomReset".try_into().unwrap(),
            pretty: "Zoom Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "FixtureGlobalReset".try_into().unwrap(),
            pretty: "Fixture Global Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Fans".try_into().unwrap(),
            pretty: "Fans".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "ColorSub_C".try_into().unwrap(),
            pretty: "C".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },
        Attribute {
            name: "ColorSub_M".try_into().unwrap(),
            pretty: "M".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },
        Attribute {
            name: "ColorSub_Y".try_into().unwrap(),
            pretty: "Y".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::ColorComponent,
            color: None,
        },
        Attribute {
            name: "ColorMacro1".try_into().unwrap(),
            pretty: "Color Macro1".to_string(),
            activation_group: None,
            feature: "Color.RGB".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "ColorMixMSpeed".try_into().unwrap(),
            pretty: "Color Mix MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "ColorWheelSelectMSpeed".try_into().unwrap(),
            pretty: "Color Wheel Select MSpeed".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Effects1Rate".try_into().unwrap(),
            pretty: "FX1 Rate".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Frequency,
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

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo1WheelAudio".try_into().unwrap(),
            pretty: "Wheel Audio".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo1WheelRandom".try_into().unwrap(),
            pretty: "Wheel Random".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
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

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo2SelectShake".try_into().unwrap(),
            pretty: "Select Shake".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo2WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo2WheelAudio".try_into().unwrap(),
            pretty: "Wheel Audio".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo2WheelRandom".try_into().unwrap(),
            pretty: "Wheel Random".to_string(),
            activation_group: Some("Gobo2".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
        Attribute {
            name: "Gobo2Pos".try_into().unwrap(),
            pretty: "G2 &lt;&gt;".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo2PosRotate".try_into().unwrap(),
            pretty: "Rotate".to_string(),
            activation_group: Some("Gobo2Pos".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo2Pos".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "Prism1".try_into().unwrap(),
            pretty: "Prism1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Prism1Macro".try_into().unwrap(),
            pretty: "Prism1 Macro".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Prism1".to_string()),
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "Prism1PosRotate".try_into().unwrap(),
            pretty: "Rotate1".to_string(),
            activation_group: Some("Prism".to_string()),
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },
        Attribute {
            name: "Frost1".try_into().unwrap(),
            pretty: "Frost1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
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
            name: "Focus1".try_into().unwrap(),
            pretty: "Focus1".to_string(),
            activation_group: None,
            feature: "Focus.Focus".to_string(),
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
            name: "Color1".try_into().unwrap(),
            pretty: "C1".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Color1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
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

//<Attribute  Pretty=""/>

        Attribute {
            name: "Color1WheelAudio".try_into().unwrap(),
            pretty: "Wheel Audio".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Color1WheelRandom".try_into().unwrap(),
            pretty: "Wheel Random".to_string(),
            activation_group: Some("ColorRGB".to_string()),
            feature: "Color.Color".to_string(),
            main_attribute: Some("Color1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Gobo1WheelSpin".try_into().unwrap(),
            pretty: "Wheel Spin".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Gobo.Gobo".to_string(),
            main_attribute: Some("Gobo1".to_string()),
            physical_unit: PhysicalUnit::AngularSpeed,
            color: None,
        },

//<Attribute  Pretty=""/>

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
            name: "Iris".try_into().unwrap(),
            pretty: "Iris".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "IrisPulseOpen".try_into().unwrap(),
            pretty: "Pulse Open".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Iris".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "IrisPulseClose".try_into().unwrap(),
            pretty: "Pulse Close".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Iris".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "IrisStrobeRandom".try_into().unwrap(),
            pretty: "Random Strobe".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Iris".to_string()),
            physical_unit: PhysicalUnit::Frequency,
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

//<Attribute  Pretty=""/>

        Attribute {
            name: "Shutter1Strobe".try_into().unwrap(),
            pretty: "Strobe1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Shutter1StrobePulseOpen".try_into().unwrap(),
            pretty: "Pulse Open1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Shutter1StrobePulseClose".try_into().unwrap(),
            pretty: "Pulse Close1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },

//<Attribute  Pretty=""/>

        Attribute {
            name: "Shutter1StrobeRandom".try_into().unwrap(),
            pretty: "Random1".to_string(),
            activation_group: None,
            feature: "Beam.Beam".to_string(),
            main_attribute: Some("Shutter1".to_string()),
            physical_unit: PhysicalUnit::Frequency,
            color: None,
        },
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
            name: "BlackoutMode".try_into().unwrap(),
            pretty: "Blackout Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "RoboSpot".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: Some("0.312700,0.329000,100.000000".try_into().unwrap()),
        },
        Attribute {
            name: "PanTiltMode".try_into().unwrap(),
            pretty: "PanTilt Mode".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        },
        Attribute {
            name: "ColorMixReset".try_into().unwrap(),
            pretty: "Color Mix Reset".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }
    ]
}