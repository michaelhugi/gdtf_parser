use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

use zip::result::ZipError;
use std::num::ParseIntError;

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
    ParseIntError(ParseIntError)
}

impl From<ParseIntError> for GdtfError{
    fn from(e: ParseIntError) -> Self {
        GdtfError::ParseIntError(e)
    }
}

impl fmt::Display for GdtfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went terribly wrong")
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
