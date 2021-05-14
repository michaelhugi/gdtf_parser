use std::borrow::Borrow;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deparse::Deparse;
use crate::errors::GdtfError;

#[derive(Debug)]
pub struct Feature {
    pub name: String,
}

fn new(name: String) -> Feature {
    Feature {
        name
    }
}

impl Deparse for Feature {
    fn from_event_unchecked(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    return Ok(new(std::str::from_utf8(attr.value.borrow())?.to_owned()));
                }
                _ => {}
            }
        }
        Err(GdtfError::RequiredValueNotFoundError("Name not found in Feature".to_string()))
    }

    fn is_event_name(event_name: &[u8]) -> bool {
        event_name == b"Feature"
    }

    fn event_name() -> String {
        "Feature".to_string()
    }
    #[cfg(test)]
    fn loose_eq_test(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


#[cfg(test)]
mod tests {
    use crate::deparse::Deparse;
    use crate::gdtf::fixture_type::attribute_definitions::feature_group::feature::Feature;

    #[test]
    fn test_feature() {
        Feature::from_xml(
            r#"<Feature Name="PanTilt"/>"#
        ).expect("Unexpected error in test feature group").test_eq(
            &Feature {
                name: "PanTilt".to_string(),
            });
    }
}
