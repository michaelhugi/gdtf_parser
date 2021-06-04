use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::DeparsePrimaryKey;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub struct ActivationGroup {}

impl DeparsePrimaryKey<Name> for ActivationGroup {
    fn primary_key_from_event(_: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Name, GdtfError> {
        for attr in e.attributes().into_iter() {
            let attr = attr?;
            if attr.key == b"Name" {
                return Ok(Name::new_from_attr(attr)?);
            }
        }
        Ok(Default::default())
    }


    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"ActivationGroup"
    }
}
