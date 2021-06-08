//! # Definition of Feature
//! groups the Fixture Type Attributes into a structured way for easier access and search
//!
use quick_xml::events::BytesStart;

use crate::utils::deparse::DeparsePrimaryKey;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub struct Feature {}


impl DeparsePrimaryKey for Feature {
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


    const NODE_NAME: &'static [u8] = b"Feature";
    type PrimaryKey = Name;
    const PARENT_NODE_NAME: &'static [u8] = &[];
}
