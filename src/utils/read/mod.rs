use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
///! Module contains traits that can be implemented to simpler deparse structs from quick-xml without serde to have full control of the flow
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;

///Trait to deparse an xml-node to a struct
pub(crate) trait ReadGdtf: std::fmt::Debug + Sized + PartialEq {
    ///The primary-key of the struct if used in a hash-map or () if no primary key present
    /// A PrimaryKey should be unique across all xml-nodes of the same type in one GDTF file
    type PrimaryKey: Eq + Hash + Debug + Clone;
    ///Type of error returned in case of failure on deparse
    type Error: From<GdtfReadError> + From<quick_xml::Error> + std::error::Error;
    ///The struct type to hold data temprary during deparse. Can be the struct that implements `ReadGdtf` itself or a similar struct with more `Option` fields
    type DataHolder: Default;
    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME: &'static [u8];
    ///The name of the parent node. Declare it as b"GDTF" for example.
    const PARENT_NODE_NAME: &'static [u8];
    ///The name of the primary_key (b"Name" for example) if present, else b""
    const PRIMARY_KEY_NAME: &'static [u8];
    ///If true the struct won't be deparsed but only the method read_primary_key_from_attr will be executed
    const ONLY_PRIMARY_KEY: bool;

    /// Is called when an attribute is found in the xml tree. Usually this method contains a match statement that checks attr.key.
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn read_any_attribute(
        data_holder: &mut Self::DataHolder,
        attr: Attribute<'_>,
    ) -> Result<(), Self::Error>;

    /// Is callen when a child node is found in the xml tree. This method should call `read_single_from_event` on the right child
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn read_any_child(
        data_holder: &mut Self::DataHolder,
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(), Self::Error>;

    /// Validates the DataHolder and puts it's data into the struct
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn move_data(data_holder: Self::DataHolder) -> Result<Self, Self::Error>;

    ///Function to return an error when an xml-attribute is missing
    fn attribute_not_found(attribute_name: &[u8]) -> GdtfReadError {
        GdtfReadError::new_xml_attribute_not_found(Self::NODE_NAME, attribute_name)
    }

    ///Function to return an error when a child node is missing
    fn child_not_found(child_name: &[u8]) -> GdtfReadError {
        GdtfReadError::new_xml_node_not_found(Self::NODE_NAME, child_name)
    }

    ///Function to return an error when a child misses a required primary_key
    fn child_primary_key_not_found(
        child_name: &[u8],
        child_primary_key_name: &[u8],
    ) -> GdtfReadError {
        GdtfReadError::new_xml_node_not_found(child_name, child_primary_key_name)
    }

    ///Returns NODE_NAME as String
    fn node_name() -> String {
        std::str::from_utf8(Self::NODE_NAME).unwrap().to_string()
    }

    ///Returns PARENT_NODE_NAME as String
    fn parent_node_name() -> String {
        std::str::from_utf8(Self::PARENT_NODE_NAME)
            .unwrap()
            .to_string()
    }

    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `event` - The Event that was triggering the overlaying struct to go down the tree one step further
    /// * `has_children` - **If true, reader.read_event() is not allowed to be executed** True if the node is formatted like <Node></Node> so the closing node will show up, false if the node is formatted like <Node/>. In that case checking the next event will consume the wrong event.
    ///
    /// # Returns
    ///
    /// * `Self` - The struct deparsed from the reader
    /// * `Self::PrimaryKey` - If the struct has a primary key (to use in a hashmap for example) it will be returned here
    fn read_single_from_event(
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<(Option<Self::PrimaryKey>, Self), Self::Error> {
        let mut data_holder: Self::DataHolder = Default::default();
        let mut primary_key = None;
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == Self::PRIMARY_KEY_NAME {
                primary_key = Self::read_primary_key_from_attr(attr)?;
            } else {
                Self::read_any_attribute(&mut data_holder, attr)?;
            }
        }
        if has_children {
            let mut buf: Vec<u8> = Vec::new();
            loop {
                match reader
                    .read_event(&mut buf)
                    .map_err(GdtfReadError::QuickXmlError)?
                {
                    Event::Start(e) => {
                        Self::read_any_child(&mut data_holder, reader, e, true)?;
                    }
                    Event::Empty(e) => {
                        Self::read_any_child(&mut data_holder, reader, e, false)?;
                    }
                    Event::End(e) => {
                        if e.name() == Self::NODE_NAME {
                            break;
                        }
                    }
                    Event::Eof => {
                        break;
                    }
                    _ => {}
                }
            }
        }

        Ok((primary_key, Self::move_data(data_holder)?))
    }

    /// Returns the primary_key of the node if the node hase a primary_key, else it returns none
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    //
    fn read_primary_key_from_attr(
        attr: Attribute<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error>;

    /// Can be used if a node only has a primary key and no children, so the struct is not required to be instanced during deparsing
    ///
    /// # Arguments
    ///
    /// * `event` - The Event that was triggering the overlaying struct to go down the tree one step further
    /// # Returns
    ///
    /// * `PrimaryKey` - The primary key as only attribute hold in the node
    fn read_primary_key_from_event(
        event: BytesStart<'_>,
    ) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == Self::PRIMARY_KEY_NAME {
                return Self::read_primary_key_from_attr(attr);
            }
        }
        Err(
            GdtfReadError::new_xml_attribute_not_found(Self::NODE_NAME, Self::PRIMARY_KEY_NAME)
                .into(),
        )
    }

