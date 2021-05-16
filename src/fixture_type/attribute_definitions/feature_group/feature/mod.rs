use std::borrow::Borrow;
use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

#[derive(Debug)]
pub struct Feature {
    pub name: Name,
}


impl DeparseSingle for Feature {
    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name == compare.name
    }
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
        return Ok(Feature {
            name: "".try_into()?
        });
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Feature"
    }

    fn single_event_name() -> String {
        "Feature".to_string()
    }
    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::utils::deparse::DeparseSingle;
    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;

    #[test]
    fn test_feature() {
        Feature {
            name: "PanTilt".try_into().unwrap()
        }.test(
            r#"<Feature Name="PanTilt"/>"#
        );
    }

    #[test]
    fn test_feature_empty() {
        Feature {
            name: "".try_into().unwrap()
        }.test(
            r#"<Feature Name=""/>"#
        );
    }

    #[test]
    fn test_feature_min() {
        Feature {
            name: "".try_into().unwrap()
        }.test(
            r#"<Feature/>"#
        );
    }

    #[test]
    fn test_feature_faulty() {
        match Feature::single_from_xml(r#"Feature Name="PanTilt"/>"#) {
            Ok(_) => { panic!("test_feature_faulty should return an error"); }
            Err(_) => {}
        }
    }
}
