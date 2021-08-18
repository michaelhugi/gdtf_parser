//!The measurement point defines the energy of a specific wavelength of a spectrum
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::measurement::Measurement;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

///The measurement point defines the energy of a specific wavelength of a spectrum
#[derive(Debug, PartialEq, Clone, Default)]
pub struct MeasurementPoint {
    ///Center wavelength of measurement (nm).
    pub wave_length: f32,
    ///Lighting energy (W/m2/nm).
    pub energy: f32,
}

impl ReadGdtf for MeasurementPoint {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = MeasurementPoint;
    const NODE_NAME: &'static [u8] = b"MeasurementPoint";
    const PARENT_NODE_NAME: &'static [u8] = Measurement::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"WaveLength" => data_holder.wave_length = read::attr_to_f32(attr),
            b"Energy" => data_holder.energy = read::attr_to_f32(attr),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(_: &mut Self::DataHolder, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(_: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for MeasurementPoint {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(MeasurementPoint { wave_length: 0.3, energy: 1.2 })),
            (None, Some(MeasurementPoint { wave_length: 0.0, energy: 1.2 })),
            (None, Some(MeasurementPoint { wave_length: 0.3, energy: 12.0 })),
            (None, Some(MeasurementPoint { wave_length: 0.3, energy: 0.0 })),
            (None, Some(MeasurementPoint { wave_length: 0.0, energy: 12.0 })),
            (None, Some(MeasurementPoint { wave_length: 0.3, energy: 0.0 })),
            (None, Some(MeasurementPoint { wave_length: 0.0, energy: 0.0 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<MeasurementPoint WaveLength="0.3" Energy="1.2"></MeasurementPoint>"#.to_string(),
            r#"<MeasurementPoint WaveLength="0" Energy="1.2"></MeasurementPoint>"#.to_string(),
            r#"<MeasurementPoint WaveLength="0.3" Energy="12"></MeasurementPoint>"#.to_string(),
            r#"<MeasurementPoint WaveLength="0.3" Energy="0"/>"#.to_string(),
            r#"<MeasurementPoint Energy="12"></MeasurementPoint>"#.to_string(),
            r#"<MeasurementPoint WaveLength="0.3"></MeasurementPoint>"#.to_string(),
            r#"<MeasurementPoint/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::measurement::measurement_point::MeasurementPoint as T;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        T::execute_tests();
    }
}
