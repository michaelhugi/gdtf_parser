//!Defines the color space that is used for color mixing with indirect RGB, Hue/Sat, xyY or CMY control input
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::color_cie::ColorCie;

///Defines the color space that is used for color mixing with indirect RGB, Hue/Sat, xyY or CMY control input
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct ColorSpace {
    ///CIE xyY of the Red Primary
    pub red: ColorCie,

    ///CIE xyY of the Green Primary
    pub green: ColorCie,

    ///CIE xyY of the Blue Primary
    pub blue: ColorCie,

    ///CIE xyY of the White Point
    pub white_point: ColorCie,
}

#[derive(Default)]
pub(crate) struct ColorSpaceHolder {
    /// Definition of the Color Space that used for the indirect color mixing. The defined values are "Custom", "sRGB", "ProPhoto" and "ANSI". Default Value: "sRGB"
    pub mode: ColorSpaceMode,

    ///CIE xyY of the Red Primary; this is used only if the ColorSpace is "Custom".
    pub red: Option<ColorCie>,

    ///CIE xyY of the Green Primary; this is used only if the ColorSpace is "Custom".
    pub green: Option<ColorCie>,

    ///CIE xyY of the Blue Primary; this is used only if the ColorSpace is "Custom".
    pub blue: Option<ColorCie>,

    ///CIE xyY of the White Point; this is used only if the ColorSpace is "Custom".
    pub white_point: Option<ColorCie>,
}

