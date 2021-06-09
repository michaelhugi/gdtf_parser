//! # Definition of ActivationGroup
//! Attributes which need to be activated together to gain control over one logical function
//! Note 1 to entry: As example Pan & Tilt are paired to gain control over Position.

use quick_xml::events::BytesStart;

use crate::utils::deparse::DeparsePrimaryKey;
#[cfg(test)]
use crate::utils::deparse::TestDeparsePrimaryKey;
use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

///ActivationGroup only contains one attribute Name, so only this primary keys are stored in a vec in AttributeDefinitions
pub struct ActivationGroup {}

///Activation Group only contains
impl DeparsePrimaryKey for ActivationGroup {
    type Error = GdtfError;
    type PrimaryKey = Name;

    const NODE_NAME: &'static [u8] = b"ActivationGroup";
    const PARENT_NODE_NAME: &'static [u8] = b"ActivationGroups";

    fn read_primary_key_from_event(event: BytesStart<'_>) -> Result<Name, GdtfError> {
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == b"Name" {
                return Ok(Name::new_from_attr(attr)?);
            }
        }
        Ok(Default::default())
    }
}

#[cfg(test)]
impl TestDeparsePrimaryKey for ActivationGroup {}

#[cfg(test)]
pub mod tests {
    use crate::fixture_type::attribute_definitions::activation_group::ActivationGroup as T;
    use crate::utils::deparse::TestDeparsePrimaryKey;
    use crate::utils::errors::GdtfError;
    use crate::utils::units::name::Name;

    #[test]
    fn test_read_primary_key() -> Result<(), GdtfError> {
        assert_eq!(activation_group_testdata(1), T::read_primary_key_from_xml(&activation_group_testdata_xml(1))?);
        assert_eq!(activation_group_testdata(2), T::read_primary_key_from_xml(&activation_group_testdata_xml(2))?);
        assert_eq!(activation_group_testdata(3), T::read_primary_key_from_xml(&activation_group_testdata_xml(3))?);
        assert_eq!(activation_group_testdata(4), T::read_primary_key_from_xml(&activation_group_testdata_xml(4))?);
        assert_eq!(activation_group_testdata(5), T::read_primary_key_from_xml(&activation_group_testdata_xml(5))?);
        assert_eq!(activation_group_testdata(6), T::read_primary_key_from_xml(&activation_group_testdata_xml(6))?);
        assert_eq!(activation_group_testdata(7), T::read_primary_key_from_xml(&activation_group_testdata_xml(7))?);
        Ok(())
    }

    #[test]
    fn test_read_primary_key_vec() -> Result<(), GdtfError> {
        assert_eq!(activation_group_testdata_vec(), T::read_vec_from_xml(&activation_group_testdata_xml_group())?);
        assert_eq!(T::read_vec_from_xml(&activation_group_teatdata_xml_group_empty())?, vec![]);
        Ok(())
    }

    ///Returns 7 different activation group names for testing
    pub(crate) fn activation_group_testdata(i: u8) -> Name {
        match i {
            1 => Name::new("ColorRGB").unwrap(),
            2 => Name::new("PanTilt").unwrap(),
            3 => Name::new("Gobo1").unwrap(),
            4 => Name::new("ColorIndirect").unwrap(),
            5 => Name::new("Gobo2").unwrap(),
            6 => Name::new("Prism").unwrap(),
            _ => Name::new("").unwrap()
        }
    }

    ///Returns a vec of names for testing
    pub(crate) fn activation_group_testdata_vec() -> Vec<Name> {
        vec![
            activation_group_testdata(1),
            activation_group_testdata(2),
            activation_group_testdata(3),
            activation_group_testdata(4),
            activation_group_testdata(5),
            activation_group_testdata(6),
            activation_group_testdata(7),
        ]
    }


    ///Returns 7 different activation group xmls for testing
    pub(crate) fn activation_group_testdata_xml(i: u8) -> String {
        match i {
            1 => r#"<ActivationGroup Name="ColorRGB"/>"#.to_string(),
            2 => r#"<ActivationGroup Name="PanTilt"/>"#.to_string(),
            3 => r#"<ActivationGroup Name="Gobo1"/>"#.to_string(),
            4 => r#"<ActivationGroup Name="ColorIndirect"/>"#.to_string(),
            5 => r#"<ActivationGroup Name="Gobo2"/>"#.to_string(),
            6 => r#"<ActivationGroup Name="Prism"/>"#.to_string(),
            _ => r#"<ActivationGroup Name="""#.to_string()
        }
    }

    ///Returns an xml with 7 different ActivationGroup nodes inside one activationGroup
    pub(crate) fn activation_group_testdata_xml_group() -> String {
        r#"
    <ActivationGroups>
        <ActivationGroup Name="ColorRGB"/>
        <ActivationGroup Name="PanTilt"/>
        <ActivationGroup Name="Gobo1"/>
        <ActivationGroup Name="ColorIndirect"/>
        <ActivationGroup Name="Gobo2"/>
        <ActivationGroup Name="Prism"/>
        <ActivationGroup Name=""/>
    </ActivationGroups>
    "#.to_string()
    }

    ///Returns an xml no nodes inside one activationGroup
    pub(crate) fn activation_group_teatdata_xml_group_empty() -> String {
        r#"
    <ActivationGroups>
    </ActivationGroups>
    "#.to_string()
    }
}