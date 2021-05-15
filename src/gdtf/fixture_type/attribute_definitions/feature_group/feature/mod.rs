use std::borrow::Borrow;
use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deparse::DeparseSingle;
use crate::errors::GdtfError;
use crate::units::name::Name;

#[derive(Debug)]
pub struct Feature {
    pub name: Name,
}


impl DeparseSingle for Feature {
    fn single_from_event_unchecked(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    return Ok(Feature {
                        name: std::str::from_utf8(attr.value.borrow())?.try_into()?
                    });
                }
                _ => {}
            }
        }
        Err(GdtfError::RequiredValueNotFoundError("Name not found in Feature".to_string()))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Feature"
    }

    fn single_event_name() -> String {
        "Feature".to_string()
    }
    #[cfg(test)]
    fn is_single_eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::deparse::DeparseSingle;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;

    #[test]
    fn test_feature() {
        Feature {
            name: "PanTilt".try_into().unwrap()
        }.test(
            r#"<Feature Name="PanTilt"/>"#
        );
    }
}
