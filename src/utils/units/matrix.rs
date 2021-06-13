//! Module for the unit Matrix used in GDTF

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

use lazy_static::lazy_static;
use quick_xml::events::attributes::Attribute;
use regex::Regex;

use crate::utils::read;

///The transformation matrix consists 4 x 4 floats. Stored in a row-major order
///The matrix rotation is stored in the first three columns, and the translation is stored in the 4th column. The metric system consists of the Right-handed Cartesian Coordinates XYZ
#[derive(Debug, PartialEq)]
pub struct Matrix(pub [[f32; 4]; 4]);


impl Matrix {
    ///Parses a string defined in gdtf-xml-description to Matrix
    /// ```rust
    /// use gdtf_parser::utils::units::matrix::Matrix;
    ///
    /// assert_eq!(
    ///             Matrix::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}").unwrap(),
    ///             Matrix([
    ///                 [1.1, 1.2, 1.3, 1.4],
    ///                 [2.1, 2.2, 2.3, 2.4],
    ///                 [3.1, 3.2, 3.3, 3.4],
    ///                 [4.1, 4.2, 4.3, 4.4],
    ///             ])
    ///         );
    /// assert!(Matrix::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3}").is_err());
    /// assert!(Matrix::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}{5.4}").is_err());
    /// ```
    pub fn new_from_str(s: &str) -> Result<Self, GdtfMatrixError> {
        use GdtfMatrixError as E;
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"[-+]?([0-9]*\.[0-9]+|[0-9]+)").unwrap();
        }
        let mut caps = REGEX.captures_iter(s);

        let m11 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m12 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m13 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m14 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        let m21 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m22 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m23 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m24 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        let m31 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m32 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m33 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m34 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        let m41 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m42 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m43 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;
        let m44 = f32::from_str(&caps.next().ok_or(E::WrongFormatError)?[0])?;

        if caps.next().is_some() {
            return Err(GdtfMatrixError::WrongFormatError);
        }

        Ok(Self([
            [m11, m12, m13, m14],
            [m21, m22, m23, m24],
            [m31, m32, m33, m34],
            [m41, m42, m43, m44]
        ]
        ))
    }

    ///Parses a quick-xml-attribute defined in gdtf-xml-description to Matrix
    /// ```rust
    /// use gdtf_parser::utils::units::matrix::Matrix;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    ///
    /// assert_eq!(
    ///             Matrix::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}")}).unwrap(),
    ///             Matrix([
    ///                 [1.1, 1.2, 1.3, 1.4],
    ///                 [2.1, 2.2, 2.3, 2.4],
    ///                 [3.1, 3.2, 3.3, 3.4],
    ///                 [4.1, 4.2, 4.3, 4.4],
    ///             ])
    ///         );
    /// assert!(Matrix::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}{5.1}")}).is_err());
    /// assert!(Matrix::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3}")}).is_err());
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Result<Self, GdtfMatrixError> {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

#[derive(Debug)]
pub enum GdtfMatrixError {
    ParseFloatError(ParseFloatError),
    WrongFormatError,
}

impl Display for GdtfMatrixError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GdtfMatrixError::ParseFloatError(e) => write!(f, "Matrix must be formatted in {{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}. {}", e),
            GdtfMatrixError::WrongFormatError => write!(f, "Matrix must be formatted in {{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}{{float,float,float,float}}")
        }
    }
}

impl Error for GdtfMatrixError {}

impl From<ParseFloatError> for GdtfMatrixError {
    fn from(e: ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::testdata;
    use crate::utils::units::matrix::{GdtfMatrixError, Matrix};

    #[test]
    fn test_new_from_str() -> Result<(), GdtfMatrixError> {
        assert_eq!(
            Matrix::new_from_str("{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}").unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert_eq!(
            Matrix::new_from_str("{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}").unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert!(Matrix::new_from_str("{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3}").is_err());
        assert!(Matrix::new_from_str("{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}5.4").is_err());
        Ok(())
    }

    #[test]
    fn test_new_attr_borrowed() -> Result<(), GdtfMatrixError> {
        assert_eq!(
            Matrix::new_from_attr(testdata::to_attr_borrowed(b"{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}")).unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert_eq!(
            Matrix::new_from_attr(testdata::to_attr_borrowed(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}")).unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert!(Matrix::new_from_attr(testdata::to_attr_borrowed(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3}")).is_err());
        assert!(Matrix::new_from_attr(testdata::to_attr_borrowed(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}5.4")).is_err());
        Ok(())
    }

    #[test]
    fn test_new_attr_owned() -> Result<(), GdtfMatrixError> {
        assert_eq!(
            Matrix::new_from_attr(testdata::to_attr_owned(b"{1.1,1.2,1.3,1.4}{2.1,2.2,2.3,2.4}{3.1,3.2,3.3,3.4}{4.1,4.2,4.3,4.4}")).unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert_eq!(
            Matrix::new_from_attr(testdata::to_attr_owned(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}")).unwrap(),
            Matrix([
                [1.1, 1.2, 1.3, 1.4],
                [2.1, 2.2, 2.3, 2.4],
                [3.1, 3.2, 3.3, 3.4],
                [4.1, 4.2, 4.3, 4.4],
            ])
        );
        assert!(Matrix::new_from_attr(testdata::to_attr_owned(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3}")).is_err());
        assert!(Matrix::new_from_attr(testdata::to_attr_owned(b"{1.1,1.2,1.3,1.4},{2.1,2.2,2.3,2.4},{3.1,3.2,3.3,3.4},{4.1,4.2,4.3,4.4}5.4")).is_err());
        Ok(())
    }
}