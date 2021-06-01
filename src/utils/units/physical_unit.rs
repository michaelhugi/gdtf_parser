//TODO check
//! Module for the unit PhysicalUnit used in GDTF


///Physical Unit representation used in GDTF
#[derive(Debug, PartialEq)]
pub enum PhysicalUnit {
    None,
    Percent,
    ///(m)
    Length,
    ///(kg)
    Mass,
    /// (s)
    Time,
    ///(K)
    Temperature,
    ///(cd)
    LuminousIntensity,
    /// (degree)
    Angle,
    ///(N)
    Force,
    ///(Hz)
    Frequency,
    /// (A)
    Current,
    ///(V)
    Voltage,
    ///(W)
    Power,
    ///(J)
    Energy,
    /// (m2)
    Area,
    ///(m3)
    Volume,
    ///(m / s)
    Speed,
    ///(m / s2)
    Acceleration,
    ///(degree / s)
    AngularSpeed,
    ///(degree / s2)
    AngularAccc,
    ///(nm)
    WaveLength,
    ColorComponent,
}

impl From<&str> for PhysicalUnit {
    fn from(s: &str) -> Self {
        use PhysicalUnit::*;
        match s {
            "Percent" => Percent,
            "Length" => Length,
            "Mass" => Mass,
            "Time" => Time,
            "Temperature" => Temperature,
            "LuminousIntensity" => LuminousIntensity,
            "Angle" => Angle,
            "Force" => Force,
            "Frequency" => Frequency,
            "Current" => Current,
            "Voltage" => Voltage,
            "Power" => Power,
            "Energy" => Energy,
            "Area" => Area,
            "Volume" => Volume,
            "Speed" => Speed,
            "Acceleration" => Acceleration,
            "AngularSpeed" => AngularSpeed,
            "AngularAccc" => AngularAccc,
            "WaveLength" => WaveLength,
            "ColorComponent" => ColorComponent,
            _ => None
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_valid() {
        assert_eq!(
            PhysicalUnit::Energy,
            PhysicalUnit::try_from("Energy").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            PhysicalUnit::None,
            PhysicalUnit::try_from("None").unwrap()
        );
    }


    #[test]
    fn test_invalid_2() {
        assert_eq!(
            PhysicalUnit::None,
            PhysicalUnit::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            PhysicalUnit::None,
            PhysicalUnit::try_from("").unwrap()
        );
    }
}