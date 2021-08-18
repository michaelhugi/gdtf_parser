//!Defines the description of the emitter
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::measurement::Measurement;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::color_cie::ColorCie;
use crate::utils::units::name::Name;

///Defines the description of the emitter
#[derive(Debug, PartialEq, Clone, Default)]
pub struct Emitter {
    ///Approximate absolute color point if applicable. Omit for non-visible emitters (e.g., UV). For Y give relative value compared to overall output defined in property Luminous Flux of related Beam Geometry (transmissive case).
    pub color: Option<ColorCie>,
    ///Required if color is omitted, otherwise it is optional. Dominant wavelength of the LED.
    pub dominant_wave_length: Option<f32>,
    ///Manufacturer’s part number of the diode.
    pub diode_part: Option<String>,
    ///The measurement defines the relation between the requested output by a control channel and the physically achieved intensity
    pub measurements: Vec<Measurement>,
}

impl ReadGdtf for Emitter {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = Emitter;
    const NODE_NAME: &'static [u8] = b"Emitter";
    const PARENT_NODE_NAME: &'static [u8] = b"Emitters";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Color" => data_holder.color = Some(ColorCie::new_from_attr(attr)?),
            b"DominantWaveLength" => {
                data_holder.dominant_wave_length = Some(read::attr_to_f32(attr))
            }
            b"DiodePart" => match read::attr_to_str(&attr) {
                "" => {}
                part => data_holder.diode_part = Some(part.to_string()),
            },
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
        if let Measurement::NODE_NAME = event.name() {
            data_holder
                .measurements
                .push(Measurement::read_single_from_event(reader, event, has_children)?.1)
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        attr: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for Emitter {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                Some(Name("MyName".to_string())),
                Some(Emitter {
                    color: None,
                    dominant_wave_length: None,
                    diode_part: None,
                    measurements: vec![],
                }),
            ),
            (
                Some(Name("Light Source 6800 High Power".to_string())),
                Some(Emitter {
                    color: Some(ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 100.0,
                    }),
                    dominant_wave_length: Some(0.0),
                    diode_part: None,
                    measurements: Measurement::testdata_vec(),
                }),
            ),
            (
                Some(Name("LED Engine".to_string())),
                Some(Emitter {
                    color: Some(ColorCie {
                        x: 0.312700,
                        y: 0.329000,
                        Y: 100.0,
                    }),
                    dominant_wave_length: Some(12.000001),
                    diode_part: Some("MyDiodePart".to_string()),
                    measurements: Measurement::testdata_vec(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Emitter Name="MyName"/>"#.to_string(),
            format!(
                r#"<Emitter Color="0.312700,0.329000,100.000000" DiodePart="" DominantWaveLength="0.000000" Name="Light Source 6800 High Power">{}</Emitter>"#,
                Measurement::testdata_xml()
            ),
            format!(
                r#"<Emitter Color="0.312700,0.329000,100.000000" DiodePart="MyDiodePart" DominantWaveLength="12.000001" Name="LED Engine">{}</Emitter>"#,
                Measurement::testdata_xml()
            ),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::emitters::Emitter;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Emitter::execute_tests()
    }
}
