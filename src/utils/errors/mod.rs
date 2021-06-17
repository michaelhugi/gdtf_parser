use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

use zip::result::ZipError;

use crate::utils::read::GdtfReadError;
use crate::utils::units::color_cie::GdtfColorCieError;
use crate::utils::units::dmx_value::GdtfDmxValueError;
use crate::utils::units::guid::GdtfGuidError;
use crate::utils::units::name::GdtfNameError;
use crate::utils::units::node::GdtfNodeError;
use crate::utils::units::pixel::GdtfPixelError;
use crate::utils::units::pixel_array::GdtfPixelArrayError;
use crate::utils::units::rotation::GdtfRotationError;

#[derive(Debug)]
pub enum GdtfError {
    Utf8Error(Utf8Error),
    QuickXmlError(quick_xml::Error),
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
    GdtfDeparseError(GdtfReadError),
    GdtfPixelArrayError(GdtfPixelArrayError),
    GdtfPixelError(GdtfPixelError),
    GdtfRotationError(GdtfRotationError),
}

impl From<GdtfRotationError> for GdtfError {
    fn from(e: GdtfRotationError) -> Self {
        GdtfError::GdtfRotationError(e)
    }
}

impl From<GdtfPixelArrayError> for GdtfError {
    fn from(e: GdtfPixelArrayError) -> Self {
        GdtfError::GdtfPixelArrayError(e)
    }
}

impl From<GdtfPixelError> for GdtfError {
    fn from(e: GdtfPixelError) -> Self {
        GdtfError::GdtfPixelError(e)
    }
}

impl From<GdtfReadError> for GdtfError {
    fn from(e: GdtfReadError) -> Self {
        GdtfError::GdtfDeparseError(e)
    }
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
        match self {
            GdtfError::Utf8Error(e) => write!(f, "GdtfError: {}", e),
            GdtfError::QuickXmlError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::ColorCieNotValidError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::DateNotValidError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::DmxAddressNotValidError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::FileReadError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::ZipError(e) => write!(f, "GdtfError: {:?}", e),
            GdtfError::ParseIntError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::ParseFloatError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::NameError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GuidError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfDmxValueError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfNodeError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfColorCieError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfDeparseError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfPixelArrayError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfPixelError(e) => write!(f, "GdtfError: {}", e),
            GdtfError::GdtfRotationError(e) => write!(f, "GdtfError: {}", e),
        }
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
