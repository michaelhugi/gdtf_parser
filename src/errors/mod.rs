use std::error::Error;
use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum GdtfError {
    Utf8Error(Utf8Error),
    QuickXMLError(quick_xml::Error),
    RequiredValueNotFoundError(String),
    ColorCIENotValidError(String)
}


impl fmt::Display for GdtfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went terribly wrong")
    }
}

impl From<Utf8Error> for GdtfError {
    fn from(e: Utf8Error) -> Self {
        GdtfError::Utf8Error(e)
    }
}

impl From<quick_xml::Error> for GdtfError{
    fn from(e: quick_xml::Error) -> Self {
        GdtfError::QuickXMLError(e)
    }
}

impl Error for GdtfError {}
