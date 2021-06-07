use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

use zip::result::ZipError;

use crate::utils::units::color_cie::GdtfColorCieError;
use crate::utils::units::dmx_value::GdtfDmxValueError;
use crate::utils::units::fixture_type_guid::GdtfGuidError;
use crate::utils::units::name::GdtfNameError;
use crate::utils::units::node::GdtfNodeError;

#[derive(Debug)]
pub enum GdtfError {
    Utf8Error(Utf8Error),
    QuickXmlError(quick_xml::Error),
    RequiredValueNotFoundError(String),
    ColorCieNotValidError(String),
    DateNotValidError(String),
    DmxAddressNotValidError(String),
    FileReadError(std::io::Error),
    ZipError(ZipError),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    NameError(GdtfNameError),
    GuidError(GdtfGuidError),
    GdtfDmxValueError(GdtfDmxValueError),
    GdtfNodeError(GdtfNodeError),
    GdtfColorCieError(GdtfColorCieError),
}

impl From<GdtfColorCieError> for GdtfError {
    fn from(e: GdtfColorCieError) -> Self {
        GdtfError::GdtfColorCieError(e)
    }
}

impl From<ParseIntError> for GdtfError {
    fn from(e: ParseIntError) -> Self {
        GdtfError::ParseIntError(e)
    }
}

impl From<ParseFloatError> for GdtfError {
    fn from(e: ParseFloatError) -> Self {
        GdtfError::ParseFloatError(e)
    }
}

impl From<GdtfNameError> for GdtfError {
    fn from(e: GdtfNameError) -> Self {
        GdtfError::NameError(e)
    }
}

impl From<GdtfDmxValueError> for GdtfError {
    fn from(e: GdtfDmxValueError) -> Self {
        GdtfError::GdtfDmxValueError(e)
    }
}

impl From<GdtfNodeError> for GdtfError {
    fn from(e: GdtfNodeError) -> Self { GdtfError::GdtfNodeError(e) }
}

impl fmt::Display for GdtfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went terribly wrong")
    }
}

impl From<GdtfGuidError> for GdtfError {
    fn from(e: GdtfGuidError) -> Self {
        GdtfError::GuidError(e)
    }
}

impl From<ZipError> for GdtfError {
    fn from(e: ZipError) -> Self {
        GdtfError::ZipError(e)
    }
}

impl From<Utf8Error> for GdtfError {
    fn from(e: Utf8Error) -> Self {
        GdtfError::Utf8Error(e)
    }
}

impl From<quick_xml::Error> for GdtfError {
    fn from(e: quick_xml::Error) -> Self {
        GdtfError::QuickXmlError(e)
    }
}

impl From<std::io::Error> for GdtfError {
    fn from(e: std::io::Error) -> Self {
        GdtfError::FileReadError(e)
    }
}

impl Error for GdtfError {}
