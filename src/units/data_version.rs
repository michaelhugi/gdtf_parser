#[derive(Debug)]
pub enum DataVersion {
    Version1_0,
    Version1_1,
    Unknown,
}


impl From<&str> for DataVersion {
    fn from(value: &str) -> Self {
        let mut value = value.split('.');
        match value.next() {
            Some(major) => {
                match major.parse::<i32>() {
                    Ok(major) => {
                        match value.next() {
                            Some(minor) => {
                                match minor.parse::<i32>() {
                                    Ok(minor) => {
                                        match major {
                                            1 => {
                                                match minor {
                                                    0 => DataVersion::Version1_0,
                                                    1 => DataVersion::Version1_1,
                                                    _ => DataVersion::Unknown
                                                }
                                            }
                                            _ => DataVersion::Unknown
                                        }
                                    }
                                    _ => DataVersion::Unknown
                                }
                            }
                            _ => DataVersion::Unknown
                        }
                    }
                    _ => DataVersion::Unknown
                }
            }
            _ => DataVersion::Unknown
        }
    }
}

impl PartialEq for DataVersion {
    fn eq(&self, other: &Self) -> bool {
        match self {
            DataVersion::Version1_0 => match other {
                DataVersion::Version1_0 => true,
                _ => false
            }
            DataVersion::Version1_1 => match other {
                DataVersion::Version1_1 => true,
                _ => false
            }
            DataVersion::Unknown => match other {
                DataVersion::Unknown => true,
                _ => false
            }
        }
    }
}