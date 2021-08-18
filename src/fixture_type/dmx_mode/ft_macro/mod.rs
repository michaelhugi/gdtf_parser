//!Describes a macro defined by the manufacturer
use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::fixture_type::dmx_mode::ft_macro::macro_dmx_step::MacroDmxStep;
use crate::utils::errors::GdtfError;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;

pub mod macro_dmx_step;

///Describes a macro defined by the manufacturer
#[derive(Debug, PartialEq, Clone, Default)]
pub struct FtMacro {
    ///All steps to execute the Macro
    pub macro_dmx_steps: Vec<MacroDmxStep>,
}

impl ReadGdtf for FtMacro {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = FtMacro;

    const NODE_NAME: &'static [u8] = b"FTMacro";
    const PARENT_NODE_NAME: &'static [u8] = b"FTMacros";
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
        if event.name() == MacroDmxStep::PARENT_NODE_NAME {
            data_holder.macro_dmx_steps =
                MacroDmxStep::read_vec_from_event(reader, event, has_children)?;
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
impl TestReadGdtf for FtMacro {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (
                Some(Name::new("Macro Name 1").unwrap()),
                Some(FtMacro {
                    macro_dmx_steps: vec![],
                }),
            ),
            (
                Some(Name::new("Macro Name 2").unwrap()),
                Some(FtMacro {
                    macro_dmx_steps: vec![],
                }),
            ),
            (
                Some(Name::new("Macro Name 2a").unwrap()),
                Some(FtMacro {
                    macro_dmx_steps: vec![],
                }),
            ),
            (
                Some(Name::new("Macro Name 3").unwrap()),
                Some(FtMacro {
                    macro_dmx_steps: MacroDmxStep::testdata_vec(),
                }),
            ),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<FTMacro Name="Macro Name 1"/>"#.to_string(),
            r#"<FTMacro Name="Macro Name 2"></FTMacro>"#.to_string(),
            r#"<FTMacro Name="Macro Name 2a"><MacroDMX/></FTMacro>"#.to_string(),
            format!(
                r#"<FTMacro Name="Macro Name 3"><MacroDMX>{}</MacroDMX></FTMacro>"#,
                MacroDmxStep::testdata_xml()
            ),
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
