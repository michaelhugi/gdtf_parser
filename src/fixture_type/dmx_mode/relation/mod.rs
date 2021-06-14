use std::fmt::Debug;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::read;
use crate::utils::read::ReadGdtf;
#[cfg(test)]
use crate::utils::read::TestReadGdtf;
use crate::utils::units::name::Name;
use crate::utils::units::node::Node;

///Relation between the master DMX channel and the following logical channel
#[derive(Debug, PartialEq, Clone)]
pub struct Relation {
    ///Link to the master DMX channel; Starting point: DMX mode
    pub master: Node,
    ///Link to the following channel function; Starting point: DMX mode
    pub follower: Node,
    ///Type of the relation; Values: “Multiply”, “Override”
    pub relation_type: RelationType,
}

///Helper struct to hold temporary data during deparsing
#[derive(Default)]
pub(crate) struct RelationDataHolder {
    ///Link to the master DMX channel; Starting point: DMX mode
    pub master: Option<Node>,
    ///Link to the following channel function; Starting point: DMX mode
    pub follower: Option<Node>,
    ///Type of the relation; Values: “Multiply”, “Override”
    pub relation_type: RelationType,
}

impl ReadGdtf for Relation {
    type PrimaryKey = Name;
    type Error = GdtfError;
    type DataHolder = RelationDataHolder;

    const NODE_NAME: &'static [u8] = b"Relation";
    const PARENT_NODE_NAME: &'static [u8] = b"Relations";
    const PRIMARY_KEY_NAME: &'static [u8] = b"Name";
    const ONLY_PRIMARY_KEY: bool = false;

    fn read_any_attribute(data_holder: &mut Self::DataHolder, attr: Attribute<'_>) -> Result<(), Self::Error> {
        match attr.key {
            b"Master" => data_holder.master = Node::new_from_attr(attr)?,
            b"Follower" => data_holder.follower = Node::new_from_attr(attr)?,
            b"Type" => data_holder.relation_type = RelationType::new_from_attr(attr),
            _ => {}
        }
        Ok(())
    }

    fn read_any_child(_: &mut Self::DataHolder, _: &mut Reader<&[u8]>, _: BytesStart<'_>, _: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error> {
        Ok(Self {
            master: data_holder.master.unwrap_or_else(|| Node::new_from_str("?").unwrap().unwrap()),
            follower: data_holder.follower.unwrap_or_else(|| Node::new_from_str("?").unwrap().unwrap()),
            relation_type: data_holder.relation_type,
        })
    }

    fn read_primary_key_from_attr(attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(Some(Name::new_from_attr(attr)?))
    }
}

#[cfg(test)]
impl TestReadGdtf for Relation {
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)> {
        vec![
            (Some(Name::new("MyRelation").unwrap()), Some(Self { master: Node::new_from_str("MyMaster").unwrap().unwrap(), follower: Node::new_from_str("MyFollower").unwrap().unwrap(), relation_type: RelationType::Multiply })),
            (Some(Name::new("MyRelation").unwrap()), Some(Self { master: Node::new_from_str("MyMaster").unwrap().unwrap(), follower: Node::new_from_str("MyFollower").unwrap().unwrap(), relation_type: RelationType::Override })),
            (Some(Name::new("MyRelation").unwrap()), Some(Self { master: Node::new_from_str("?").unwrap().unwrap(), follower: Node::new_from_str("MyFollower").unwrap().unwrap(), relation_type: RelationType::Multiply })),
            (Some(Name::new("MyRelation").unwrap()), Some(Self { master: Node::new_from_str("MyMaster").unwrap().unwrap(), follower: Node::new_from_str("?").unwrap().unwrap(), relation_type: RelationType::Multiply })),
            (Some(Name::new("MyRelation").unwrap()), Some(Self { master: Node::new_from_str("MyMaster").unwrap().unwrap(), follower: Node::new_from_str("MyFollower").unwrap().unwrap(), relation_type: RelationType::Multiply })),
        ]
    }

    fn testdatas_xml() -> Vec<String> {
        vec![
            r#"<Relation Name="MyRelation" Master="MyMaster" Follower="MyFollower" Type="Multiply"/>"#.to_string(),
            r#"<Relation Name="MyRelation" Master="MyMaster" Follower="MyFollower" Type="Override"></Relation>"#.to_string(),
            r#"<Relation Name="MyRelation"  Follower="MyFollower" Type="Multiply"/>"#.to_string(),
            r#"<Relation Name="MyRelation" Master="MyMaster"  Type="Multiply"/>"#.to_string(),
            r#"<Relation Name="MyRelation" Master="MyMaster" Follower="MyFollower"/>"#.to_string(),
        ]
    }

    fn testdatas_xml_faulty() -> Vec<String> {
        vec![]
    }
}


