use std::convert::TryFrom;

use crate::errors::GdtfError;

///CIE color representation xyY 1931
#[derive(Debug)]
pub struct GUID {
    value: String,
}

impl GUID {
    pub fn new() -> GUID {
        GUID {
            value: String::new()
        }
    }
    pub fn is_empty(&self) -> bool {
        self.value == ""
    }
}


impl TryFrom<&str> for GUID {
    type Error = GdtfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(GUID { value: String::from(s) })
    }
}

impl PartialEq for GUID {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
