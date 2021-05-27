use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::test::assert_eq_allow_empty::AssertEqAllowEmpty;
use crate::utils::units::name::Name;

#[derive(Debug)]
pub struct Feature {
    pub name: Name,
}


impl DeparseSingle for Feature {
    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty_no_log(&compare.name)
    }
    fn single_from_event(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    return Ok(Feature { name: attr.into() });
                }
                _ => {}
            }
        }
        return Ok(Feature {
            name: Default::default()
        });
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Feature"
    }

    fn single_event_name() -> String {
        "Feature".to_string()
    }
}

impl AssertEqAllowEmpty for Feature {
    fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
        self.name.is_eq_allow_empty(&other.name)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::utils::deparse::DeparseSingle;

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
