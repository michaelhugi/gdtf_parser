//!Defines the general properties of the device type
use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;
use serde::{Serialize, Deserialize};

use crate::fixture_type::physical_descriptions::properties::leg_height::LegHeight;
use crate::fixture_type::physical_descriptions::properties::operating_temperature::OperatingTemperature;
use crate::fixture_type::physical_descriptions::properties::power_consumtion::PowerConsumtion;
use crate::fixture_type::physical_descriptions::properties::weight::Weight;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
#[cfg(test)]
use crate::utils::units::node::Node;

pub mod leg_height;
pub mod operating_temperature;
pub mod power_consumtion;
pub mod weight;

///Defines the general properties of the device type
#[derive(Debug, PartialEq, Default, Clone, Serialize, Deserialize)]
pub struct Properties {
    ///Temperature range in which the device can be operated.
    pub operationg_temperature: Option<OperatingTemperature>,
    ///Weight of the device including all accessories
    pub weight: Option<Weight>,
    ///Power information for a given connector
    pub power_consumtion: Vec<PowerConsumtion>,
    ///Height of the legs.
    pub leg_height: Option<LegHeight>,
}

impl ReadGdtf for Properties {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = Properties;
    const NODE_NAME: &'static [u8] = b"Properties";
    const PARENT_NODE_NAME: &'static [u8] = &[];
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(_: &mut Self::DataHolder, _: Attribute<'_>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error> {
        match event.name() {
            OperatingTemperature::NODE_NAME => {
                data_holder.operationg_temperature = Some(
                    OperatingTemperature::read_single_from_event(reader, event, has_children)?.1,
                )
            }
            Weight::NODE_NAME => {
                data_holder.weight =
                    Some(Weight::read_single_from_event(reader, event, has_children)?.1)
            }
            PowerConsumtion::NODE_NAME => data_holder
                .power_consumtion
                .push(PowerConsumtion::read_single_from_event(reader, event, has_children)?.1),
            LegHeight::NODE_NAME => {
                data_holder.leg_height =
                    Some(LegHeight::read_single_from_event(reader, event, has_children)?.1)
            }
            _ => {}
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        _: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

#[cfg(test)]
impl TestReadGdtf for Properties {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                None,
                Some(Properties {
                    operationg_temperature: Some(OperatingTemperature {
                        low: 0.0,
                        high: 45.0,
                    }),
                    weight: Some(Weight { value: 23.0 }),
                    power_consumtion: vec![PowerConsumtion {
                        value: 800.000000,
                        power_factor: 0.960000,
                        connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(),
                        voltage_low: 111.000001,
                        voltage_high: 241.000000,
                        frequency_low: 49.0,
                        frequency_high: 61.0,
                    }],
                    leg_height: Some(LegHeight { value: 0.012 }),
                }),
            ),
            (
                None,
                Some(Properties {
                    operationg_temperature: None,
                    weight: None,
                    power_consumtion: vec![],
                    leg_height: None,
                }),
            ),
            (
                None,
                Some(Properties {
                    operationg_temperature: None,
                    weight: None,
                    power_consumtion: vec![],
                    leg_height: None,
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"
            <Properties>
        <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="61.000000" FrequencyLow="49.000000" PowerFactor="0.960000" Value="800.000000" VoltageHigh="241.000000" VoltageLow="111.000001"/>
        <OperatingTemperature High="45.000000" Low="0.000000"/>
        <Weight Value="23.000000"/>
        <LegHeight Value="0.012000"/>
      </Properties>
           "#.to_string(),
            "<Properties></Properties>".to_string(),
            "<Properties/>".to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::properties::Properties;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Properties::execute_tests();
    }
}
