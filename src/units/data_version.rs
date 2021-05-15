use std::convert::TryFrom;

use crate::errors::GdtfError;

pub enum DataVersion {
    Version1_1
}

impl TryFrom<&str> for DataVersion {
    type Error = GdtfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.split('.');
        let major = value.next().ok_or(GdtfError::VersionNotValidError("First argument of version not valid"));
        let minor=value.next()

        todo!()
    }
}