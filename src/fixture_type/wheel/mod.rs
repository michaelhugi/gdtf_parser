//! Each wheel describes a single physical or virtual wheel of the fixture type.
use std::collections::HashMap;
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::wheel::slot::Slot;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

pub mod slot;

///Each wheel describes a single physical or virtual wheel of the fixture type.
#[derive(Debug, PartialEq, Default, Clone)]
pub struct Wheel {
    /// All slots for the wheel
    pub slots: HashMap<Name, Slot>,
}

impl ReadGdtf for Wheel {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = Wheel;
    const NODE_NAME: &'static [u8] = b"Wheel";
    const PARENT_NODE_NAME: &'static [u8] = b"Wheels";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
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
        if event.name() == Slot::NODE_NAME {
            let slot = Slot::read_single_from_event(reader, event, has_children)?;
            data_holder
                .slots
                .insert(slot.0.unwrap_or_else(|| Name("?".to_string())), slot.1);
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(
        attr: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for Wheel {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                Some(Name::new("PrismWheel 1").unwrap()),
                Some(Self {
                    slots: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("PrismWheel 2").unwrap()),
                Some(Self {
                    slots: HashMap::new(),
                }),
            ),
            (
                Some(Name::new("PrismWheel 3").unwrap()),
                Some(Self {
                    slots: Slot::testdata_hash_map(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Wheel Name="PrismWheel 1"/>"#.to_string(),
            r#"<Wheel Name="PrismWheel 2"></Wheel>"#.to_string(),
            format!(
                r#"<Wheel Name="PrismWheel 3">{}</Wheel>"#,
                Slot::testdata_xml()
            ),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::wheel::Wheel;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        Wheel::execute_tests();
    }
}
