//! Module for the unit Master for LogicalChannel used in GDTF
use std::fmt::{Display, Formatter};

///Master representation for logicalChannel used in GDTF
#[derive(Debug)]
pub enum Master {
    None,
    Grand,
    Group,
}

impl Default for Master {
    fn default() -> Self {
        Master::None
    }
}

impl From<&str> for Master {
    fn from(s: &str) -> Self {
        use Master::*;
        match s {
            "None" => None,
            "Grand" => Grand,
            "Group" => Group,
            _ => None
        }
    }
}

#[cfg(test)]
impl PartialEq for Master {
    fn eq(&self, other: &Self) -> bool {
        use Master::*;
        match self {
            None => {
                match other {
                    None => true,
                    _ => false
                }
            }
            Grand => {
                match other {
                    Grand => true,
                    _ => false
                }
            }
            Group => {
                match other {
                    Group => true,
                    _ => false
                }
            }
        }
    }
}

impl Display for Master {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Master::*;
        match self {
            None => write!(f, "None"),
            Grand => write!(f, "Grand"),
            Group => write!(f, "Group"),
        }
    }
}


#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::master::Master;

    #[test]
    fn test_valid() {
        assert_eq!(
            Master::Grand,
            Master::try_from("Grand").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            Master::Group,
            Master::try_from("Group").unwrap()
        );
    }

    #[test]
    fn test_valid_3() {
        assert_eq!(
            Master::None,
            Master::try_from("None").unwrap()
        );
    }


    #[test]
    fn test_invalid_2() {
        assert_eq!(
            Master::None,
            Master::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            Master::None,
            Master::try_from("").unwrap()
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Master::Grand), "Grand");
    }
}