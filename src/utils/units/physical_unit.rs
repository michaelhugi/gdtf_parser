//! Module for the unit PhysicalUnit used in GDTF

use std::fmt;
use std::fmt::Formatter;

use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

///Physical Unit representation used in GDTF
#[derive(Debug, PartialEq, Clone)]
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

impl PhysicalUnit {
    ///Creates a new Physical unit from a str defined in gdtf-xml.
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::physical_unit::PhysicalUnit;
    ///
    /// assert_eq!(PhysicalUnit::Time, PhysicalUnit::new_from_str("Time"));
    /// assert_eq!(PhysicalUnit::ColorComponent, PhysicalUnit::new_from_str("ColorComponent"));
    /// assert_eq!(PhysicalUnit::None, PhysicalUnit::new_from_str("None"));
    /// assert_eq!(PhysicalUnit::None, PhysicalUnit::new_from_str("Something else"));
    /// ```
    pub fn new_from_str(s: &str) -> Self {
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
            _ => None,
        }
    }
}

impl<'de> Deserialize<'de> for PhysicalUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(PhysicalUnitVisitor)
    }
}

struct PhysicalUnitVisitor;

impl<'de> Visitor<'de> for PhysicalUnitVisitor {
    type Value = PhysicalUnit;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("valid PhysicalUnit String")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(PhysicalUnit::new_from_str(v))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        self.visit_str(&v)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::units::physical_unit::PhysicalUnit as T;

    #[test]
    fn test_new_from_str() {
        use T::*;
        assert_eq!(None, T::new_from_str("None"));
        assert_eq!(None, T::new_from_str("Something else"));
        assert_eq!(None, T::new_from_str(""));
        assert_eq!(Percent, T::new_from_str("Percent"));
        assert_eq!(Length, T::new_from_str("Length"));
        assert_eq!(Mass, T::new_from_str("Mass"));
        assert_eq!(Time, T::new_from_str("Time"));
        assert_eq!(Temperature, T::new_from_str("Temperature"));
        assert_eq!(LuminousIntensity, T::new_from_str("LuminousIntensity"));
        assert_eq!(Angle, T::new_from_str("Angle"));
        assert_eq!(Force, T::new_from_str("Force"));
        assert_eq!(Frequency, T::new_from_str("Frequency"));
        assert_eq!(Current, T::new_from_str("Current"));
        assert_eq!(Voltage, T::new_from_str("Voltage"));
        assert_eq!(Power, T::new_from_str("Power"));
        assert_eq!(Energy, T::new_from_str("Energy"));
        assert_eq!(Area, T::new_from_str("Area"));
        assert_eq!(Volume, T::new_from_str("Volume"));
        assert_eq!(Speed, T::new_from_str("Speed"));
        assert_eq!(Acceleration, T::new_from_str("Acceleration"));
        assert_eq!(AngularSpeed, T::new_from_str("AngularSpeed"));
        assert_eq!(AngularAccc, T::new_from_str("AngularAccc"));
        assert_eq!(WaveLength, T::new_from_str("WaveLength"));
        assert_eq!(ColorComponent, T::new_from_str("ColorComponent"));
    }
}
