use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::connectors::ConnectionGender::{Female, Male, Neutral};
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::connector_type::ConnectorType;
use crate::utils::units::name::Name;

///defines the connector
#[derive(Debug, PartialEq, Default)]
pub struct Connector {
    ///The type of the connector. Find a list of predefined types in Annex D.
    pub connector_type: ConnectorType,
    ///Optional: Defines which DMX Break this connector belongs to.
    pub dmx_break: Option<u32>,
    ///Connectors where the addition of the Gender value equals 0, can be connected; Default value: 0; Male Connectors are −1, Female are +1, Universal are 0.
    pub gender: ConnectionGender,
    ///Defines the length of the connector’s wire in meters. "0" means that there is no cable and the connector is built into the housing. Default value "0"
    pub length: f32,
}

///Connectors where the addition of the Gender value equals 0, can be connected; Default value: 0; Male Connectors are −1, Female are +1, Universal are 0.
#[derive(Debug, PartialEq)]
pub enum ConnectionGender {
    Male = -1,
    Neutral = 0,
    Female = 1,
}

///Default fro ConnectionGender is 0 = Neutral
impl Default for ConnectionGender {
    fn default() -> Self {
        Self::Neutral
    }
}


impl Connector {
    ///Tells if two connectors are able to connect or not. They must be of right type and the gender must be correct
    pub fn can_connect(&self, other: &Self) -> bool {
        self.connector_type == other.connector_type && match self.gender {
            Female => -1,
            Male => 1,
            _ => 0
        } + match other.gender {
            Female => -1,
            Male => 1,
            _ => 0
        } == 0
    }
}

