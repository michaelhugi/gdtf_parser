//!Describes a macro defined by the manufacturer
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::MacroDmxStep;
use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;

pub mod macro_dmx_step;

///Describes a macro defined by the manufacturer
#[derive(Debug, PartialEq, Clone, Default)]
pub struct FtMacro {
    ///The unique name of the macro
    pub name: String,
    ///All steps to execute the Macro
    pub macro_dmx_steps: Vec<MacroDmxStep>,
}

impl ReadGdtf for FtMacro {
    type PrimaryKey = ();
    type Error = GdtfError;
    type DataHolder = FtMacro;

    const NODE_NAME: &'static [u8] = b"FTMacro";
    const PARENT_NODE_NAME: &'static [u8] = b"FTMacros";
    const PRIMARY_KEY_NAME: &'static [u8] = &[];
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        if attr.key == b"Name" {
            data_holder.name = read::attr_to_string(attr);
        }
        Ok(())
    }

    fn read_any_child(data_holder: &mut Self::DataHolder, reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(), Self::Error> {
        if event.name() == MacroDmxStep::PARENT_NODE_NAME {
            data_holder.macro_dmx_steps = MacroDmxStep::read_vec_from_event(reader, event, has_children)?;
        }
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(data_holder)
    }

    fn read_primary_key_from_attr(_: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        panic!("Should not be executed!");
    }
}

#[cfg(test)]
impl TestReadGdtf for FtMacro {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (None, Some(FtMacro { name: "Macro Name 1".to_string(), macro_dmx_steps: vec![] })),
            (None, Some(FtMacro { name: "Macro Name 2".to_string(), macro_dmx_steps: vec![] })),
            (None, Some(FtMacro { name: "Macro Name 2a".to_string(), macro_dmx_steps: vec![] })),
            (None, Some(FtMacro { name: "Macro Name 3".to_string(), macro_dmx_steps: MacroDmxStep::testdata_vec() })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<FTMacro Name="Macro Name 1"/>"#.to_string(),
            r#"<FTMacro Name="Macro Name 2"></FTMacro>"#.to_string(),
            r#"<FTMacro Name="Macro Name 2a"><MacroDMX/></FTMacro>"#.to_string(),
            format!(r#"<FTMacro Name="Macro Name 3"><MacroDMX>{}</MacroDMX></FTMacro>"#, MacroDmxStep::testdata_xml())
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::ft_macro::FtMacro;
    use crate::utils::read::TestReadGdtf;

    #[test]
    fn test_deparse() {
        FtMacro::execute_tests();
    }
}