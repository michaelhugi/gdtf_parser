use std::collections::HashMap;
///! Module contains traits that can be implemented to simpler deparse structs from quick-xml without serde to have full control of the flow
use std::fmt::Debug;
use std::hash::Hash;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::deparse::GdtfDeparseError;

///Trait to store data during deparsing from xml in a mutable and Optional way. All Data will be moved from this struct to the actual Self in the last step.
pub(crate) trait ReadGdtfDataHolder<T: ReadGdtf<Self>>: Default {
    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME: &'static [u8] = T::NODE_NAME;
    ///The name of the parent node
    const PARENT_NODE_NAME: &'static [u8] = T::PARENT_NODE_NAME;

    /// Function should be overridden if needed.
    /// Is called when an attribute is found in the xml tree. Usually this method contains a match statement that checks attr.key.
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn read_any_attribute(&mut self, _attr: Attribute<'_>) -> Result<(), T::Error> {
        Ok(())
    }
    /// Function should be overridden if needed.
    /// Is callen when a child node is found in the xml tree. This method should call `read_single_from_event` on the right child
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn read_any_child(&mut self, _reader: &mut Reader<&[u8]>, _event: BytesStart<'_>, _has_children: bool) -> Result<(), T::Error> {
        Ok(())
    }
    /// Validates the DataHolder and puts it's data into the struct
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    fn move_data(self) -> Result<T, T::Error>;
}

///Trait to deparse an xml-node to a struct
pub(crate) trait ReadGdtf<DataHolder: ReadGdtfDataHolder<Self>>: std::fmt::Debug + Sized + PartialEq {
    ///The primary-key of the struct if used in a hash-map or () if no primary key present
    /// A PrimaryKey should be unique across all xml-nodes of the same type in one GDTF file
    type PrimaryKey: Eq + Hash + Debug + Clone;
    ///Type of error returned in case of failure on deparse
    type Error: From<GdtfDeparseError> + From<quick_xml::Error> + std::error::Error;

    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME: &'static [u8];
    ///The name of the parent node. Declare it as b"GDTF" for example.
    const PARENT_NODE_NAME: &'static [u8];
    ///The name of the primary_key (b"Name" for example) if present, else b""
    const PRIMARY_KEY_NAME: &'static [u8];
    ///If true the struct won't be deparsed but only the method read_primary_key_from_attr will be executed
    const ONLY_PRIMARY_KEY: bool;

    ///Returns NODE_NAME as String
    fn node_name() -> String {
        std::str::from_utf8(Self::NODE_NAME).unwrap().to_string()
    }

