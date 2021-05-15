///Physical Unit representation
#[derive(Debug)]
pub enum PhysicalUnit {
    None,
    Percent,
    /// (m)
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