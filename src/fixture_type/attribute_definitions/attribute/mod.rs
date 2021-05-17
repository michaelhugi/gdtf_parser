use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::{DeparseSingle, DeparseVec};
use crate::utils::errors::GdtfError;
use crate::utils::units::color_cie::ColorCIE;
use crate::utils::units::name::Name;
use crate::utils::units::physical_unit::PhysicalUnit;
use crate::utils::deparse;

///Describes a singular mutual exclusive control function
#[derive(Debug)]
pub struct Attribute {
    pub name: Name,
    pub pretty: String,
    pub activation_group: Option<String>,
    pub feature: String,
    pub main_attribute: Option<String>,
    pub physical_unit: PhysicalUnit,
    pub color: Option<ColorCIE>,
}

impl DeparseSingle for Attribute {
    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool {
        self.name == compare.name
    }
    fn single_from_event_unchecked(_reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        let mut name = Name::new();
        let mut pretty = String::new();
        let mut activation_group = None;
        let mut feature = String::new();
        let mut main_attribute = None;
        let mut physical_unit: PhysicalUnit = PhysicalUnit::None;
        let mut color: Option<ColorCIE> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = deparse::attr_to_name(&attr)?,
                b"Pretty" => pretty = deparse::attr_to_string(&attr)?,
                b"ActivationGroup" => activation_group = deparse::attr_to_string_option(&attr)?,
                b"Feature" => feature = deparse::attr_to_string(&attr)?,
                b"MainAttribute" => main_attribute = deparse::attr_to_string_option(&attr)?,
                b"PhysicalUnit" => physical_unit = deparse::attr_to_str(&attr)?.into(),
                b"Color" => color = match deparse::attr_to_str_option(&attr)? {
                    None => None,
                    Some(v) => Some(v.try_into()?)
                },
                _ => {}
            }
        }

        Ok(Attribute {
            feature,
            pretty,
            name,
            color,
            activation_group,
            main_attribute,
            physical_unit,
        })
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attribute"
    }

    fn single_event_name() -> String {
        "Attribute".to_string()
    }

    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool {
        self.name == other.name &&
            self.pretty == other.pretty &&
            self.activation_group.as_deref() == other.activation_group.as_deref() &&
            self.main_attribute.as_deref() == other.main_attribute.as_deref() &&
            self.physical_unit == other.physical_unit &&
            self.color == other.color
    }
}

impl DeparseVec for Attribute {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attributes"
    }

    fn group_event_name() -> String {
        "Attributes".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::utils::deparse::DeparseSingle;
    use crate::utils::units::color_cie::ColorCIE;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_attribute_all() {
        Attribute {
            name: "Sound".try_into().unwrap(),
            pretty: "SoundP".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Control.Control".to_string(),
            main_attribute: Some("Gobo1M".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: Some(ColorCIE {
                x: 0.312700,
                y: 0.329000,
                Y: 100.000000,
            }),
        }.test(
            r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP" ActivationGroup="Gobo1"  MainAttribute="Gobo1M" />"#
        )
    }

    #[test]
    fn test_attribute_min() {
        Attribute {
            name: "Sound".try_into().unwrap(),
            pretty: "SoundP".to_string(),
            activation_group: None,
            feature: "Control.Control".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        }.test(
            r#"<Attribute Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP"/>"#
        )
    }

    #[test]
    fn test_attribute_min_2() {
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(
            r#"<Attribute Feature="" Name="" MainAttribute="" ActivationGroup="" PhysicalUnit="" Pretty=""/>"#
        )
    }

    #[test]
    fn test_attribute_empty() {
        Attribute {
            name: "".try_into().unwrap(),
            pretty: "".to_string(),
            activation_group: None,
            feature: "".to_string(),
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(
            r#"<Attribute/>"#
        )
    }

    #[test]
    fn test_faulty() {
        match Attribute::single_from_xml(r#"<ttribute"#) {
            Ok(_) => { panic!("test_faulty should return an error"); }
            Err(_) => {}
        }
    }
}