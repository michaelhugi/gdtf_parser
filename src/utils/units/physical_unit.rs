//! Module for the unit PhysicalUnit used in GDTF

use quick_xml::events::attributes::Attribute;

use crate::utils::read;

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

    ///Creates a new Physical unit from an xml-attribute defined in gdtf-xml.
    /// ## Examples
    /// ```rust
    /// use gdtf_parser::utils::units::physical_unit::PhysicalUnit;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    ///
    /// assert_eq!(PhysicalUnit::Time, PhysicalUnit::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Time")}));
    /// assert_eq!(PhysicalUnit::ColorComponent, PhysicalUnit::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"ColorComponent")}));
    /// assert_eq!(PhysicalUnit::None, PhysicalUnit::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"None")}));
    /// assert_eq!(PhysicalUnit::None, PhysicalUnit::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Something else")}));
    /// ```
    pub fn new_from_attr(attr: Attribute) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
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

    #[test]
    fn test_new_from_attr_owned() {
        use T::*;
        assert_eq!(None, T::new_from_attr(testdata::to_attr_owned(b"None")));
        assert_eq!(
            None,
            T::new_from_attr(testdata::to_attr_owned(b"Something else"))
        );
        assert_eq!(None, T::new_from_attr(testdata::to_attr_owned(b"")));
        assert_eq!(
            Percent,
            T::new_from_attr(testdata::to_attr_owned(b"Percent"))
        );
        assert_eq!(Length, T::new_from_attr(testdata::to_attr_owned(b"Length")));
        assert_eq!(Mass, T::new_from_attr(testdata::to_attr_owned(b"Mass")));
        assert_eq!(Time, T::new_from_attr(testdata::to_attr_owned(b"Time")));
        assert_eq!(
            Temperature,
            T::new_from_attr(testdata::to_attr_owned(b"Temperature"))
        );
        assert_eq!(
            LuminousIntensity,
            T::new_from_attr(testdata::to_attr_owned(b"LuminousIntensity"))
        );
        assert_eq!(Angle, T::new_from_attr(testdata::to_attr_owned(b"Angle")));
        assert_eq!(Force, T::new_from_attr(testdata::to_attr_owned(b"Force")));
        assert_eq!(
            Frequency,
            T::new_from_attr(testdata::to_attr_owned(b"Frequency"))
        );
        assert_eq!(
            Current,
            T::new_from_attr(testdata::to_attr_owned(b"Current"))
        );
        assert_eq!(
            Voltage,
            T::new_from_attr(testdata::to_attr_owned(b"Voltage"))
        );
        assert_eq!(Power, T::new_from_attr(testdata::to_attr_owned(b"Power")));
        assert_eq!(Energy, T::new_from_attr(testdata::to_attr_owned(b"Energy")));
        assert_eq!(Area, T::new_from_attr(testdata::to_attr_owned(b"Area")));
        assert_eq!(Volume, T::new_from_attr(testdata::to_attr_owned(b"Volume")));
        assert_eq!(Speed, T::new_from_attr(testdata::to_attr_owned(b"Speed")));
        assert_eq!(
            Acceleration,
            T::new_from_attr(testdata::to_attr_owned(b"Acceleration"))
        );
        assert_eq!(
            AngularSpeed,
            T::new_from_attr(testdata::to_attr_owned(b"AngularSpeed"))
        );
        assert_eq!(
            AngularAccc,
            T::new_from_attr(testdata::to_attr_owned(b"AngularAccc"))
        );
        assert_eq!(
            WaveLength,
            T::new_from_attr(testdata::to_attr_owned(b"WaveLength"))
        );
        assert_eq!(
            ColorComponent,
            T::new_from_attr(testdata::to_attr_owned(b"ColorComponent"))
        );
    }

    #[test]
    fn test_new_from_attr_borrowed() {
        use T::*;
        assert_eq!(None, T::new_from_attr(testdata::to_attr_borrowed(b"None")));
        assert_eq!(
            None,
            T::new_from_attr(testdata::to_attr_borrowed(b"Something else"))
        );
        assert_eq!(None, T::new_from_attr(testdata::to_attr_borrowed(b"")));
        assert_eq!(
            Percent,
            T::new_from_attr(testdata::to_attr_borrowed(b"Percent"))
        );
        assert_eq!(
            Length,
            T::new_from_attr(testdata::to_attr_borrowed(b"Length"))
        );
        assert_eq!(Mass, T::new_from_attr(testdata::to_attr_borrowed(b"Mass")));
        assert_eq!(Time, T::new_from_attr(testdata::to_attr_borrowed(b"Time")));
        assert_eq!(
            Temperature,
            T::new_from_attr(testdata::to_attr_borrowed(b"Temperature"))
        );
        assert_eq!(
            LuminousIntensity,
            T::new_from_attr(testdata::to_attr_borrowed(b"LuminousIntensity"))
        );
        assert_eq!(
            Angle,
            T::new_from_attr(testdata::to_attr_borrowed(b"Angle"))
        );
        assert_eq!(
            Force,
            T::new_from_attr(testdata::to_attr_borrowed(b"Force"))
        );
        assert_eq!(
            Frequency,
            T::new_from_attr(testdata::to_attr_borrowed(b"Frequency"))
        );
        assert_eq!(
            Current,
            T::new_from_attr(testdata::to_attr_borrowed(b"Current"))
        );
        assert_eq!(
            Voltage,
            T::new_from_attr(testdata::to_attr_borrowed(b"Voltage"))
        );
        assert_eq!(
            Power,
            T::new_from_attr(testdata::to_attr_borrowed(b"Power"))
        );
        assert_eq!(
            Energy,
            T::new_from_attr(testdata::to_attr_borrowed(b"Energy"))
        );
        assert_eq!(Area, T::new_from_attr(testdata::to_attr_borrowed(b"Area")));
        assert_eq!(
            Volume,
            T::new_from_attr(testdata::to_attr_borrowed(b"Volume"))
        );
        assert_eq!(
            Speed,
            T::new_from_attr(testdata::to_attr_borrowed(b"Speed"))
        );
        assert_eq!(
            Acceleration,
            T::new_from_attr(testdata::to_attr_borrowed(b"Acceleration"))
        );
        assert_eq!(
            AngularSpeed,
            T::new_from_attr(testdata::to_attr_borrowed(b"AngularSpeed"))
        );
        assert_eq!(
            AngularAccc,
            T::new_from_attr(testdata::to_attr_borrowed(b"AngularAccc"))
        );
        assert_eq!(
            WaveLength,
            T::new_from_attr(testdata::to_attr_borrowed(b"WaveLength"))
        );
        assert_eq!(
            ColorComponent,
            T::new_from_attr(testdata::to_attr_borrowed(b"ColorComponent"))
        );
    }
}
