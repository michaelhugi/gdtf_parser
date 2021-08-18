//!Defines the description of the filter
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::measurement::Measurement;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::color_cie::ColorCie;
use crate::utils::units::name::Name;

///Defines the description of the filter
#[derive(Debug, PartialEq, Clone)]
pub struct Filter {
    ///Approximate absolute color point when this filter is the only item fully inserted into the beam and the fixture is at maximum intensity. For Y give relative value compared to overall output defined in property Luminous Flux of related Beam Geometry (transmissive case).
    pub color: ColorCie,
    ///The measurement defines the relation between the requested output by a control channel and the physically achieved intensity
    pub measurements: Vec<Measurement>,
}

///Helper Data Holder for reading Filter
#[derive(Default)]
pub(crate) struct FilterDataHolder {
    ///Approximate absolute color point when this filter is the only item fully inserted into the beam and the fixture is at maximum intensity. For Y give relative value compared to overall output defined in property Luminous Flux of related Beam Geometry (transmissive case).
    pub color: Option<ColorCie>,
    ///The measurement defines the relation between the requested output by a control channel and the physically achieved intensity
    pub measurements: Vec<Measurement>,
}

impl ReadGdtf for Filter {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = FilterDataHolder;
    const NODE_NAME: &'static [u8] = b"Filter";
    const PARENT_NODE_NAME: &'static [u8] = b"Filters";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        if let b"Color" = attr.key {
            data_holder.color = Some(ColorCie::new_from_attr(attr)?);
        }
        Ok(())
    }

    fn read_any_child(data_holder: &mut Self::DataHolder, reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(), Self::Error> {
        if let Measurement::NODE_NAME = event.name() {
            data_holder.measurements.push(Measurement::read_single_from_event(reader, event, has_children)?.1)
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            color: data_holder.color.ok_or_else(|| Self::attribute_not_found(b"Color"))?,
            measurements: data_holder.measurements,
        })
    }

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}


#[cfg(test)]
impl TestReadGdtf for Filter {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name("magenta_flag".to_string())), Some(Filter { color: ColorCie { x: 0.384400, y: 0.158500, Y: 100.0 }, measurements: Measurement::testdata_vec() })),
            (Some(Name("yellow_flag".to_string())), Some(Filter { color: ColorCie { x: 0.431200, y: 0.507000, Y: 100.0 }, measurements: Measurement::testdata_vec() })),
            (Some(Name("cto_flag".to_string())), Some(Filter { color: ColorCie { x: 0.470600, y: 0.392300, Y: 100.0 }, measurements: vec![] })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            format!(r#"<Filter Color="0.384400,0.158500,100.000000" Name="magenta_flag">{}</Filter>"#, Measurement::testdata_xml()),
            format!(r#"<Filter Color="0.431200,0.507000,100.000000" Name="yellow_flag">{}</Filter>"#, Measurement::testdata_xml()),
            r#"<Filter Color="0.470600,0.392300,100.000000" Name="cto_flag"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![r#"<Filter Name="MyName"/>"#.to_string()]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::filters::Filter;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Filter::execute_tests()
    }
}