//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// Start of RelationType
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq, Clone)]
///Type of the relation; Values: “Multiply”, “Override”
pub enum RelationType {
    Multiply,
    Override,
}

impl RelationType {
    /// Parses a str provided by gdtf-xml-description to RelationType
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::relation::RelationType;
    /// assert_eq!(RelationType::Override, RelationType::new_from_str("Override"));
    /// assert_eq!(RelationType::Multiply, RelationType::new_from_str("Multiply"));
    /// assert_eq!(RelationType::Multiply, RelationType::new_from_str("Anything else"));
    /// ```
    pub fn new_from_str(value: &str) -> Self {
        match value {
            "Override" => RelationType::Override,
            _ => RelationType::Multiply
        }
    }
    /// Parses a quick-xml-attribute provided by gdtf-xml-description to RelationType
    /// ```rust
    /// use gdtf_parser::fixture_type::dmx_mode::relation::RelationType;
    /// use quick_xml::events::attributes::Attribute;
    /// use std::borrow::Cow;
    /// assert_eq!(RelationType::Override, RelationType::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Override")}));
    /// assert_eq!(RelationType::Multiply, RelationType::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Multiply")}));
    /// assert_eq!(RelationType::Multiply, RelationType::new_from_attr(Attribute{ key: &[], value: Cow::Borrowed(b"Anything else")}));
    /// ```
    pub fn new_from_attr(attr: Attribute<'_>) -> Self {
        Self::new_from_str(read::attr_to_str(&attr))
    }
}

/// ```rust
/// use gdtf_parser::fixture_type::dmx_mode::relation::RelationType;
/// assert_eq!(RelationType::Multiply, Default::default());
/// ```
impl Default for RelationType {
    fn default() -> Self {
        RelationType::Multiply
    }
}

//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------
// End of RelationType
//-----------------------------------------------------------------------------------------------------------------
//-----------------------------------------------------------------------------------------------------------------


#[cfg(test)]
mod tests {
    use crate::fixture_type::dmx_mode::relation::{Relation, RelationType};
    use crate::utils::read::TestReadGdtf;
    use crate::utils::testdata;

    #[test]
    fn test_deparse() {
        Relation::execute_tests();
    }

    #[test]
    fn test_relation_type_new_from_str() {
        assert_eq!(RelationType::Override, RelationType::new_from_str("Override"));
        assert_eq!(RelationType::Multiply, RelationType::new_from_str("Multiply"));
        assert_eq!(RelationType::Multiply, RelationType::new_from_str("Anything else"));
    }

    #[test]
    fn test_relation_type_new_from_attr_owned() {
        assert_eq!(RelationType::Override, RelationType::new_from_attr(testdata::to_attr_owned(b"Override")));
        assert_eq!(RelationType::Multiply, RelationType::new_from_attr(testdata::to_attr_owned(b"Multiply")));
        assert_eq!(RelationType::Multiply, RelationType::new_from_attr(testdata::to_attr_owned(b"Anything else")));
    }

    #[test]
    fn test_relation_type_new_from_attr_borrowed() {
        assert_eq!(RelationType::Override, RelationType::new_from_attr(testdata::to_attr_borrowed(b"Override")));
        assert_eq!(RelationType::Multiply, RelationType::new_from_attr(testdata::to_attr_borrowed(b"Multiply")));
        assert_eq!(RelationType::Multiply, RelationType::new_from_attr(testdata::to_attr_borrowed(b"Anything else")));
    }

    #[test]
    fn test_relation_type_default() {
        assert_eq!(RelationType::Multiply, Default::default());
    }
}