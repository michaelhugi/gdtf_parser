use std::borrow::Borrow;
use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deparse::{DeparseSingle, DeparseVec};
use crate::errors::GdtfError;
use crate::units::name::Name;

#[derive(Debug)]
pub struct ActivationGroup {
    pub name: Name,
}


impl DeparseSingle for ActivationGroup {
    fn single_from_event_unchecked(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => {
                    return Ok(ActivationGroup {
                        name: std::str::from_utf8(attr.value.borrow())?.try_into()?
                    });
                }
                _ => {}
            }
        }
        Err(GdtfError::RequiredValueNotFoundError("Name not found in ActivationGroup".to_string()))
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

    use crate::deparse::DeparseSingle;
    use crate::gdtf::fixture_type::attribute_definitions::activation_group::ActivationGroup;

    #[test]
    fn test_activation_group() {
        ActivationGroup {
            name: "PanTilt".try_into().unwrap()
        }.test(
            r#"<ActivationGroup Name="PanTilt"/>"#
        );
    }
}
