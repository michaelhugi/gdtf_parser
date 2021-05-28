//! Module for the unit Snap for LogicalChannel used in GDTF


///Snap representation for logicalChannel used in GDTF
#[derive(Debug)]
pub enum Snap {
    No,
    Yes,
    On,
    Off,
}

impl Default for Snap {
    fn default() -> Self {
        Snap::No
    }
}

impl From<&str> for Snap {
    fn from(s: &str) -> Self {
        use Snap::*;
        match s {
            "No" => No,
            "Yes" => Yes,
            "On" => On,
            "Off" => Off,
            _ => No
        }
    }
}

#[cfg(test)]
impl PartialEq for Snap {
    fn eq(&self, other: &Self) -> bool {
        use Snap::*;
        match self {
            No => {
                match other {
                    No => true,
                    _ => false
                }
            }
            Yes => {
                match other {
                    Yes => true,
                    _ => false
                }
            }
            On => {
                match other {
                    On => true,
                    _ => false
                }
            }
            Off => {
                match other {
                    Off => true,
                    _ => false
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use crate::utils::units::snap::Snap;

    #[test]
    fn test_valid() {
        assert_eq!(
            Snap::Yes,
            Snap::try_from("Yes").unwrap()
        );
    }

    #[test]
    fn test_valid_2() {
        assert_eq!(
            Snap::No,
            Snap::try_from("No").unwrap()
        );
    }


    #[test]
    fn test_invalid_2() {
        assert_eq!(
            Snap::No,
            Snap::try_from("something invalid").unwrap()
        );
    }

    #[test]
    fn test_invalid_3() {
        assert_eq!(
            Snap::No,
            Snap::try_from("").unwrap()
        );
    }
}