///! Module contains traits that can be implemented to simpler deparse structs from quick-xml without serde to have full control of the flow
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::read::GdtfReadError;

///Trait to deparse an xml-node to a struct
pub(crate) trait DeparseSingle: std::fmt::Debug + Sized {
    ///The primary-key of the struct if used in a hash-map or () if no primary key present
    /// A PrimaryKey should be unique across all xml-nodes of the same type in one GDTF file
    type PrimaryKey: Eq + Hash + Debug + Clone;
    ///Type of error returned in case of failure on deparse
    type Error: From<GdtfReadError> + std::error::Error;
    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME_DS: &'static [u8];

    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected.
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
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
    fn read_single_from_event(reader: &mut Reader<&[u8]>, event: BytesStart<'_>, has_children: bool) -> Result<(Option<Self::PrimaryKey>, Self), Self::Error>;
}

///Trait to help testing DeparseSingle
#[cfg(test)]
pub(crate) trait TestDeparseSingle: Debug + PartialEq<Self> + Sized + DeparseSingle {
    /// When a node should be deparsed for testing without deparsing the whole tree above, this method will give an entry point to do so without going trough GDTF-struct.
    /// This function will iterate trough the events of an xml until it matches the `NODE_NAME` given by `DeparseSingle`. At that point it will call `DeparseSinge::read_single_from_event`
    ///
    /// Returns an Error if `DeparseSingle::read_single_from_event` returns an error, if `NODE_NAME` was not found or if xml is not valid
    ///
    /// # Arguments
    ///
    /// * `xml` - The xml that should be deparsed
    fn read_single_from_xml(xml: &str) -> Result<(Option<Self::PrimaryKey>, Self), Self::Error> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME_DS {
                        return Self::read_single_from_event(&mut reader, e, true);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME_DS {
                        return Self::read_single_from_event(&mut reader, e, false);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Eof => {
                    break;
                }
                Event::End(_) => {
                    tree_down -= 1;
                    if tree_down <= 0 {
                        break;
                    }
                }
                _ => {}
            };
        }
        buf.clear();
        Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME_DS))?
    }

    /// Method for testing `DeparseSingle`. On a struct that implements `DeparseSingle` you can call this method to compare it to a deparsed xml.
    ///
    /// Panics in case the values are not the same or when an error occurs
    ///
    /// # Arguments
    ///
    /// * `primary_key` - If the deparsing of the xml should return a `PrimaryKey`, they will be compared. If the struct has no PrimaryKey pass `None`
    /// * `xml` - The xml that will de deparsed and compared to the struct implementing `DeparseSingle` and the `PrimaryKey` if is some.
    fn compare_to_primary_key_and_xml(&self, primary_key: Option<Self::PrimaryKey>, xml: &str) {
        let other = Self::read_single_from_xml(xml).expect(&format!("Unexpected error in test of {}", u8_array_to_string(Self::NODE_NAME_DS))[..]);
        assert_eq!(self, &other.1);
        match (primary_key, other.0) {
            (Some(primary_key), Some(other)) => assert_eq!(primary_key, other),
            (None, None) => {}
            (primary_key, other) => panic!("Primary Keys not equal \nleft: {:?}\nright: {:?}", primary_key, other)
        }
    }
}

