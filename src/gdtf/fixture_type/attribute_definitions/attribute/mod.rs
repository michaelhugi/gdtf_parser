use std::borrow::Borrow;
use std::convert::TryInto;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::deparse::{DeparseSingle, DeparseVec};
use crate::errors::GdtfError;
use crate::units::color::ColorCIE;
use crate::units::name::Name;
use crate::units::physical_unit::PhysicalUnit;

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
                b"Name" => name = std::str::from_utf8(attr.value.borrow())?.try_into()?,
                b"Pretty" => pretty = std::str::from_utf8(attr.value.borrow())?.to_owned(),
                b"ActivationGroup" => activation_group = Some(std::str::from_utf8(attr.value.borrow())?.to_owned()),
                b"Feature" => feature = std::str::from_utf8(attr.value.borrow())?.to_owned(),
                b"MainAttribute" => main_attribute = Some(std::str::from_utf8(attr.value.borrow())?.to_owned()),
                b"PhysicalUnit" => physical_unit = std::str::from_utf8(attr.value.borrow())?.into(),
                b"Color" => color = Some(std::str::from_utf8(attr.value.borrow())?.try_into()?),
                _ => {}
            }
        }
        if name.is_empty() {
            return Err(GdtfError::RequiredValueNotFoundError("Name not found in Attribute".to_string()));
        }
        if pretty == "" {
            return Err(GdtfError::RequiredValueNotFoundError("Pretty not found in Attribute".to_string()));
        }
        if feature == "" {
            return Err(GdtfError::RequiredValueNotFoundError("Feature not found in Attribute".to_string()));
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
    fn is_single_eq(&self, other: &Self) -> bool {
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

    use crate::deparse::DeparseSingle;
    use crate::gdtf::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::units::color::ColorCIE;
    use crate::units::physical_unit::PhysicalUnit;

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
                z: 100.000000,
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
}