impl ReadGdtf for ColorSpace {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = ColorSpaceHolder;
    const NODE_NAME: &'static [u8] = b"ColorSpace";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error> {
        match attr.key {
            b"Mode" => data_holder.mode = ColorSpaceMode::new_from_attr(attr),
            b"Red" => data_holder.red = Some(ColorCie::new_from_attr(attr)?),
            b"Green" => data_holder.green = Some(ColorCie::new_from_attr(attr)?),
            b"Blue" => data_holder.blue = Some(ColorCie::new_from_attr(attr)?),
            b"WhitePoint" => data_holder.white_point = Some(ColorCie::new_from_attr(attr)?),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(
        _: &mut Self::DataHolder,
        _: &mut Reader<&[u8]>,
        _: BytesStart<'_>,
        _: bool,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(match data_holder.mode {
            ColorSpaceMode::Custom => Self {
                red: data_holder
                    .red
                    .ok_or_else(|| Self::attribute_not_found(b"Red"))?,
                green: data_holder
                    .green
                    .ok_or_else(|| Self::attribute_not_found(b"Green"))?,
                blue: data_holder
                    .blue
                    .ok_or_else(|| Self::attribute_not_found(b"Blue"))?,
                white_point: data_holder
                    .white_point
                    .ok_or_else(|| Self::attribute_not_found(b"WhitePoint"))?,
            },
            ColorSpaceMode::ProPhoto => Self {
                red: PRO_PHOTO_RED,
                green: PRO_PHOTO_GREEN,
                blue: PRO_PHOTO_BLUE,
                white_point: PRO_PHOTO_WHITE_POINT,
            },
            ColorSpaceMode::SRgb => Self {
                red: SRGB_RED,
                green: SRGB_GREEN,
                blue: SRGB_BLUE,
                white_point: SRGB_WHITE_POINT,
            },
            ColorSpaceMode::Ansi => Self {
                red: ANSI_RED,
                green: ANSI_GREEN,
                blue: ANSI_BLUE,
                white_point: ANSI_WHITE_POINT,
            },
        })
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for ColorSpace {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(ColorSpace {
                    red: ColorCie {
                        x: 0.6400,
                        y: 0.3300,
                        Y: 021.26,
                    },
                    green: ColorCie {
                        x: 0.3000,
                        y: 0.6000,
                        Y: 071.52,
                    },
                    blue: ColorCie {
                        x: 0.1500,
                        y: 0.0600,
                        Y: 007.22,
                    },
                    white_point: ColorCie {
                        x: 0.3127,
                        y: 0.3290,
                        Y: 100.00,
                    },
                }),
            ),
            (
                None,
                Some(ColorSpace {
                    red: ColorCie {
                        x: 0.7347,
                        y: 0.2653,
                        Y: 100.0,
                    },
                    green: ColorCie {
                        x: 0.1596,
                        y: 0.8404,
                        Y: 100.0,
                    },
                    blue: ColorCie {
                        x: 0.0366,
                        y: 0.0001,
                        Y: 100.0,
                    },
                    white_point: ColorCie {
                        x: 0.3457,
                        y: 0.3585,
                        Y: 100.0,
                    },
                }),
            ),
            (
                None,
                Some(ColorSpace {
                    red: ColorCie {
                        x: 0.7347,
                        y: 0.2653,
                        Y: 100.0,
                    },
                    green: ColorCie {
                        x: 0.1596,
                        y: 0.8404,
                        Y: 100.0,
                    },
                    blue: ColorCie {
                        x: 0.0366,
                        y: 0.001,
                        Y: 100.0,
                    },
                    white_point: ColorCie {
                        x: 0.4254,
                        y: 0.4044,
                        Y: 100.0,
                    },
                }),
            ),
            (
                None,
                Some(ColorSpace {
                    red: ColorCie {
                        x: 1.3,
                        y: 3.2,
                        Y: 13.4,
                    },
                    green: ColorCie {
                        x: 12.1,
                        y: 73.2,
                        Y: 46.2,
                    },
                    blue: ColorCie {
                        x: 74.3,
                        y: 93.0,
                        Y: 77.1,
                    },
                    white_point: ColorCie {
                        x: 90.1,
                        y: 38.5,
                        Y: 12.1,
                    },
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<ColorSpace Mode="sRGB"/>"#.to_string(),
            r#"<ColorSpace Mode="ProPhoto"/>"#.to_string(),
            r#"<ColorSpace Mode="ANSI"/>"#.to_string(),
            r#"<ColorSpace Mode="Custom" Red="1.3,3.2,13.4" Green="12.1,73.2,46.2" Blue="74.3,93.0,77.1" WhitePoint="90.1,38.5,12.1"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![
            r#"<ColorSpace Mode="Custom" Green="12.1,73.2,46.2" Blue="74.3,93.0,77.1" WhitePoint="90.1,38.5,12.1"/>"#.to_string(),
            r#"<ColorSpace Mode="Custom" Red="1.3,3.2,13.4" Blue="74.3,93.0,77.1" WhitePoint="90.1,38.5,12.1"/>"#.to_string(),
            r#"<ColorSpace Mode="Custom" Red="1.3,3.2,13.4" Green="12.1,73.2,46.2" WhitePoint="90.1,38.5,12.1"/>"#.to_string(),
            r#"<ColorSpace Mode="Custom" Red="1.3,3.2,13.4" Green="12.1,73.2,46.2" Blue="74.3,93.0,77.1"/>"#.to_string(),
            r#"<ColorSpace Mode="Custom"/>"#.to_string(),
        ]
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of Predefined
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

///0.6400, 0.3300, 0.2126
const SRGB_RED: ColorCie = ColorCie {
    x: 0.64,
    y: 0.33,
    Y: 21.26,
};
///0.3000, 0.6000, 0.7152
const SRGB_GREEN: ColorCie = ColorCie {
    x: 0.3,
    y: 0.6,
    Y: 71.52,
};
///0.1500, 0.0600, 0.0722
const SRGB_BLUE: ColorCie = ColorCie {
    x: 0.15,
    y: 0.06,
    Y: 7.22,
};
///0.3127, 0.3290, 1.0000
const SRGB_WHITE_POINT: ColorCie = ColorCie {
    x: 0.3127,
    y: 0.329,
    Y: 100.0,
};

///0.7347, 0.2653
const PRO_PHOTO_RED: ColorCie = ColorCie {
    x: 0.7347,
    y: 0.2653,
    Y: 100.0,
};
///0.1596, 0.8404
const PRO_PHOTO_GREEN: ColorCie = ColorCie {
    x: 0.1596,
    y: 0.8404,
    Y: 100.0,
};
///0.0366, 0.0001
const PRO_PHOTO_BLUE: ColorCie = ColorCie {
    x: 0.0366,
    y: 0.0001,
    Y: 100.0,
};
///0.3457, 0.3585
const PRO_PHOTO_WHITE_POINT: ColorCie = ColorCie {
    x: 0.3457,
    y: 0.3585,
    Y: 100.0,
};

///0.7347, 0.2653
const ANSI_RED: ColorCie = ColorCie {
    x: 0.7347,
    y: 0.2653,
    Y: 100.0,
};
///0.1596, 0.8404
const ANSI_GREEN: ColorCie = ColorCie {
    x: 0.1596,
    y: 0.8404,
    Y: 100.0,
};
///0.0366, 0.001
const ANSI_BLUE: ColorCie = ColorCie {
    x: 0.0366,
    y: 0.001,
    Y: 100.0,
};
///0.4254, 0.4044
const ANSI_WHITE_POINT: ColorCie = ColorCie {
    x: 0.4254,
    y: 0.4044,
    Y: 100.0,
};

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of Predefined
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of ColorSpaceMode
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
///Definition of the Color Space that used for the indirect color mixing. The defined values are "Custom", "sRGB", "ProPhoto" and "ANSI". Default Value: "sRGB"
#[derive(Clone, PartialEq, Debug)]
pub enum ColorSpaceMode {
    Custom,
    ///Adobe sRGB, HDTV IEC 61966-2-1:1999
    SRgb,
    ///Kodak ProPhoto ROMM RGB ISO 22028-2:2013
    ProPhoto,
    ///ANSI E1.54-2015
    Ansi,
}

impl ColorSpaceMode {
    /// Parses a str provided by gdtf-xml-description to ColorSpaceMode
    /// ```rust
    /// use gdtf_parser::fixture_type::physical_descriptions::color_space::ColorSpaceMode;
    /// assert_eq!(ColorSpaceMode::Custom, ColorSpaceMode::new_from_str("Custom"));
    /// assert_eq!(ColorSpaceMode::SRgb, ColorSpaceMode::new_from_str("sRGB"));
    /// assert_eq!(ColorSpaceMode::ProPhoto, ColorSpaceMode::new_from_str("ProPhoto"));
    /// assert_eq!(ColorSpaceMode::Ansi, ColorSpaceMode::new_from_str("ANSI"));
    /// assert_eq!(ColorSpaceMode::SRgb, ColorSpaceMode::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(value: &str) -> Self {
        match value {
            "Custom" => Self::Custom,
            "ProPhoto" => Self::ProPhoto,
            "ANSI" => Self::Ansi,
            _ => Self::SRgb,
        }
    }

    /// Parses a quick-xml-attribute provided by gdtf-xml-description to ColorSpaceMode
    /// ```rust
    /// use gdtf_parser::fixture_type::physical_descriptions::color_space::ColorSpaceMode;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(ColorSpaceMode::Custom, ColorSpaceMode::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Custom")}));
    /// assert_eq!(ColorSpaceMode::SRgb, ColorSpaceMode::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"sRGB")}));
    /// assert_eq!(ColorSpaceMode::ProPhoto, ColorSpaceMode::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"ProPhoto")}));
    /// assert_eq!(ColorSpaceMode::Ansi, ColorSpaceMode::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"ANSI")}));
    /// assert_eq!(ColorSpaceMode::SRgb, ColorSpaceMode::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Anything else")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

///```rust
/// use gdtf_parser::fixture_type::physical_descriptions::color_space::ColorSpaceMode;
/// assert_eq!(ColorSpaceMode::default(), ColorSpaceMode::SRgb)
/// ```
impl Default for ColorSpaceMode {
    fn default() -> Self {
        Self::SRgb
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of ColorSpaceMode
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::color_space::ColorSpace;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        ColorSpace::execute_tests()
    }
}
