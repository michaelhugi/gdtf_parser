use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::{DeparseSingle, DeparseVec};
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;
use crate::utils::deparse;

#[derive(Debug)]
pub struct ActivationGroup {
    pub name: Name,
}


impl DeparseSingle for ActivationGroup {
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
                    return Ok(ActivationGroup {
                        name: deparse::attr_try_to_name(&attr)?
                    });
                }
                _ => {}
            }
        }
        return Ok(ActivationGroup {
            name: "".try_into()?
        });
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"ActivationGroup"
    }

    fn single_event_name() -> String {
        "ActivationGroup".to_string()
    }
    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl DeparseVec for ActivationGroup {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"ActivationGroups"
    }

    fn group_event_name() -> String {
        "ActivationGroups".to_string()
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup;
    use crate::utils::deparse::DeparseSingle;

    #[test]
    fn test_activation_group() {
        ActivationGroup {
            name: "PanTilt".try_into().unwrap()
        }.test(
            r#"<ActivationGroup Name="PanTilt"/>"#
        );
    }

    #[test]
    fn test_activation_group_min() {
        ActivationGroup {
            name: "".try_into().unwrap()
        }.test(
            r#"<ActivationGroup Name=""/>"#
        );
    }

    #[test]
    fn test_activation_group_empty() {
        ActivationGroup {
            name: "".try_into().unwrap()
        }.test(
            r#"<ActivationGroup/>"#
        );
    }

    #[test]
    fn test_activation_group_faulty() {
        match ActivationGroup::single_from_xml(r#"<ActivationGrou Name="Some Name""#) {
            Ok(_) => { panic!("test_activation-group_faulty should return an error"); }
            Err(_) => {}
        }
    }
}