impl ReadGdtf for Connector {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = Connector;
    const NODE_NAME: &'static [u8] = b"Connector";
    const PARENT_NODE_NAME: &'static [u8] = b"Connectors";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"Type" => data_holder.connector_type = ConnectorType::new_from_attr(attr)?,
            b"DMXBreak" => data_holder.dmx_break = Some(u32::from_str(read::attr_try_to_str(&attr).unwrap_or("")).unwrap_or(0_u32)),
            b"Gender" => {
                data_holder.gender = match i8::from_str(read::attr_try_to_str(&attr).unwrap_or("")).unwrap_or(0_i8) {
                    -1 => Female,
                    1 => Male,
                    _ => Neutral
                }
            }
            b"Length" => data_holder.length = read::attr_to_f32(attr),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(_: &mut Self::DataHolder, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for Connector {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("DMX-IN").unwrap()), Some(Connector { connector_type: ConnectorType::XLR5, dmx_break: None, gender: Female, length: 0.0 })),
            (Some(Name::new("DMX-OUT").unwrap()), Some(Connector { connector_type: ConnectorType::XLR5, dmx_break: Some(0), gender: Male, length: 0.0 })),
            (Some(Name::new("Ethernet1").unwrap()), Some(Connector { connector_type: ConnectorType::RJ45, dmx_break: Some(1), gender: Neutral, length: 0.0 })),
            (Some(Name::new("Ethernet2").unwrap()), Some(Connector { connector_type: ConnectorType::RJ45, dmx_break: Some(1), gender: Neutral, length: 12.0 })),
            (Some(Name::new("powerCON TRUE1 IN").unwrap()), Some(Connector { connector_type: ConnectorType::PowerconTRUE1, dmx_break: Some(1), gender: Female, length: 0.001 })),
            (Some(Name::new("powerCON TRUE1 OUT").unwrap()), Some(Connector { connector_type: ConnectorType::Other(Name::new("Something Else").unwrap()), dmx_break: Some(1), gender: Male, length: 0.0 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Connector  Gender="-1" Length="0.000000" Name="DMX-IN" Type="XLR5"/>"#.to_string(),
            r#"<Connector DMXBreak="0" Gender="1" Name="DMX-OUT" Type="XLR5"/>"#.to_string(),
            r#"<Connector DMXBreak="1" Length="0.000000" Name="Ethernet1" Type="RJ45"/>"#.to_string(),
            r#"<Connector DMXBreak="1" Gender="0" Length="12.000000" Name="Ethernet2" Type="RJ45"/>"#.to_string(),
            r#"<Connector DMXBreak="1" Gender="-1" Length="0.001000" Name="powerCON TRUE1 IN" Type="PowerconTRUE1"/>"#.to_string(),
            r#"<Connector DMXBreak="1" Gender="1" Length="0.000000" Name="powerCON TRUE1 OUT" Type="Something Else"/>"#.to_string()
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::connectors::{ConnectionGender, Connector};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::units::connector_type::ConnectorType;
    use crate::utils::units::name::Name;

    #[test]
    fn test_deparse() {
        Connector::execute_tests()
    }

    #[test]
    fn test_connection_gender_default() {
        assert_eq!(ConnectionGender::default(), ConnectionGender::Neutral)
    }

    #[test]
    fn test_can_connect() {
        let bncf = Connector {
            connector_type: ConnectorType::BNC,
            dmx_break: None,
            gender: ConnectionGender::Female,
            length: 0.0,
        };
        let bncm = Connector {
            connector_type: ConnectorType::BNC,
            dmx_break: None,
            gender: ConnectionGender::Male,
            length: 0.0,
        };

        let han4f = Connector {
            connector_type: ConnectorType::HAN_4,
            dmx_break: None,
            gender: ConnectionGender::Female,
            length: 0.0,
        };
        let han4m = Connector {
            connector_type: ConnectorType::HAN_4,
            dmx_break: None,
            gender: ConnectionGender::Male,
            length: 0.0,
        };

        let stagepin = Connector {
            connector_type: ConnectorType::Stagepin,
            dmx_break: None,
            gender: ConnectionGender::default(),
            length: 0.0,
        };

        let custom1f = Connector {
            connector_type: ConnectorType::Other(Name::new("MyCustom1").unwrap()),
            dmx_break: None,
            gender: ConnectionGender::Female,
            length: 0.0,
        };
        let custom1m = Connector {
            connector_type: ConnectorType::Other(Name::new("MyCustom1").unwrap()),
            dmx_break: None,
            gender: ConnectionGender::Male,
            length: 0.0,
        };

        let custom2n = Connector {
            connector_type: ConnectorType::Other(Name::new("MyCustom2").unwrap()),
            dmx_break: None,
            gender: ConnectionGender::default(),
            length: 0.0,
        };

        assert!(bncf.can_connect(&bncm));
        assert!(bncm.can_connect(&bncf));
        assert!(han4f.can_connect(&han4m));
        assert!(han4m.can_connect(&han4f));
        assert!(stagepin.can_connect(&stagepin));
        assert!(custom1f.can_connect(&custom1m));
        assert!(custom1m.can_connect(&custom1f));
        assert!(custom2n.can_connect(&custom2n));

        assert!(!bncf.can_connect(&bncf));
        assert!(!bncm.can_connect(&bncm));
        assert!(!han4f.can_connect(&han4f));
        assert!(!han4m.can_connect(&han4m));
        assert!(!custom1f.can_connect(&custom1f));
        assert!(!custom1m.can_connect(&custom1m));

        assert!(!bncf.can_connect(&han4m));
        assert!(!bncf.can_connect(&han4f));
        assert!(!bncf.can_connect(&custom1f));
        assert!(!bncf.can_connect(&custom1m));
        assert!(!bncf.can_connect(&stagepin));
        assert!(!bncf.can_connect(&custom2n));

        assert!(!bncm.can_connect(&han4m));
        assert!(!bncm.can_connect(&han4f));
        assert!(!bncm.can_connect(&custom1f));
        assert!(!bncm.can_connect(&custom1m));
        assert!(!bncm.can_connect(&stagepin));
        assert!(!bncm.can_connect(&custom2n));

        assert!(!custom1f.can_connect(&han4m));
        assert!(!custom1f.can_connect(&han4f));
        assert!(!custom1f.can_connect(&bncf));
        assert!(!custom1f.can_connect(&bncm));
        assert!(!custom1f.can_connect(&stagepin));
        assert!(!custom1f.can_connect(&custom2n));

        assert!(!custom1m.can_connect(&han4m));
        assert!(!custom1m.can_connect(&han4f));
        assert!(!custom1m.can_connect(&bncf));
        assert!(!custom1m.can_connect(&bncm));
        assert!(!custom1m.can_connect(&stagepin));
        assert!(!custom1m.can_connect(&custom2n));
    }
}