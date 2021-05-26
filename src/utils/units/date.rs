//! Module for the unit Date used in GDTF
use std::borrow::Borrow;
use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::{FromStr, Utf8Error};

use quick_xml::events::attributes::Attribute;

///Date representation used in GDTF
#[derive(Debug)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

/// Parses a date from format yyyy-mm-ddThh:mm:ss
impl TryFrom<&str> for Date {
    type Error = GDTFDateError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: Vec<&str> = value.split("T").collect();
        if value.len() != 2 {
            return Err(GDTFDateError {});
        }
        let date = value[0];
        let time = value[1];

        let date: Vec<&str> = date.split("-").collect();
        if date.len() != 3 {
            return Err(GDTFDateError {});
        }
        let time: Vec<&str> = time.split(":").collect();
        if time.len() != 3 {
            return Err(GDTFDateError {});
        }

        Ok(Date {
            year: u16::from_str(date[0])?,
            month: u8::from_str(date[1])?,
            day: u8::from_str(date[2])?,
            hour: u8::from_str(time[0])?,
            minute: u8::from_str(time[1])?,
            second: u8::from_str(time[2])?,
        })
    }
}

/// Parses a date from an xml attribute formatted yyyy-mm-ddThh:mm:ss
impl TryFrom<Attribute<'_>> for Date {
    type Error = GDTFDateError;

    fn try_from(attr: Attribute<'_>) -> Result<Self, Self::Error> {
        std::str::from_utf8(attr.value.borrow())?.try_into()
    }
}

#[cfg(test)]
impl PartialEq for Date {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year &&
            self.month == other.month &&
            self.day == other.day &&
            self.hour == other.hour &&
            self.minute == other.minute &&
            self.second == other.second
    }
}

///Displays date in format yyyy-mm-ddThh:mm:ss
impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}

#[derive(Debug)]
/// Error that occures if the format of Date is wrong e.q. not yyyy-mm-ddThh:mm:ss
pub struct GDTFDateError {}


impl Display for GDTFDateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "The Date must be formatted yyyy-mm-ddThh:mm:ss instance of T != 1")
    }
}

impl Error for GDTFDateError {}

impl From<ParseIntError> for GDTFDateError {
    fn from(_: ParseIntError) -> Self {
        GDTFDateError {}
    }
}

impl From<Utf8Error> for GDTFDateError {
    fn from(_: Utf8Error) -> Self {
        GDTFDateError {}
    }
}

#[cfg(test)]
mod tests {
    use std::convert::{TryFrom, TryInto};

    use crate::utils::testdata;
    use crate::utils::units::date::Date;

    #[test]
    fn test_try_from_str_valid() {
        assert_eq!(
            Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 },
            "2021-05-16T17:41:12".try_into().unwrap()
        );
        assert_eq!(
            Date { year: 2022, month: 2, day: 8, hour: 17, minute: 12, second: 12 },
            "2022-02-08T17:12:12".try_into().unwrap()
        );
    }

    #[test]
    fn test_try_from_str_invalid() {
        assert!(Date::try_from("2021-05/16T17:41:12").is_err());
        assert!(Date::try_from("something invalid").is_err());
        assert!(Date::try_from("2021-05-1617:41:12").is_err());
        assert!(Date::try_from("2021/05-12T17:41:12").is_err());
        assert!(Date::try_from("202105-16T17:41:12").is_err());
        assert!(Date::try_from("").is_err());
        assert!(Date::try_from("2021-05-16T17:4112").is_err());
        assert!(Date::try_from("2021-05-16T1741:12").is_err());
    }

    #[test]
    fn test_try_from_attr_owned_valid() {
        assert_eq!(
            Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 },
            testdata::to_attr_owned(b"2021-05-16T17:41:12").try_into().unwrap()
        );
        assert_eq!(
            Date { year: 2022, month: 2, day: 8, hour: 17, minute: 12, second: 12 },
            testdata::to_attr_owned(b"2022-02-08T17:12:12").try_into().unwrap()
        );
    }

    #[test]
    fn test_try_from_attr_owned_invalid() {
        assert!(Date::try_from(testdata::to_attr_owned(b"2021-05/16T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"something invalid")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"2021-05-1617:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"2021/05-12T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"202105-16T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"2021-05-16T17:4112")).is_err());
        assert!(Date::try_from(testdata::to_attr_owned(b"2021-05-16T1741:12")).is_err());
    }

    #[test]
    fn test_try_from_attr_borrowed_valid() {
        assert_eq!(
            Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 },
            testdata::to_attr_borrowed(b"2021-05-16T17:41:12").try_into().unwrap()
        );
        assert_eq!(
            Date { year: 2022, month: 2, day: 8, hour: 17, minute: 12, second: 12 },
            testdata::to_attr_borrowed(b"2022-02-08T17:12:12").try_into().unwrap()
        );
    }

    #[test]
    fn test_try_from_attr_borrowed_invalid() {
        assert!(Date::try_from(testdata::to_attr_borrowed(b"2021-05/16T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"something invalid")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"2021-05-1617:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"2021/05-12T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"202105-16T17:41:12")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"2021-05-16T17:4112")).is_err());
        assert!(Date::try_from(testdata::to_attr_borrowed(b"2021-05-16T1741:12")).is_err());
    }


    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 }), "2021-05-16T17:41:12");
        assert_eq!(format!("{}", Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 }), "0002-05-03T01:04:07");
    }

    #[test]
    fn test_partial_eq() {
        assert_eq!(Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 },
                   Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 });
        assert_eq!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 1, month: 5, day: 3, hour: 1, minute: 4, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 6, day: 3, hour: 1, minute: 4, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 5, day: 4, hour: 1, minute: 4, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 5, day: 3, hour: 3, minute: 4, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 5, day: 3, hour: 1, minute: 1, second: 07 });
        assert_ne!(Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 },
                   Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 12 });
    }
}