///Trait to deparse multiple xml-nodes with PrimaryKeys to a HashMap
pub(crate) trait DeparseHashMap: DeparseSingle {
    ///The name of the node wraps a list of nodes that contain the data for the struct. Declare it as b"GDTF" for example.
    const PARENT_NODE_NAME: &'static [u8];
    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected. If a list of Nodes has a primary-key they most likely are stored in a hash-map. This event returns this hashmap directly to avoid further memory allocation.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    ///
    /// # Returns
    ///
    /// * `HashMap` - The hashmap containing the structs with key as it's primary-key
    fn read_hash_map_from_event(reader: &mut Reader<&[u8]>) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut out: HashMap<Self::PrimaryKey, Self> = HashMap::new();


        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME_DS {
                        let val = Self::read_single_from_event(reader, e, true)?;
                        if val.0.is_some() {
                            out.insert(val.0.unwrap(), val.1);
                        }
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME_DS {
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
}

///Trait to help testing DeparseHashMap
#[cfg(test)]
pub(crate) trait TestDeparseHashMap: DeparseHashMap + TestDeparseSingle {
    /// Reads a hashmap from an xml string slice. The function will dive down the nodes in the xml until it hits `PARENT_NODE_NAME` and then call `DeparseHashMap.read_hash_map_from_event` on it.
    ///
    /// Returns an error if xml is invalid, `PARENT_NODE_NAME` is not found or if `DeparsehashMap.read_hash_map_from_event` returns an error
    ///
    /// # Attributes
    ///
    /// * `xml` - The xml containing a node with the name `PARENT_NODE_NAME` with children with the name `Node` and a `PrimaryKey`
    fn read_hash_map_from_xml(xml: &str) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_hash_map_from_event(&mut reader);
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
        Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME_DS))?
    }

    /// Reads the provided xml with `read_hash_map_from_xml` and compares it to the provided Hashmap.
    ///
    /// Panics if HashMaps are not equal or if `read_hash_map_from_xml` returns an error
    ///
    /// # Attributes
    ///
    /// * `one` - The Hasmap that is expected when xml is deparsed
    /// * `xml` - The xml containing a node with the name `PARENT_NODE_NAME` with children with the name `Node` and a `PrimaryKey`
    ///
    fn compare_hash_maps(one: HashMap<Self::PrimaryKey, Self>, xml: &str) where Self: Sized {
        let two = Self::read_hash_map_from_xml(xml);
        let two = two.expect(&format!("Testing {} for list raised an unexpected error", std::any::type_name::<Self>())[..]);
        assert_eq!(one, two)
    }
}

///If an xml-node just has a primary key (like name) it's unlikely that for every Name there should be created a struct. This method will just read out the `PrimaryKey`, put it in a vec and return it.
///The generic type P is the type of the primary-key for this xml-node. (example: Name if a node only contains one attribute node and no children)
pub(crate) trait DeparsePrimaryKey {
    /// A PrimaryKey should be unique across all xml-nodes of the same type in one GDTF file
    type PrimaryKey: Eq + Hash + Debug + Clone;
    ///Type of error returned in case of failure on deparse
    type Error: From<GdtfReadError> + std::error::Error;
    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME: &'static [u8];
    ///The name of the node wraps a list of nodes that contain the data for the struct. Declare it as b"GDTF" for example.
    const PARENT_NODE_NAME: &'static [u8];
    ///When a gdtf is deparsed, it will go down the tree of nodes. If a node has only one attribute that is unique, it's better to just store this `PrimaryKey` in a vec instead of having a vec of a wrapping struct.
    /// This method should return this single attribute.
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    /// # Arguments
    ///
    /// * `event` - The Event that was triggering the overlaying struct to go down the tree one step further
    ///
    /// # Returns
    ///
    /// * `P` - The unique and only attribute called `PrimaryKey` of the xml-node
    fn read_primary_key_from_event(event: BytesStart<'_>) -> Result<Self::PrimaryKey, Self::Error>;

    /// If a xml-node only has one attribute that is unique across all nodes of the same type inside one GDTF, This method can be used To deparse just this `PrimaryKeys` as vec from a List of Nodes
    /// The method will go down the tree of xml-nodes from the point it enters the function and deparse all node's `PrimaryKey` with the name equal to `NODE_NAME`
    fn read_primary_key_vec_from_event(reader: &mut Reader<&[u8]>) -> Result<Vec<Self::PrimaryKey>, Self::Error> where Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self::PrimaryKey> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        out.push(Self::read_primary_key_from_event(e)?);
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
}

///Trait to help testing DeparsePrimaryKey
#[cfg(test)]
pub(crate) trait TestDeparsePrimaryKey: DeparsePrimaryKey {
    /// Parses a given xml string to a Vector of PrimaryKeys. This method will go down the tree of nodes in the xml until it finds a node with the name `PARENT_NODE_NAME` and call `DeparsePrimaryKey::read_primary_key_vec_from_event` to all child-nodes found with the name eq to `NODE_NAME`
    ///
    /// Will return an error if xml is invalid, if `DeparseVec::read_primary_key_vec_from_event` returns an error or if `PARENT_NODE_NAME` is not found.
    ///
    /// # Attributes
    ///
    /// * `xml` The xml containing the node with name `PARENT_NODE_NAME` and children named `NODE_NAME`
    fn read_vec_from_xml(xml: &str) -> Result<Vec<Self::PrimaryKey>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_primary_key_vec_from_event(&mut reader);
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
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    /// Parses a given xml string to a PrimaryKey. This method will go down the tree of nodes in the xml until it finds a node with the name `NODE_NAME` and call `DeparsePrimaryKey::read_primary_key_vec_from_event` to it
    ///
    /// Will return an error if xml is invalid, if `DeparseVec::read_primary_key_vec_from_event` returns an error or if `NODE_NAME` is not found.
    ///
    /// # Attributes
    ///
    /// * `xml` The xml containing the node with name `PARENT_NODE_NAME` and children named `NODE_NAME`
    fn read_primary_key_from_xml(xml: &str) -> Result<Self::PrimaryKey, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Self::read_primary_key_from_event(e);
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
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME))?
    }
}

