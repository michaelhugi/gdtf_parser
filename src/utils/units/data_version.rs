use std::fmt::{format, Formatter};

use serde::{Deserialize, Deserializer};
use serde::de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor};

use crate::GdtfError;

#[derive(Debug, PartialEq, Clone)]
pub enum DataVersion {
    Version1_0,
    Version1_1,
}

impl DataVersion {
    fn major(&self) -> u8 {
        match self {
            Self::Version1_0 => 1,
            Self::Version1_1 => 1,
        }
    }
    fn minor(&self) -> u8 {
        match self {
            Self::Version1_0 => 0,
            Self::Version1_1 => 1,
        }
    }
}
