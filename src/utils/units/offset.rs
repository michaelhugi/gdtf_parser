//! Module for the unit Offset used for DMXChannel used in GDTF


use std::fmt::{Display, Formatter};
use std::str::FromStr;

///Offset used for DMXChannel in GDTF
#[derive(Debug)]
pub enum Offset {
    ///Relative addresses of the current DMX channel from highest to least significant
    Value(Vec<u32>),
    ///No Offset
    None,
}

impl Default for Offset {
    fn default() -> Self {
        Offset::None
    }
}

impl From<&str> for Offset {
    fn from(s: &str) -> Self {
        use Offset::*;
        if s == "None" {
            return None;
        }
        let mut v = Vec::new();
        for s in s.split(",").into_iter() {
            match u32::from_str(s) {
                Ok(s) => v.push(s),
                Err(_) => return Self::default()
            }
        }
        Value(v)
    }
}

#[cfg(test)]
impl PartialEq for Offset {
    fn eq(&self, other: &Self) -> bool {
        use Offset::*;
        match self {
            Value(one) => match other {
                Value(two) => one == two,
                _ => false
            },
            None => match other {
                None => true,
                _ => false
            }
        }
    }
}

impl Display for Offset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Offset::*;
        match self {
            Value(x) => {
                let mut s = String::new();
                for v in x {
                    s = format!("{},{}", s, v);
                }
                let s = if s.len() > 0 { &s[1..s.len()] } else { &"None" };
                write!(f, "{}", s)
            }
            None => write!(f, "None")
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::offset::Offset;

    #[test]
    fn test_valid() {
        assert_eq!(
            Offset::None,
            Offset::try_from("").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            Offset::Value(vec![1]),
            Offset::try_from("1").unwrap()
        );
    }

    #[test]
    fn test_valid_3() {
        assert_eq!(
            Offset::None,
            Offset::try_from("None").unwrap()
        );
    }

    #[test]
    fn test_valid_4() {
        assert_eq!(
            Offset::Value(vec![1, 3]),
            Offset::try_from("1,3").unwrap()
        );
    }


    #[test]
    fn test_invalid() {
        assert_eq!(
            Offset::default(),
            Offset::try_from("Something").unwrap()
        );
    }

    #[test]
    fn test_invalid_5() {
        assert_eq!(
            Offset::None,
            Offset::try_from("1,3,").unwrap()
        );
    }

    #[test]
    fn test_invalid_6() {
        assert_eq!(
            Offset::None,
            Offset::try_from(",1,3").unwrap()
        );
    }

    #[test]
    fn test_invalid_7() {
        assert_eq!(
            Offset::None,
            Offset::try_from(",1,3,").unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Offset::Value(vec![12])), "12");
    }

    #[test]
    fn test_display_2() {
        assert_eq!(format!("{}", Offset::Value(vec![1, 12])), "1,12");
    }

    #[test]
    fn test_display_3() {
        assert_eq!(format!("{}", Offset::None), "None");
    }

    #[test]
    fn test_display_4() {
        assert_eq!(format!("{}", Offset::Value(vec![])), "None");
    }
}