///If an xml-node is listed multiple times but has no unique attribute accross all these nodes, it's likely that this nodes are stored in a Vec. This trait implements methods for helping to achieve this
pub(crate) trait DeparseVec: DeparseSingle {
    ///The name of the node wraps a list of nodes that contain the data for the struct. Declare it as b"GDTF" for example.
    const PARENT_NODE_NAME: &'static [u8];
    ///If an xml-node is listed multiple times but has no unique attribute accross all these nodes (`PrimaryKey`), this function can be used.
    /// This function will go down the tree and deparse all nodes with the name eq to `NODE_NAME` in a vec and return when it's back up at it's position.
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    ///
    /// # Returns
    ///
    /// * `Vec<Self>` - All structs found by xml-nodes in a vec
    fn read_vec_from_event(reader: &mut Reader<&[u8]>) -> Result<Vec<Self>, Self::Error> where Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) => {
                    if e.name() == Self::NODE_NAME_DS {
                        out.push(Self::read_single_from_event(reader, e, true)?.1);
                    } else {
                        tree_down += 1;
                    }
                }
                Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME_DS {
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
}

///Trait to help testing DeparseVec
#[cfg(test)]
pub(crate) trait TestDeparseVec: DeparseVec + TestDeparseSingle {
    /// Parses a given xml string to a Vector of structs. This method will go down the tree of nodes in the xml until it finds a node with the name `PARENT_NODE_NAME` and call `DeparseVec::read_vec_from_event` to all child-nodes found with the name eq to `NODE_NAME`
    ///
    /// Will return an error if xml is invalid, if `DeparseVec::read_vec_from_event` returns an error or if `PARENT_NODE_NAME` is not found.
    ///
    /// # Attributes
    ///
    /// * `xml` The xml containing the node with name `PARENT_NODE_NAME` and children named `NODE_NAME`
    fn read_vec_from_xml(xml: &str) -> Result<Vec<Self>, Self::Error> where Self: Sized {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfReadError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::PARENT_NODE_NAME {
                        return Self::read_vec_from_event(&mut reader);
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
            };
            buf.clear();
        }
        Err(GdtfReadError::new_xml_node_not_found(Self::NODE_NAME_DS))?
    }

    /// Reads the provided xml with `read_vec_from_xml` and compares it to the provided Vec.
    ///
    /// Panics if Vecs are not equal or if `read_vec_from_xml` returns an error
    ///
    /// # Attributes
    ///
    /// * `one` - The Vec that is expected when xml is deparsed
    /// * `xml` - The xml containing a node with the name `PARENT_NODE_NAME` with children with the name `Node`
    ///
    fn compare_vecs(one: Vec<Self>, xml: &str) where Self: Sized {
        let two = Self::read_vec_from_xml(xml);
        let two = two.expect(&format!("Testing {} for list raised an unexpected error", std::any::type_name::<Self>())[..]);
        assert_eq!(one, two)
    }
}

///Helper function to create useful error messages
#[allow(dead_code)]
fn u8_array_to_string(val: &[u8]) -> String {
    std::str::from_utf8(val).map_or("?".to_string(), |e| e.to_string())
}