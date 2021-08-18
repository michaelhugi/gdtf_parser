use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::physical_descriptions::properties::Properties;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::node::Node;

///defines the maximum power consumption per connector
#[derive(Debug, PartialEq, Clone)]
pub struct PowerConsumtion {
    ///Defines the power consumption of the connector at full load. Unit: VA. Default value: 0
    pub value: f32,
    ///Defines the cosine of phase of voltage relative to current. Unit: None. Default value: 1
    pub power_factor: f32,
    ///Name of the linked Connector
    pub connector: Node,
    ///Defines the lowest possible operating voltage. Unit: Volt. Default value: 90
    pub voltage_low: f32,
    ///Defines the highest possible operating voltage. Unit: Volt. Default value: 240
    pub voltage_high: f32,
    ///Defines the lowest possible operating frequency. Unit: Hertz. Default value: 50
    pub frequency_low: f32,
    ///Defines the highest possible operating frequency. Unit: Hertz. Default value: 60
    pub frequency_high: f32,
}

impl ReadGdtf for PowerConsumtion {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = PowerConsumtion;
    const NODE_NAME: &'static [u8] = b"PowerConsumption";
    const PARENT_NODE_NAME: &'static [u8] = Properties::NODE_NAME;
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"Value" => data_holder.value = read::attr_to_f32(attr),
            b"PowerFactor" => data_holder.power_factor = read::attr_to_f32(attr),
            b"Connector" => data_holder.connector = Node::new_from_attr(attr)?.ok_or_else(|| Self::attribute_not_found(b"Connector"))?,
            b"VoltageLow" => data_holder.voltage_low = read::attr_to_f32(attr),
            b"VoltageHigh" => data_holder.voltage_high = read::attr_to_f32(attr),
            b"FrequencyLow" => data_holder.frequency_low = read::attr_to_f32(attr),
            b"FrequencyHigh" => data_holder.frequency_high = read::attr_to_f32(attr),
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

    fn read_primary_key_from_attr(_: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed")
    }
}

///Default value implementation due to GDTF spec
impl Default for PowerConsumtion {
    fn default() -> Self {
        Self {
            value: 0.0,
            power_factor: 1.0,
            connector: Node::new_from_str("??").unwrap().unwrap(),
            voltage_low: 90.0,
            voltage_high: 240.0,
            frequency_low: 50.0,
            frequency_high: 60.0,
        }
    }
}

#[cfg(test)]
impl TestReadGdtf for PowerConsumtion {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 243.0, frequency_low: 40.0, frequency_high: 70.0 })),
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 243.0, frequency_low: 40.0, frequency_high: 60.0 })),
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 243.0, frequency_low: 50.0, frequency_high: 70.0 })),
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 1.0, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 243.0, frequency_low: 40.0, frequency_high: 70.0 })),
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 240.0, frequency_low: 40.0, frequency_high: 70.0 })),
            (None, Some(PowerConsumtion { value: 800.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 90.0, voltage_high: 243.0, frequency_low: 40.0, frequency_high: 70.0 })),
            (None, Some(PowerConsumtion { value: 0.0, power_factor: 0.96, connector: Node::new_from_str("powerCON TRUE1 IN").unwrap().unwrap(), voltage_low: 110.0, voltage_high: 243.0, frequency_low: 40.0, frequency_high: 70.0 })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" FrequencyLow="40.000000" PowerFactor="0.960000" Value="800.000000" VoltageHigh="243.000000" VoltageLow="110.000000"/>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyLow="40.000000" PowerFactor="0.960000" Value="800.000000" VoltageHigh="243.000000" VoltageLow="110.000000"/>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" PowerFactor="0.960000" Value="800.000000" VoltageHigh="243.000000" VoltageLow="110.000000"/>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" FrequencyLow="40.000000" Value="800.000000" VoltageHigh="243.000000" VoltageLow="110.000000"/>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" FrequencyLow="40.000000" PowerFactor="0.960000" Value="800.000000" VoltageLow="110.000000"/>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" FrequencyLow="40.000000" PowerFactor="0.960000" Value="800.000000" VoltageHigh="243.000000"></PowerConsumption>"#.to_string(),
            r#" <PowerConsumption Connector="powerCON TRUE1 IN" FrequencyHigh="70.000000" FrequencyLow="40.000000" PowerFactor="0.960000" VoltageHigh="243.000000" VoltageLow="110.000000"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::physical_descriptions::properties::power_consumtion::PowerConsumtion;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        PowerConsumtion::execute_tests();
    }
}