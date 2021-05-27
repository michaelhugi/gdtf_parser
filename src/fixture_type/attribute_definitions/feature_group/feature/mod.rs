use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
use crate::utils::errors::GdtfError;
#[cfg(test)]
use crate::utils::test::partial_eq_allow_empty::PartialEqAllowEmpty;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::units::name::Name;

#[derive(Debug)]
pub struct Feature {
    pub name: Name,
}


impl DeparseSingle for Feature {
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

#[cfg(test)]
impl PartialEqAllowEmpty for Feature {
    fn is_eq_allow_empty_no_log(&self, other: &Self) -> bool {
        self.name.is_eq_allow_empty(&other.name)
    }
}

#[cfg(test)]
impl TestDeparseSingle for Feature {
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name.is_eq_allow_empty_no_log(&compare.name)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::utils::deparse::TestDeparseSingle;

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
