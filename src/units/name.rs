
use std::convert::TryFrom;

use crate::errors::GdtfError;

///CIE color representation xyY 1931
#[derive(Debug)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new() -> Name {
        Name {
            name: String::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.name == ""
    }
}


impl TryFrom<&str> for Name {
    type Error = GdtfError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(Name { name: String::from(s) })
    }
}
#[cfg(test)]
impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