    /// ⚠️**This function can only be used on the event of the parent-struct if the parent-struct only has one kind of children. It will consume all children!** ⚠️
    ///
    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected. If a list of Nodes has a primary-key they most likely are stored in a hash-map. This event returns this hashmap directly to avoid further memory allocation.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `event` - The event that triggered call of this method must be the start or empty event of `PARENT_NODE_NAME`. If this is not possible, the map must be handled manually in read_one_child entry by entry.
    /// * `has_children` - **If true, reader.read_event() is not allowed to be executed** True if the node is formatted like <Node></Node> so the closing node will show up, false if the node is formatted like <Node/>. In that case checking the next event will consume the wrong event.
    ///
    /// # Returns
    ///
    /// * `HashMap` - The hashmap containing the structs with key as it's primary-key
    fn read_hash_map_from_event(
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error>
    where
        Self: Sized,
    {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_hash_map_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the map manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }
        let mut buf: Vec<u8> = Vec::new();
        let mut out: HashMap<Self::PrimaryKey, Self> = HashMap::new();

        if has_children {
            loop {
                match reader
                    .read_event(&mut buf)
                    .map_err(GdtfReadError::QuickXmlError)?
                {
                    Event::Start(e) => {
                        if e.name() == Self::NODE_NAME {
                            let val = Self::read_single_from_event(reader, e, true)?;
                            out.insert(
                                val.0.ok_or_else(|| {
                                    Self::child_primary_key_not_found(
                                        Self::NODE_NAME,
                                        Self::PRIMARY_KEY_NAME,
                                    )
                                })?,
                                val.1,
                            );
                        }
                    }
                    Event::Empty(e) => {
                        if e.name() == Self::NODE_NAME {
                            let val = Self::read_single_from_event(reader, e, false)?;
                            out.insert(
                                val.0.ok_or_else(|| {
                                    Self::child_primary_key_not_found(
                                        Self::NODE_NAME,
                                        Self::PRIMARY_KEY_NAME,
                                    )
                                })?,
                                val.1,
                            );
                        }
                    }
                    Event::End(e) => {
                        if e.name() == Self::PARENT_NODE_NAME {
                            break;
                        }
                    }
                    Event::Eof => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(out)
    }

    /// ⚠️**This function can only be used on the event of the parent-struct if the parent-struct only has one kind of children. It will consume all children!** ⚠️
    ///
    /// If an xml-node is listed multiple times but has no unique attribute accross all these nodes (`PrimaryKey`), this function can be used.
    /// This function will go down the tree and deparse all nodes with the name eq to `NODE_NAME` in a vec and return when it's back up at it's position.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `event` - The event that triggered call of this method must be the start or empty event of `PARENT_NODE_NAME`. If this is not possible, the vec must be handled manually in read_one_child entry by entry.
    /// * `has_children` - **If true, reader.read_event() is not allowed to be executed** True if the node is formatted like <Node></Node> so the closing node will show up, false if the node is formatted like <Node/>. In that case checking the next event will consume the wrong event.
    ///
    /// # Returns
    ///
    /// * `Vec<Self>` - All structs found by xml-nodes in a vec
    fn read_vec_from_event(
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<Vec<Self>, Self::Error>
    where
        Self: Sized,
    {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_vec_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the vec manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        if has_children {
            loop {
                match reader
                    .read_event(&mut buf)
                    .map_err(GdtfReadError::QuickXmlError)?
                {
                    Event::Start(e) => {
                        if e.name() == Self::NODE_NAME {
                            out.push(Self::read_single_from_event(reader, e, true)?.1);
                        }
                    }
                    Event::Empty(e) => {
                        if e.name() == Self::NODE_NAME {
                            out.push(Self::read_single_from_event(reader, e, false)?.1);
                        }
                    }
                    Event::End(e) => {
                        if e.name() == Self::PARENT_NODE_NAME {
                            break;
                        }
                    }
                    Event::Eof => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(out)
    }

    /// ⚠️**This function can only be used on the event of the parent-struct if the parent-struct only has one kind of children. It will consume all children!** ⚠️
    ///
    /// If a xml-node only contains one primary key and no children, this method can be used to avoid instancing unnessecary structs.
    /// This function will go down the tree and deparse all node's primary-keys with the name eq to `NODE_NAME` in a vec and return when it's back up at it's position.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `event` - The event that triggered call of this method must be the start or empty event of `PARENT_NODE_NAME`. If this is not possible, the vec must be handled manually in read_one_child entry by entry.
    /// * `has_children` - **If true, reader.read_event() is not allowed to be executed** True if the node is formatted like <Node></Node> so the closing node will show up, false if the node is formatted like <Node/>. In that case checking the next event will consume the wrong event.
    ///
    /// # Returns
    ///
    /// * `Vec<Self::PrimaryKey>` - All PrimaryKeys found by xml-nodes in a vec
    fn read_primary_key_vec_from_event(
        reader: &mut Reader<&[u8]>,
        event: BytesStart<'_>,
        has_children: bool,
    ) -> Result<Vec<Self::PrimaryKey>, Self::Error>
    where
        Self: Sized,
    {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_vec_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the vec manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self::PrimaryKey> = Vec::new();
        if has_children {
            loop {
                match reader
                    .read_event(&mut buf)
                    .map_err(GdtfReadError::QuickXmlError)?
                {
                    Event::Start(e) | Event::Empty(e) => {
                        if e.name() == Self::NODE_NAME {
                            out.push(Self::read_primary_key_from_event(e)?.ok_or_else(|| {
                                GdtfReadError::new_xml_attribute_not_found(
                                    Self::NODE_NAME,
                                    Self::PRIMARY_KEY_NAME,
                                )
                            })?);
                        }
                    }
                    Event::End(e) => {
                        if e.name() == Self::PARENT_NODE_NAME {
                            break;
                        }
                    }
                    Event::Eof => {
                        break;
                    }
                    _ => {}
                }
            }
        }
        Ok(out)
    }
}

#[cfg(test)]
///Trait only compiled in testing. Offers testing of different stuff
pub(crate) trait TestReadGdtf: ReadGdtf {
    /// Should return a vec of structs to be tested.
    /// * If a struct has only a primary-key, only the first part of the tuple must be returned.
    /// * If a struct has primary-key and (attributes or children) both parts of the tuple are required
    /// * If a struct has no primary-key, only the second part of the tuple is required
    fn testdatas() -> Vec<(Option<Self::PrimaryKey>, Option<Self>)>;

    /// Should return different xmls that will be parsed to structs and compared to the ones returned by `testdatas()`
    fn testdatas_xml() -> Vec<String>;

    /// Returns a list of xmls that will return an error when deparsing. (Missing required value for example)
    fn testdatas_xml_faulty() -> Vec<String>;

    /// Joins all xml returned by `testdatas()` and joins them to one xml without parent node. This is useful for returning testvalues from parent node.
    fn testdata_xml() -> String {
        let mut out = "".to_string();
        for xml in Self::testdatas_xml().into_iter() {
            out = format!("{}{}", out, xml);
        }
        out
    }

    /// If the node has a primary_key, this method will return a hashmap of testvalues returned by `testdatas()` with the primary_key as the hash-map's key. This is useful for avoiding duplicate coding in testing of parent nodes.
    fn testdata_hash_map() -> HashMap<Self::PrimaryKey, Self> {
        if Self::PRIMARY_KEY_NAME == b"" {
            panic!(
                "The node {} did not declare a PRIMARY_KEY_NAME",
                Self::node_name()
            );
        }
        let mut map: HashMap<Self::PrimaryKey, Self> = Default::default();
        for tup in Self::testdatas().into_iter() {
            map.insert(tup.0.unwrap(), tup.1.unwrap());
        }
        map
    }

    /// If the node has no primary_key, this method will return a vec of testvalues returned by `testdatas()`. This is useful for avoiding duplicate coding in testing of parent nodes.
    fn testdata_vec() -> Vec<Self> {
        if Self::PRIMARY_KEY_NAME != b"" {
            panic!("The node {} did declare a PRIMARY_KEY_NAME and should not be used in a vec but a HashMap", Self::node_name())
        }
        let mut v: Vec<Self> = Default::default();

        for tup in Self::testdatas().into_iter() {
            v.push(tup.1.unwrap());
        }
        v
    }

    /// If the node only has a primary_key and no children, this method will return a vec of testvalues returned by `testdatas()`. This is useful for avoiding duplicate coding in testing of parent nodes.
    fn testdata_primary_key_vec() -> Vec<Self::PrimaryKey> {
        if Self::PRIMARY_KEY_NAME == b"" {
            panic!("The node {} did not declare a PRIMARY_KEY_NAME and can't be used for testdata_primary_key_vec()", Self::node_name())
        }
        let mut v: Vec<Self::PrimaryKey> = Default::default();

        for tup in Self::testdatas().into_iter() {
            v.push(tup.0.unwrap());
        }
        v
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them one by one.
    fn execute_test_read_single() {
        let xmls = Self::testdatas_xml().into_iter();
        let mut structs = Self::testdatas().into_iter();
        if xmls.len() != structs.len() {
            panic!(
                "testdatas_xml() and testdatas() do not return the same length for node {}",
                Self::node_name()
            );
        }

        for (i, xml) in xmls.enumerate() {
            let s1 = match Self::read_single_from_xml(&xml) {
                Ok(s1) => s1,
                Err(e) => panic!(
                    "execute_test_read_single failed nr {} in {}: {}",
                    i,
                    Self::node_name(),
                    e
                ),
            };
            let s2 = match structs.next() {
                None => panic!(
                    "testdatas_xml() and testdatas() do not return the same length for node {}",
                    Self::node_name()
                ),
                Some(s2) => s2,
            };
            if Self::PRIMARY_KEY_NAME != b"" {
                let s1 = match s1.0 {
                    None => panic!(
                        "execute_test_read_single did not return primary_key from xml nr {} in {}",
                        i,
                        Self::node_name()
                    ),
                    Some(s1) => s1,
                };
                let s2 = match s2.0 {
                    None => panic!("execute_test_read_single did not return primary_key from testdata nr {} in {}", i, Self::node_name()),
                    Some(s2) => s2
                };
                if s1 != s2 {
                    panic!("execute_test_read_single Primary_keys were not equal nr {} in {}\n  left: {:?}\n right: {:?}", i, Self::node_name(), s1, s2);
                }
                assert_eq!(s1, s2);
            }
            let s1 = s1.1;
            let s2 = match s2.1 {
                None => panic!(
                    "execute_test_read_single testdata provided none nr {} in {}",
                    i,
                    Self::node_name()
                ),
                Some(s2) => s2,
            };
            if s1 != s2 {
                panic!("execute_test_read_single structs were not equal nr {} in {}\n  left: {:?}\n right: {:?}", i, Self::node_name(), s1, s2);
            }
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_vec() {
        let xml = format!(
            "{}{}{}",
            Self::parent_node_start_xml(),
            Self::testdata_xml(),
            Self::parent_node_end_xml()
        );
        let left = match Self::read_vec_from_xml(&xml) {
            Ok(left) => left,
            Err(e) => panic!(
                "execute_test_read_vec failed in {}: {}",
                Self::node_name(),
                e
            ),
        };
        let right = Self::testdata_vec();
        if left != right {
            panic!(
                "execute_test_read_vec were not equal in {}\n  left: {:?}\n right: {:?}",
                Self::node_name(),
                left,
                right
            );
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_hash_map() {
        let xml = format!(
            "{}{}{}",
            Self::parent_node_start_xml(),
            Self::testdata_xml(),
            Self::parent_node_end_xml()
        );
        let left = match Self::read_hash_map_from_xml(&xml) {
            Ok(left) => left,
            Err(e) => panic!(
                "execute_test_read_hash_map failed in {}: {}",
                Self::node_name(),
                e
            ),
        };
        let right = Self::testdata_hash_map();
        if left != right {
            panic!(
                "execute_test_read_hash_map were not equal in {}\n  left: {:?}\n right: {:?}",
                Self::node_name(),
                left,
                right
            );
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if all values returned by `testdatas_xml_faulty()` will return an error. This is useful for required values without default.
    fn execute_test_faulty() {
        for (pos, xml) in Self::testdatas_xml_faulty().iter().enumerate() {
            if !Self::read_single_from_xml(xml).is_err() {
                panic!(
                    "execute_test_faulty nr {} was no error in {}",
                    pos,
                    Self::node_name()
                );
            }
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if all values returned by `testdatas_xml_faulty()` will return an error when only a primary_key and no children are present in a node. This is useful for required values without default.
    fn execute_test_primary_key_faulty() {
        for (pos, xml) in Self::testdatas_xml_faulty().iter().enumerate() {
            if !Self::read_primary_key_from_xml(xml).is_err() {
                panic!(
                    "execute_test_primary_key_faulty nr {} was no error in {}",
                    pos,
                    Self::node_name()
                );
            }
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if deparsing a node that only contains the primary-key and no children works as expected entry_by_entry
    fn execute_test_read_primary_key_single() {
        let xmls = Self::testdatas_xml().into_iter();
        let mut structs = Self::testdatas().into_iter();
        if xmls.len() != structs.len() {
            panic!(
                "testdatas_xml() and testdatas() do not return the same length for node {}",
                Self::node_name()
            );
        }

        for xml in xmls {
            let s1 = Self::read_primary_key_from_xml(&xml).unwrap();
            let s2 = structs.next().unwrap();
            assert_eq!(s1, s2.0.unwrap());
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_primary_key_vec() {
        let xml = format!(
            "{}{}{}",
            Self::parent_node_start_xml(),
            Self::testdata_xml(),
            Self::parent_node_end_xml()
        );
        assert_eq!(
            Self::read_primary_key_vec_from_xml(&xml).unwrap(),
            Self::testdata_primary_key_vec()
        )
    }

    /// Main function to run all tests for deparsing the node
    fn execute_tests() {
        if Self::ONLY_PRIMARY_KEY {
            Self::execute_test_primary_key_faulty();
            Self::execute_test_read_primary_key_single();
            Self::execute_test_read_primary_key_vec();
        } else {
            Self::execute_test_faulty();
            Self::execute_test_read_single();
            if Self::PARENT_NODE_NAME != b"" {
                if Self::PRIMARY_KEY_NAME == b"" {
                    Self::execute_test_read_vec();
                } else {
                    Self::execute_test_read_hash_map();
                }
            }
        }
    }

    /// Helper function to wrap xmls in parent nodes for testing deparsing in groups
    fn parent_node_start_xml() -> String {
        format!("<{}>", Self::parent_node_name())
    }

    /// Helper function to wrap xmls in parent nodes for testing deparsing in groups
    fn parent_node_end_xml() -> String {
        format!("</{}>", Self::parent_node_name())
    }

    /// Helper function for testing deparsing a single node from an xml
    fn read_single_from_xml(xml: &str) -> Result<(Option<Self::PrimaryKey>, Self), Self::Error> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader
                .read_event(&mut buf)
                .map_err(GdtfReadError::QuickXmlError)?
            {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Self::read_single_from_event(&mut reader, e, true);
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Self::read_single_from_event(&mut reader, e, false);
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(e) => {
                    if e.name() == Self::NODE_NAME {
                        break;
                    }
                }
                _ => {}
            };
        }
        buf.clear();
        Err(GdtfReadError::new_xml_node_not_found(
            b"TopLevel",
            Self::NODE_NAME,
        ))?
    }

    /// Helper function for testing deparsing  multiple nodes as hash map from an xml
    fn read_hash_map_from_xml(xml: &str) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error>
    where
        Self: Sized,
    {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader
                .read_event(&mut buf)
                .map_err(GdtfReadError::QuickXmlError)?
            {
                Event::Start(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_hash_map_from_event(&mut reader, e, true);
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_hash_map_from_event(&mut reader, e, false);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        break;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {}
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(
            b"TopLevel",
            Self::NODE_NAME,
        ))?
    }

    /// Helper function for testing deparsing  multiple nodes as vec from an xml
    fn read_vec_from_xml(xml: &str) -> Result<Vec<Self>, Self::Error>
    where
        Self: Sized,
    {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader
                .read_event(&mut buf)
                .map_err(GdtfReadError::QuickXmlError)?
            {
                Event::Start(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_vec_from_event(&mut reader, e, true);
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_vec_from_event(&mut reader, e, false);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        break;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {}
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(
            b"TopLevel",
            Self::NODE_NAME,
        ))?
    }

    /// Helper function for testing deparsing a single node that only contains primary-key and no childen to just the primary-key from an xml
    fn read_primary_key_from_xml(xml: &str) -> Result<Self::PrimaryKey, Self::Error> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader
                .read_event(&mut buf)
                .map_err(GdtfReadError::QuickXmlError)?
            {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Ok(Self::read_primary_key_from_event(e)?.ok_or(
                            GdtfReadError::new_xml_attribute_not_found(
                                Self::NODE_NAME,
                                Self::PRIMARY_KEY_NAME,
                            ),
                        )?);
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(e) => {
                    if e.name() == Self::NODE_NAME {
                        break;
                    }
                }
                _ => {}
            };
        }
        buf.clear();
        Err(GdtfReadError::new_xml_node_not_found(
            b"TopLevel",
            Self::NODE_NAME,
        ))?
    }

    /// Helper function for testing deparsing multiple nodes that only contain primary keys and no children as a vec of primary-keys from an xml
    fn read_primary_key_vec_from_xml(xml: &str) -> Result<Vec<Self::PrimaryKey>, Self::Error>
    where
        Self: Sized,
    {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader
                .read_event(&mut buf)
                .map_err(GdtfReadError::QuickXmlError)?
            {
                Event::Start(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_primary_key_vec_from_event(&mut reader, e, true);
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_primary_key_vec_from_event(&mut reader, e, false);
                    }
                }
                Event::End(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        break;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {}
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(
            b"TopLevel",
            Self::NODE_NAME,
        ))?
    }
}

///Error used if an Error in global Deparsing occrus
#[derive(Debug)]
pub enum GdtfReadError {
    ///Error when any error is returned from the underlying qick-xml
    QuickXmlError(quick_xml::Error),
    ///Error when an expected xml-node was not found
    QuickXmlNodeNotFoundError(String, String),
    ///Error when an expected xml-attribute was not found
    QuickXmlAttributeNotFoundError(String, String),
}

impl GdtfReadError {
    ///Constructor for `QuickXmlNodeNotFoundError`
    pub fn new_xml_node_not_found(parent_node_name: &[u8], node_name: &[u8]) -> Self {
        Self::QuickXmlNodeNotFoundError(
            u8_array_to_string(parent_node_name),
            u8_array_to_string(node_name),
        )
    }

    ///Constructor for `QuickXmlAttributeNotFoundError`
    pub fn new_xml_attribute_not_found(node_name: &[u8], attribute_name: &[u8]) -> Self {
        Self::QuickXmlAttributeNotFoundError(
            u8_array_to_string(node_name),
            u8_array_to_string(attribute_name),
        )
    }
}

impl Display for GdtfReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GdtfReadError::QuickXmlError(e) => write!(f, "GdtfDeparseError: {}", e),
            GdtfReadError::QuickXmlAttributeNotFoundError(node_name, attribute) => write!(
                f,
                "Could not find xml-attribute '{}' in '{}'",
                attribute, node_name
            ),
            GdtfReadError::QuickXmlNodeNotFoundError(parent_node_name, node_name) => write!(
                f,
                "Could not find xml-node name '{}' in '{}'",
                node_name, parent_node_name
            ),
        }
    }
}

impl Error for GdtfReadError {}

///Parses an xml-attribute to str but returns "" if any error occurs
pub(crate) fn attr_to_str<'a>(attr: &'a Attribute) -> &'a str {
    std::str::from_utf8(attr.value.borrow()).unwrap_or("")
}

///Parses an xml-attribute to str and returns the error if one occurs
pub(crate) fn attr_try_to_str<'a>(attr: &'a Attribute) -> Result<&'a str, GdtfError> {
    Ok(std::str::from_utf8(attr.value.borrow())?)
}

///Parses an xml-attribute to f32 but returns 0 if any error occurs
pub(crate) fn attr_to_f32(attr: Attribute) -> f32 {
    f32::from_str(attr_try_to_str(&attr).unwrap_or("")).unwrap_or(0_f32)
}

///Parses an xml-attribute to f32 but returns None if any error occurs
pub(crate) fn attr_to_f32_option(attr: Attribute) -> Option<f32> {
    match f32::from_str(attr_try_to_str(&attr).unwrap_or("")) {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

///Parses an xml-attribute to string but returns None if any error occurs or String is empty
pub(crate) fn attr_to_string_option(attr: Attribute) -> Option<String> {
    match attr_try_to_str(&attr).unwrap_or("") {
        "" => None,
        s => Some(s.to_owned()),
    }
}

///Parses an xml-attribute to but returns "" if any error occurs
pub(crate) fn attr_to_string(attr: Attribute) -> String {
    attr_try_to_str(&attr).unwrap_or("").to_owned()
}

///Parses an xml-attribute to u8 but returns None if any error occurs
pub(crate) fn attr_to_u8_option(attr: Attribute) -> Option<u8> {
    match u8::from_str(attr_try_to_str(&attr).unwrap_or("")) {
        Ok(f) => Some(f),
        Err(_) => None,
    }
}

///Helper function to create useful error messages
fn u8_array_to_string(val: &[u8]) -> String {
    std::str::from_utf8(val).map_or("?".to_string(), |e| e.to_string())
}
