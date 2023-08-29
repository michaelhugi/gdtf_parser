//!The measurement defines the relation between the requested output by a control channel and the physically achieved intensity

use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::physical_descriptions::measurement::measurement_point::MeasurementPoint;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

pub mod measurement_point;

///The measurement defines the relation between the requested output by a control channel and the physically achieved intensity
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Measurement {
    ///For additive color mixing: uniquely given emitter intensity DMX percentage. Value range between > 0 and ≤ 100.
    ///For subtractive color mixing: uniquely given flag insertion DMX percentage. Value range between 0 and 100.
    pub physical: f32,
    ///Used for additive color mixing: overall candela value for the enclosed set of measurements
    pub luminous_intensity: f32,
    ///Used for subtractive color mixing: total amount of lighting energy passed at this insertion percentage.
    pub transmission: f32,
    ///Interpolation scheme from the previous value. The currently defined values are: "Linear", "Step", "Log"; Default: Linear
    pub interpolation_to: InterpolationTo,
    ///The measurement point defines the energy of a specific wavelength of a spectrum
    pub measurement_points: Vec<MeasurementPoint>,
}

impl ReadGdtf for Measurement {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = Measurement;
    const NODE_NAME: &'static [u8] = b"Measurement";
    const PARENT_NODE_NAME: &'static [u8] = b"Measurements";
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Physical" => data_holder.physical = read::attr_to_f32(attr),
            b"LuminousIntensity" => data_holder.luminous_intensity = read::attr_to_f32(attr),
            b"Transmission" => data_holder.transmission = read::attr_to_f32(attr),
            b"InterpolationTo" => {
                data_holder.interpolation_to = InterpolationTo::new_from_attr(attr)
            }
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        if let MeasurementPoint::NODE_NAME = event.name() {
            data_holder
                .measurement_points
                .push(MeasurementPoint::read_single_from_event(reader, event, has_children)?.1)
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for Measurement {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Self {
                    physical: 100.0,
                    luminous_intensity: 0.0,
                    transmission: 1.0,
                    interpolation_to: InterpolationTo::Linear,
                    measurement_points: vec![],
                }),
            ),
            (
                None,
                Some(Self {
                    physical: 76.000001,
                    luminous_intensity: 0.0,
                    transmission: 1.0,
                    interpolation_to: InterpolationTo::Linear,
                    measurement_points: vec![],
                }),
            ),
            (
                None,
                Some(Self {
                    physical: 100.0,
                    luminous_intensity: 0.0,
                    transmission: 76.000000,
                    interpolation_to: InterpolationTo::Step,
                    measurement_points: MeasurementPoint::testdata_vec(),
                }),
            ),
            (
                None,
                Some(Self {
                    physical: 0.0,
                    luminous_intensity: 300000.000000,
                    transmission: 0.0,
                    interpolation_to: InterpolationTo::Log,
                    measurement_points: vec![],
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Measurement InterpolationTo="Linear" Physical="100.000000" Transmission="1.000000"/>"#.to_string(),
            r#"<Measurement Physical="76.000001" Transmission="1.000000"/>"#.to_string(),
            format!(r#"<Measurement InterpolationTo="Step" Physical="100.000000" Transmission="76.000000">{}</Measurement>"#, MeasurementPoint::testdata_xml()),
            r#"<Measurement InterpolationTo="Log" LuminousIntensity="300000.000000"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of InterpolationTo
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///Interpolation scheme from the previous value.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum InterpolationTo {
    Linear,
    Step,
    Log,
}

impl InterpolationTo {
    /// Parses a str provided by gdtf-xml-description to InterpolationTo
    /// ```rust
    /// use gdtf_parser::fixture_type::physical_descriptions::measurement::InterpolationTo;
    /// assert_eq!(InterpolationTo::Linear, InterpolationTo::new_from_str("Linear"));
    /// assert_eq!(InterpolationTo::Log, InterpolationTo::new_from_str("Log"));
    /// assert_eq!(InterpolationTo::Step, InterpolationTo::new_from_str("Step"));
    /// assert_eq!(InterpolationTo::Linear, InterpolationTo::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(value: &str) -> Self {
        match value {
            "Step" => Self::Step,
            "Log" => Self::Log,
            _ => Self::Linear,
        }
    }

    /// Parses a quick-xml-attribute provided by gdtf-xml-description to InterpolationTo
    /// ```rust
    /// use gdtf_parser::fixture_type::physical_descriptions::measurement::InterpolationTo;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(InterpolationTo::Linear, InterpolationTo::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Linear")}));
    /// assert_eq!(InterpolationTo::Log, InterpolationTo::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Log")}));
    /// assert_eq!(InterpolationTo::Step, InterpolationTo::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Step")}));
    /// assert_eq!(InterpolationTo::Linear, InterpolationTo::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Anything else")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

///```rust
/// use gdtf_parser::fixture_type::physical_descriptions::measurement::InterpolationTo;
/// assert_eq!(InterpolationTo::Linear, Default::default())
/// ```
impl Default for InterpolationTo {
    fn default() -> Self {
        Self::Linear
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of InterpolationTo
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::measurement::{InterpolationTo, Measurement};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::testdata;

    #[test]
    fn test_deparse() {
        Measurement::execute_tests()
    }

    #[test]
    fn test_interpolation_to_new_from_str() {
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_str("Linear")
        );
        assert_eq!(InterpolationTo::Log, InterpolationTo::new_from_str("Log"));
        assert_eq!(InterpolationTo::Step, InterpolationTo::new_from_str("Step"));
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_str("Anything else")
        );
    }

    #[test]
    fn test_interpolation_to_new_from_attr_owned() {
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_attr(testdata::to_attr_owned(b"Linear"))
        );
        assert_eq!(
            InterpolationTo::Log,
            InterpolationTo::new_from_attr(testdata::to_attr_owned(b"Log"))
        );
        assert_eq!(
            InterpolationTo::Step,
            InterpolationTo::new_from_attr(testdata::to_attr_owned(b"Step"))
        );
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_attr(testdata::to_attr_owned(b"Anything else"))
        );
    }

    #[test]
    fn test_interpolation_to_new_from_attr_borrowed() {
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_attr(testdata::to_attr_borrowed(b"Linear"))
        );
        assert_eq!(
            InterpolationTo::Log,
            InterpolationTo::new_from_attr(testdata::to_attr_borrowed(b"Log"))
        );
        assert_eq!(
            InterpolationTo::Step,
            InterpolationTo::new_from_attr(testdata::to_attr_borrowed(b"Step"))
        );
        assert_eq!(
            InterpolationTo::Linear,
            InterpolationTo::new_from_attr(testdata::to_attr_borrowed(b"Anything else"))
        );
    }
}
