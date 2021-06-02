use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparseSingle;
#[cfg(test)]
use crate::utils::deparse::TestDeparseSingle;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

#[derive(Debug, PartialEq, Clone)]
pub struct Feature {
    pub name: Name,
}


impl DeparseSingle for Feature {
    type PrimaryKey = ();

    fn single_from_event(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    return Ok((Feature { name: attr.into() }, None));
                }
                _ => {}
            }
        }
        return Ok((Feature {
            name: Default::default()
        }, None));
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Feature"
    }

    fn single_event_name() -> String {
        "Feature".to_string()
    }
}

#[cfg(test)]
impl TestDeparseSingle for Feature {}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::feature_group::feature::Feature;
    use crate::utils::deparse::TestDeparseSingle;

    #[test]
    fn test_feature() {
        Feature {
            name: "PanTilt".try_into().unwrap()
        }.test(None,
               r#"<Feature Name="PanTilt"/>"#,
        );
    }

    #[test]
    fn test_feature_empty() {
        Feature {
            name: "".try_into().unwrap()
        }.test(None,
               r#"<Feature Name=""/>"#,
        );
    }

    #[test]
    fn test_feature_min() {
        Feature {
            name: "".try_into().unwrap(),
        }.test(None,
               r#"<Feature/>"#,
        );
    }

    #[test]
    fn test_feature_faulty() {
        assert!(Feature::single_from_xml(r#"Feature Name="PanTilt"/>"#).is_err())
    }
}
