//! # Definition of ActivationGroup
//! Attributes which need to be activated together to gain control over one logical function
//! Note 1 to entry: As example Pan & Tilt are paired to gain control over Position.

use quick_xml::events::BytesStart;

use crate::utils::deparse::DeparsePrimaryKey;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub struct ActivationGroup {}

impl DeparsePrimaryKey<Name> for ActivationGroup {
    type Error = GdtfError;

    fn read_primary_key_from_event(event: BytesStart<'_>) -> Result<Name, GdtfError> {
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == b"Name" {
                return Ok(Name::new_from_attr(attr)?);
            }
        }
        Ok(Default::default())
    }

    const NODE_NAME: &'static [u8] = b"ActivationGroup";
}
