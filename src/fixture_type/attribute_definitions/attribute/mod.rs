use std::convert::TryInto;
use std::fmt::Debug;

use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::deparse::{DeparseHashMap, DeparseSingle};
use crate::utils::deparse;
#[cfg(test)]
use crate::utils::deparse::{TestDeparseHashMap, TestDeparseSingle};
use crate::utils::errors::GdtfError;
use crate::utils::units::attribute_name::AttributeName;
use crate::utils::units::color_cie::ColorCIE;
use crate::utils::units::node::node_attribute_feature::NodeAttributeFeature;
use crate::utils::units::physical_unit::PhysicalUnit;

///Describes a singular mutual exclusive control function
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    pub pretty: String,
    pub activation_group: Option<String>,
    pub feature: NodeAttributeFeature,
    pub main_attribute: Option<String>,
    pub physical_unit: PhysicalUnit,
    pub color: Option<ColorCIE>,
}

impl Attribute {
    pub fn new(pretty: &str, activation_group: Option<&str>, feature: NodeAttributeFeature, main_attribute: Option<&str>, physical_unit: PhysicalUnit, color: Option<ColorCIE>) -> Self {
        let main_attribute = match main_attribute {
            None => None,
            Some(value) => Some(value.to_string())
        };
        let activation_group = match activation_group {
            None => None,
            Some(value) => Some(value.to_string())
        };
        Self { pretty: pretty.to_string(), activation_group, feature, main_attribute, physical_unit, color }
    }
}

impl DeparseSingle for Attribute {
    type PrimaryKey = AttributeName;

    fn single_from_event(_reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> where
        Self: Sized {
        let mut name = Default::default();
        let mut pretty = String::new();
        let mut activation_group = None;
        let mut feature = NodeAttributeFeature::default();
        let mut main_attribute = None;
        let mut physical_unit: PhysicalUnit = PhysicalUnit::None;
        let mut color: Option<ColorCIE> = None;

        for attr in e.attributes().into_iter() {
            let attr = attr?;
            match attr.key {
                b"Name" => name = attr.into(),
                b"Pretty" => pretty = deparse::attr_to_string(&attr),
                b"ActivationGroup" => activation_group = deparse::attr_to_string_option(&attr),
                b"Feature" => feature = attr.into(),
                b"MainAttribute" => main_attribute = deparse::attr_to_string_option(&attr),
                b"PhysicalUnit" => physical_unit = deparse::attr_to_str(&attr).into(),
                b"Color" => color = attr.try_into().ok(),
                _ => {}
            }
        }

        Ok((Attribute {
            feature,
            pretty,
            color,
            activation_group,
            main_attribute,
            physical_unit,
        }, Some(name)))
    }

    fn is_single_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attribute"
    }

    fn single_event_name() -> String {
        "Attribute".to_string()
    }
}

impl DeparseHashMap for Attribute {
    fn is_group_event_name(event_name: &[u8]) -> bool {
        event_name == b"Attributes"
    }
}

#[cfg(test)]
impl TestDeparseSingle for Attribute {}

#[cfg(test)]
impl TestDeparseHashMap for Attribute {}

#[cfg(test)]
mod tests {
    use std::convert::TryInto;

    use crate::fixture_type::attribute_definitions::attribute::Attribute;
    use crate::utils::deparse::TestDeparseSingle;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::attribute_name::AttributeName;
    use crate::utils::units::color_cie::ColorCIE;
    use crate::utils::units::name::Name;
    use crate::utils::units::physical_unit::PhysicalUnit;

    #[test]
    fn test_attribute_all() -> Result<(), GdtfError> {
        Attribute {
            pretty: "SoundP".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Control.Control".try_into()?,
            main_attribute: Some("Gobo1M".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: Some(ColorCIE {
                x: 0.312700,
                y: 0.329000,
                Y: 100.000000,
            }),
        }.test(Some(AttributeName::UserDefined(Name::new_unchecked("Sound"))),
               r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP" ActivationGroup="Gobo1"  MainAttribute="Gobo1M" />"#,
        );
        Ok(())
    }

    #[test]
    fn test_attribute_all_2() -> Result<(), GdtfError> {
        Attribute {
            pretty: "SoundP".to_string(),
            activation_group: Some("Gobo1".to_string()),
            feature: "Control.Control".try_into()?,
            main_attribute: Some("Gobo1M".to_string()),
            physical_unit: PhysicalUnit::Angle,
            color: Some(ColorCIE {
                x: 0.312700,
                y: 0.329000,
                Y: 100.000000,
            }),
        }.test(Some(AttributeName::Effects_n_Adjust_m_(2, 4)),
               r#"<Attribute Color="0.312700,0.329000,100.000000" Feature="Control.Control" Name="Effects2Adjust4" PhysicalUnit="Angle" Pretty="SoundP" ActivationGroup="Gobo1"  MainAttribute="Gobo1M" />"#,
        );
        Ok(())
    }

    #[test]
    fn test_attribute_min() -> Result<(), GdtfError> {
        Attribute {
            pretty: "SoundP".to_string(),
            activation_group: None,
            feature: "Control.Control".try_into()?,
            main_attribute: None,
            physical_unit: PhysicalUnit::Angle,
            color: None,
        }.test(Some(AttributeName::UserDefined(Name::new_unchecked("Sound"))),
               r#"<Attribute Feature="Control.Control" Name="Sound" PhysicalUnit="Angle" Pretty="SoundP"/>"#,
        );
        Ok(())
    }

    #[test]
    fn test_attribute_min_2() -> Result<(), GdtfError> {
        Attribute {
            pretty: "".to_string(),
            activation_group: None,
            feature: "".try_into()?,
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(Some(AttributeName::UserDefined(Name::new_unchecked(""))),
               r#"<Attribute Feature="" Name="" MainAttribute="" ActivationGroup="" PhysicalUnit="" Pretty=""/>"#,
        );
        Ok(())
    }

    #[test]
    fn test_attribute_empty() -> Result<(), GdtfError> {
        Attribute {
            pretty: "".to_string(),
            activation_group: None,
            feature: "".try_into()?,
            main_attribute: None,
            physical_unit: PhysicalUnit::None,
            color: None,
        }.test(Some(AttributeName::UserDefined(Name::new_unchecked(""))),
               r#"<Attribute/>"#,
        );
        Ok(())
    }

    #[test]
    fn test_faulty() {
        assert!(Attribute::single_from_xml(r#"<ttribute"#).is_err());
    }
}