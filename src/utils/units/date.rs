//! Module for the unit Date used in GDTF
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::utils::errors::GdtfError;

///Date representation used in GDTF
#[derive(Debug)]
pub struct Date {
    ///The GDTF Date represented as Rust's DateTime
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl TryFrom<&str> for Date {
    type Error = GdtfError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value: Vec<&str> = value.split("T").collect();
        if value.len() != 2 {
            return Err(GdtfError::DateNotValidError("The Date must be formatted yyyy-mm-ddThh:mm:ss instance of T != 1".to_string()));
        }
        let date = value[0];
        let time = value[1];

        let date: Vec<&str> = date.split("-").collect();
        if date.len() != 3 {
            return Err(GdtfError::DateNotValidError("The Date must be formatted yyyy-mm-ddThh:mm:ss instance of - != 2".to_string()));
        }
        let time: Vec<&str> = time.split(":").collect();
        if time.len() != 3 {
            return Err(GdtfError::DateNotValidError("The Date must be formatted yyyy-mm-ddThh:mm:ss instance of : != 2".to_string()));
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

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}", self.year, self.month, self.day, self.hour, self.minute, self.second)
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::date::Date;

    #[test]
    fn test_valid() {
        assert_eq!(
            Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 },
            Date::try_from("2021-05-16T17:41:12").unwrap()
        );
    }

    #[test]
    fn test_invalid_all() {
        match Date::try_from("something invalid") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }


    #[test]
    fn test_invalid_empty() {
        match Date::try_from("") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_first() {
        match Date::try_from("2021/05-16T17:41:12") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_first2() {
        match Date::try_from("asdfT17:41:12") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_first3() {
        match Date::try_from("T17:41:12") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_second() {
        match Date::try_from("2021-05-16T17:41/12") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_second_2() {
        match Date::try_from("2021-05-16T") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_invalid_second_3() {
        match Date::try_from("2021-05-16Tdasd") {
            Ok(_) => { panic!("test_invalid should return an error"); }
            Err(_) => {}
        }
    }

    #[test]
    fn test_display_1() {
        assert_eq!(format!("{}", Date { year: 2021, month: 5, day: 16, hour: 17, minute: 41, second: 12 }), "2021-05-16T17:41:12");
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", Date { year: 2, month: 5, day: 3, hour: 1, minute: 4, second: 07 }), "0002-05-03T01:04:07");
    }
}