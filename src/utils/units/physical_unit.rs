//! Module for the unit PhysicalUnit used in GDTF
use std::fmt::{Display, Formatter};

///Physical Unit representation used in GDTF
#[derive(Debug)]
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
        match s {
            "Percent" => PhysicalUnit::Percent,
            "Length" => PhysicalUnit::Length,
            "Mass" => PhysicalUnit::Mass,
            "Time" => PhysicalUnit::Time,
            "Temperature" => PhysicalUnit::Temperature,
            "LuminousIntensity" => PhysicalUnit::LuminousIntensity,
            "Angle" => PhysicalUnit::Angle,
            "Force" => PhysicalUnit::Force,
            "Frequency" => PhysicalUnit::Frequency,
            "Current" => PhysicalUnit::Current,
            "Voltage" => PhysicalUnit::Voltage,
            "Power" => PhysicalUnit::Power,
            "Energy" => PhysicalUnit::Energy,
            "Area" => PhysicalUnit::Area,
            "Volume" => PhysicalUnit::Volume,
            "Speed" => PhysicalUnit::Speed,
            "Acceleration" => PhysicalUnit::Acceleration,
            "AngularSpeed" => PhysicalUnit::AngularSpeed,
            "AngularAccc" => PhysicalUnit::AngularAccc,
            "WaveLength" => PhysicalUnit::WaveLength,
            "ColorComponent" => PhysicalUnit::ColorComponent,
            _ => PhysicalUnit::None
        }
    }
}

#[cfg(test)]
impl PartialEq for PhysicalUnit {
    fn eq(&self, other: &Self) -> bool {
        match self {
            PhysicalUnit::None => {
                match other {
                    PhysicalUnit::None => true,
                    _ => false
                }
            }
            PhysicalUnit::Percent => {
                match other {
                    PhysicalUnit::Percent => true,
                    _ => false
                }
            }
            PhysicalUnit::Length => {
                match other {
                    PhysicalUnit::Length => true,
                    _ => false
                }
            }
            PhysicalUnit::Mass => {
                match other {
                    PhysicalUnit::Mass => true,
                    _ => false
                }
            }
            PhysicalUnit::Time => {
                match other {
                    PhysicalUnit::Time => true,
                    _ => false
                }
            }
            PhysicalUnit::Temperature => {
                match other {
                    PhysicalUnit::Temperature => true,
                    _ => false
                }
            }
            PhysicalUnit::LuminousIntensity => {
                match other {
                    PhysicalUnit::LuminousIntensity => true,
                    _ => false
                }
            }
            PhysicalUnit::Angle => {
                match other {
                    PhysicalUnit::Angle => true,
                    _ => false
                }
            }
            PhysicalUnit::Force => {
                match other {
                    PhysicalUnit::Force => true,
                    _ => false
                }
            }
            PhysicalUnit::Frequency => {
                match other {
                    PhysicalUnit::Frequency => true,
                    _ => false
                }
            }
            PhysicalUnit::Current => {
                match other {
                    PhysicalUnit::Current => true,
                    _ => false
                }
            }
            PhysicalUnit::Voltage => {
                match other {
                    PhysicalUnit::Voltage => true,
                    _ => false
                }
            }
            PhysicalUnit::Power => {
                match other {
                    PhysicalUnit::Power => true,
                    _ => false
                }
            }
            PhysicalUnit::Energy => {
                match other {
                    PhysicalUnit::Energy => true,
                    _ => false
                }
            }
            PhysicalUnit::Area => {
                match other {
                    PhysicalUnit::Area => true,
                    _ => false
                }
            }
            PhysicalUnit::Volume => {
                match other {
                    PhysicalUnit::Volume => true,
                    _ => false
                }
            }
            PhysicalUnit::Speed => {
                match other {
                    PhysicalUnit::Speed => true,
                    _ => false
                }
            }
            PhysicalUnit::Acceleration => {
                match other {
                    PhysicalUnit::Acceleration => true,
                    _ => false
                }
            }
            PhysicalUnit::AngularSpeed => {
                match other {
                    PhysicalUnit::AngularSpeed => true,
                    _ => false
                }
            }
            PhysicalUnit::AngularAccc => {
                match other {
                    PhysicalUnit::AngularAccc => true,
                    _ => false
                }
            }
            PhysicalUnit::WaveLength => {
                match other {
                    PhysicalUnit::WaveLength => true,
                    _ => false
                }
            }
            PhysicalUnit::ColorComponent => {
                match other {
                    PhysicalUnit::ColorComponent => true,
                    _ => false
                }
            }
        }
    }
}

impl Display for PhysicalUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PhysicalUnit::None => write!(f, "None"),
            PhysicalUnit::Percent => write!(f, "Percent"),
            PhysicalUnit::Length => write!(f, "Length (m)"),
            PhysicalUnit::Mass => write!(f, "Mass (kg)"),
            PhysicalUnit::Time => write!(f, "Time (s)"),
            PhysicalUnit::Temperature => write!(f, "Temperature (K)"),
            PhysicalUnit::LuminousIntensity => write!(f, "LuminousIntensity (cd)"),
            PhysicalUnit::Angle => write!(f, "Angle (degree)"),
            PhysicalUnit::Force => write!(f, "Force (N)"),
            PhysicalUnit::Frequency => write!(f, "Frequency (Hz)"),
            PhysicalUnit::Current => write!(f, "Current (A)"),
            PhysicalUnit::Voltage => write!(f, "Voltage (V)"),
            PhysicalUnit::Power => write!(f, "Power (W)"),
            PhysicalUnit::Energy => write!(f, "Energy (J)"),
            PhysicalUnit::Area => write!(f, "Area (m2)"),
            PhysicalUnit::Volume => write!(f, "Volume (m3)"),
            PhysicalUnit::Speed => write!(f, "Speed (m / s)"),
            PhysicalUnit::Acceleration => write!(f, "Acceleration (m / s2)"),
            PhysicalUnit::AngularSpeed => write!(f, "AngularSpeed (degree / s)"),
            PhysicalUnit::AngularAccc => write!(f, "AngularAccc (degree / s2"),
            PhysicalUnit::WaveLength => write!(f, "WaveLength (nm)"),
            PhysicalUnit::ColorComponent => write!(f, "ColorComponent"),
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

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PhysicalUnit::Area), "Area (m2)");
    }
}