    ///Returns PARENT_NODE_NAME as String
    fn parent_node_name() -> String {
        std::str::from_utf8(Self::PARENT_NODE_NAME).unwrap().to_string()
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
    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(Option<Self::PrimaryKey>, Self), Self::Error> {
        let mut data_holder: DataHolder = Default::default();
        let mut primary_key = None;
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == Self::PRIMARY_KEY_NAME {
                primary_key = Self::read_primary_key_from_attr(attr)?;
            } else {
                data_holder.read_any_attribute(attr)?;
            }
        }
        if has_children {
            let mut buf: Vec<u8> = Vec::new();
            loop {
                match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                    Event::Start(e) => {
                        data_holder.read_any_child(reader, e, true)?;
                    }
                    Event::Empty(e) => {
                        data_holder.read_any_child(reader, e, false)?;
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

        Ok((primary_key, data_holder.move_data()?))
    }

    /// Function should be overridden if needed.
    /// Returns the primary_key of the node if the node hase a primary_key, else it returns none
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    //
    fn read_primary_key_from_attr(_attr: Attribute<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        Ok(None)
    }

    /// Can be used if a node only has a primary key and no children, so the struct is not required to be instanced during deparsing
    ///
    /// # Arguments
    ///
    /// * `event` - The Event that was triggering the overlaying struct to go down the tree one step further
    /// # Returns
    ///
    /// * `PrimaryKey` - The primary key as only attribute hold in the node
    fn read_primary_key_from_event(event: BytesStart<'_>) -> Result<Option<Self::PrimaryKey>, Self::Error> {
        for attr in event.attributes().into_iter() {
            let attr = attr?;
            if attr.key == Self::PRIMARY_KEY_NAME {
                return Self::read_primary_key_from_attr(attr);
            }
        }
        Err(GdtfDeparseError::new_xml_attribute_not_found(Self::NODE_NAME, Self::PRIMARY_KEY_NAME).into())
    }

    /// ⚠️**This function can only be used on the event of the parent-struct if the parent-struct only has one kind of children. It will consume all children!** ⚠️
    ///
    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected. If a list of Nodes has a primary-key they most likely are stored in a hash-map. This event returns this hashmap directly to avoid further memory allocation.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `event` - The event that triggered call of this method must be the start or empty event of `PARENT_NODE_NAME`. If this is not possible, the map must be handled manually in read_one_child entry by entry.
    ///
    /// # Returns
    ///
    /// * `HashMap` - The hashmap containing the structs with key as it's primary-key
    fn read_hash_map_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where Self: Sized {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_hash_map_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the map manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }
        let mut buf: Vec<u8> = Vec::new();
        let mut out: HashMap<Self::PrimaryKey, Self> = HashMap::new();

        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME {
                        let val = Self::read_single_from_event(reader, e, true)?;
                        if val.0.is_some() {
                            out.insert(val.0.unwrap(), val.1);
                        }
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        let val = Self::read_single_from_event(reader, e, false)?;
                        if val.0.is_some() {
                            out.insert(val.0.unwrap(), val.1);
                        }
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
    ///
    /// # Returns
    ///
    /// * `Vec<Self>` - All structs found by xml-nodes in a vec
    fn read_vec_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<Vec<Self>, Self::Error> where Self: Sized {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_vec_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the vec manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME {
                        out.push(Self::read_single_from_event(reader, e, true)?.1);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        out.push(Self::read_single_from_event(reader, e, false)?.1);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                Event::Eof => {
                    break;
                }
                _ => {}
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
   ///
   /// # Returns
   ///
   /// * `Vec<Self::PrimaryKey>` - All PrimaryKeys found by xml-nodes in a vec
    fn read_primary_key_vec_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>) -> Result<Vec<Self::PrimaryKey>, Self::Error> where Self: Sized {
        if event.name() != Self::PARENT_NODE_NAME {
            panic!("Wrong call of read_vec_from_event for node {}. This method can only be used if you have an empty {}. If this is not empty, fill the vec manually in your read_one_child() entry by entry.", Self::node_name(), Self::parent_node_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self::PrimaryKey> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        out.push(Self::read_primary_key_from_event(e)?.ok_or_else(|| GdtfDeparseError::new_xml_attribute_not_found(Self::NODE_NAME, Self::PRIMARY_KEY_NAME))?);
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
        Ok(out)
    }
}

#[cfg(test)]
///Trait only compiled in testing. Offers testing of different stuff
pub(crate) trait TestReadSingle<DataHolder: ReadGdtfDataHolder<Self>>: ReadGdtf<DataHolder> {
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
            panic!("The node {} did not declare a PRIMARY_KEY_NAME", Self::node_name());
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
        if Self::PRIMARY_KEY_NAME != b"" {
            panic!("The node {} did declare a PRIMARY_KEY_NAME and can't be used for testdata_primary_key_vec()", Self::node_name())
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
            panic!("testdatas_xml() and testdatas() do not return the same length for node {}", Self::node_name());
        }

        for xml in xmls {
            let s1 = Self::read_single_from_xml(&xml).unwrap();
            let s2 = structs.next().unwrap();
            if Self::PRIMARY_KEY_NAME != b"" {
                assert_eq!(s1.0.unwrap(), s2.0.unwrap());
            }
            assert_eq!(s1.1, s2.1.unwrap());
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_vec() {
        let xml = format!("{}{}{}", Self::parent_node_start_xml(), Self::testdata_xml(), Self::parent_node_end_xml());
        assert_eq!(Self::read_vec_from_xml(&xml).unwrap(), Self::testdata_vec())
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_hash_map() {
        let xml = format!("{}{}{}", Self::parent_node_start_xml(), Self::testdata_xml(), Self::parent_node_end_xml());
        assert_eq!(Self::read_hash_map_from_xml(&xml).unwrap(), Self::testdata_hash_map())
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if all values returned by `testdatas_xml_faulty()` will return an error. This is useful for required values without default.
    fn execute_test_faulty() {
        for xml in Self::testdatas_xml_faulty().iter() {
            assert!(Self::read_single_from_xml(xml).is_err());
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if all values returned by `testdatas_xml_faulty()` will return an error when only a primary_key and no children are present in a node. This is useful for required values without default.
    fn execute_test_primary_key_faulty() {
        for xml in Self::testdatas_xml_faulty().iter() {
            assert!(Self::read_primary_key_from_xml(xml).is_err());
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function tests if deparsing a node that only contains the primary-key and no children works as expected entry_by_entry
    fn execute_test_read_primary_key_single() {
        let xmls = Self::testdatas_xml().into_iter();
        let mut structs = Self::testdatas().into_iter();
        if xmls.len() != structs.len() {
            panic!("testdatas_xml() and testdatas() do not return the same length for node {}", Self::node_name());
        }

        for xml in xmls {
            let s1 = Self::read_single_from_xml(&xml).unwrap();
            let s2 = structs.next().unwrap();
            assert_eq!(s1.0.unwrap(), s2.0.unwrap());
        }
    }

    /// Don't call manually! `Use execute_tests()` instead. This function will check all values returned by `testdatas()` and `testdatas_xml()` and compare them while joined together and parsed in one shot. This additional test is important to detect if a child consumes an event that shouldn't be consumed. This would not be detecteble by `execute_test_read_single()`
    fn execute_test_read_primary_key_vec() {
        let xml = format!("{}{}{}", Self::parent_node_start_xml(), Self::testdata_xml(), Self::parent_node_end_xml());
        assert_eq!(Self::read_primary_key_vec_from_xml(&xml).unwrap(), Self::testdata_primary_key_vec())
    }

    /// Main function to run all tests for deparsing the node
    fn execute_tests() {
        if !Self::ONLY_PRIMARY_KEY {
            Self::execute_test_faulty();
            Self::execute_test_read_single();
            if Self::PARENT_NODE_NAME != b"" {
                if Self::PRIMARY_KEY_NAME == b"" {
                    Self::execute_test_read_vec();
                } else {
                    Self::execute_test_read_hash_map();
                }
            }
        } else {
            Self::execute_test_primary_key_faulty();
            Self::execute_test_read_primary_key_single();
            Self::execute_test_read_primary_key_vec();
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
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }


    /// Helper function for testing deparsing  multiple nodes as hash map from an xml
    fn read_hash_map_from_xml(xml: &str) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_hash_map_from_event(&mut reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    /// Helper function for testing deparsing  multiple nodes as vec from an xml
    fn read_vec_from_xml(xml: &str) -> Result<Vec<Self>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_vec_from_event(&mut reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    /// Helper function for testing deparsing a single node that only contains primary-key and no childen to just the primary-key from an xml
    fn read_primary_key_from_xml(xml: &str) -> Result<Self::PrimaryKey, Self::Error> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Ok(Self::read_primary_key_from_event(e)?.ok_or(GdtfDeparseError::new_xml_attribute_not_found(Self::NODE_NAME, Self::PRIMARY_KEY_NAME))?);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    /// Helper function for testing deparsing multiple nodes that only contain primary keys and no children as a vec of primary-keys from an xml
    fn read_primary_key_vec_from_xml(xml: &str) -> Result<Vec<Self::PrimaryKey>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_primary_key_vec_from_event(&mut reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }
}

