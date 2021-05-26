use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;

use zip::result::ZipError;

use crate::utils::units::dmx_value::GDTFDmxValueError;
use crate::utils::units::guid::GDTFGUIDError;
use crate::utils::units::name::GDTFNameError;

#[derive(Debug)]
pub enum GdtfError {
    Utf8Error(Utf8Error),
    QuickXMLError(quick_xml::Error),
    RequiredValueNotFoundError(String),
    ColorCIENotValidError(String),
    DateNotValidError(String),
    DMXAddressNotValidError(String),
    FileReadError(std::io::Error),
    ZipError(ZipError),
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    NameError(GDTFNameError),
    GUIDError(GDTFGUIDError),
    GDTFDmxValueError(GDTFDmxValueError),
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

impl From<GDTFNameError> for GdtfError {
    fn from(e: GDTFNameError) -> Self {
        GdtfError::NameError(e)
    }
}

impl From<GDTFDmxValueError> for GdtfError {
    fn from(e: GDTFDmxValueError) -> Self {
        GdtfError::GDTFDmxValueError(e)
    }
}

impl fmt::Display for GdtfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went terribly wrong")
    }
}

impl From<GDTFGUIDError> for GdtfError {
    fn from(e: GDTFGUIDError) -> Self {
        GdtfError::GUIDError(e)
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
        GdtfError::QuickXMLError(e)
    }
}

impl From<std::io::Error> for GdtfError {
    fn from(e: std::io::Error) -> Self {
        GdtfError::FileReadError(e)
    }
}

impl Error for GdtfError {}
