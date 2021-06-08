///! Module contains traits that can be implemented to simpler deparse structs from quick-xml without serde to have full control of the flow
use std::borrow::Borrow;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fmt;
use std::hash::Hash;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;

///Trait to deparse an xml-node to a struct
pub(crate) trait DeparseSingle: std::fmt::Debug + Sized {
    ///The primary-key of the struct if used in a hash-map or () if no primary key present
    type PrimaryKey: Eq + Hash + Debug + Clone;
    ///Type of error returned in case of failure on deparse
    type Error: From<GdtfDeparseError> + std::error::Error;
    ///The name of the node that contains the data for the struct. Declare it as b"GDTF" for example.
    const NODE_NAME: &'static [u8];

    /// When a gdtf is deparsed it will go down the tree if a event hits and returns when end of the Node from the event is detected.
    ///
    /// ⚠️**Be aware that when returning an Error, the whole GDTF-Deparsing will fail!** ⚠️
    ///
    /// # Arguments
    ///
    /// * `reader` - The quick-xml-Reader that is passed trough the whole tree to read next events of the tree branch if needed. Iterates trough all xml-events with buffering.
    /// * `e` - The Event that was triggering the overlaying struct to go down the tree one step further
    ///
    /// # Returns
    ///
    /// * `Self` - The struct deparsed from the reader
    /// * `Self::PrimaryKey` - If the struct has a primary key (to use in a hashmap for example) it will be returned here
    fn read_single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), Self::Error>;
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
    fn read_single_from_xml(xml: &str) -> Result<(Self, Option<Self::PrimaryKey>), Self::Error> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(|e| GdtfDeparseError::QuickXmlError(e))? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        return Self::read_single_from_event(&mut reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
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
        let other = Self::read_single_from_xml(xml).expect(&format!("Unexpected error in test of {}", u8_array_to_string(Self::NODE_NAME))[..]);
        assert_eq!(self, &other.0);
        match (primary_key, other.1) {
            (Some(primary_key), Some(other)) => assert_eq!(primary_key, other),
            (None, None) => {}
            (primary_key, other) => panic!("Primary Keys not equal \nleft: {:?}\nright: {:?}", primary_key, other)
        }
    }

}

pub(crate) trait DeparseHashMap: DeparseSingle {
    fn hash_map_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", std::any::type_name::<Self>());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: HashMap<Self::PrimaryKey, Self> = HashMap::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        let val = Self::read_single_from_event(reader, e)?;
                        if val.1.is_some() {
                            out.insert(val.1.unwrap(), val.0);
                        }
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
    fn is_group_event_name(event_name: &[u8]) -> bool;
}

pub(crate) trait DeparsePrimaryKey<P: Eq + Hash + Debug + Clone> {
    type Error: From<GdtfDeparseError> + std::error::Error;
    fn primary_key_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<P, Self::Error>;


    fn primary_key_vec_from_event(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<Vec<P>, Self::Error> where
        Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<P> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_single_event_name(e.name()) {
                        out.push(Self::primary_key_from_event(reader, e)?);
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

    fn is_single_event_name(event_name: &[u8]) -> bool;
}

pub(crate) trait DeparseVec: DeparseSingle {
    fn vec_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Vec<Self>, Self::Error> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", std::any::type_name::<Self>());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(GdtfDeparseError::QuickXmlError)? {
                Event::Start(e) | Event::Empty(e) => {
                    if e.name() == Self::NODE_NAME {
                        out.push(Self::read_single_from_event(reader, e)?.0);
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
    fn is_group_event_name(event_name: &[u8]) -> bool;
}

#[cfg(test)]
pub(crate) trait TestDeparseVec: DeparseVec + TestDeparseSingle {
    fn vec_from_reader(reader: &mut Reader<&[u8]>) -> Result<Vec<Self>, Self::Error> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(|e| GdtfDeparseError::QuickXmlError(e))? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_group_event_name(e.name()) {
                        return Self::vec_from_event(reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    fn vec_from_xml(xml: &str) -> Result<Vec<Self>, Self::Error>
        where Self: Sized {
        let mut reader = Reader::from_str(xml);
        Self::vec_from_reader(&mut reader)
    }

    fn test_group(one: Vec<Self>, xml: &str) where Self: Sized {
        let two = Self::vec_from_xml(xml);
        let two = two.expect(&format!("Testing {} for list raised an unexpected error", std::any::type_name::<Self>())[..]);
        assert_eq!(one, two)
    }
}


#[cfg(test)]
pub(crate) trait TestDeparseHashMap: DeparseHashMap + TestDeparseSingle {
    fn hash_map_from_reader(reader: &mut Reader<&[u8]>) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf).map_err(|e| GdtfDeparseError::QuickXmlError(e))? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_group_event_name(e.name()) {
                        return Self::hash_map_from_event(reader, e);
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
        Err(GdtfDeparseError::new_xml_node_not_found(Self::NODE_NAME))?
    }

    fn hash_map_from_xml(xml: &str) -> Result<HashMap<Self::PrimaryKey, Self>, Self::Error>
        where Self: Sized {
        let mut reader = Reader::from_str(xml);
        Self::hash_map_from_reader(&mut reader)
    }

    fn test_group(one: HashMap<Self::PrimaryKey, Self>, xml: &str) where Self: Sized {
        let two = Self::hash_map_from_xml(xml);
        let two = two.expect(&format!("Testing {} for list raised an unexpected error", std::any::type_name::<Self>())[..]);
        assert_eq!(one, two)
    }
}

pub(crate) fn attr_to_str<'a>(attr: &'a Attribute) -> &'a str {
    std::str::from_utf8(attr.value.borrow()).unwrap_or("")
}

pub(crate) fn attr_try_to_str<'a>(attr: &'a Attribute) -> Result<&'a str, GdtfError> {
    Ok(std::str::from_utf8(attr.value.borrow())?)
}

pub(crate) fn attr_to_f32(attr: &Attribute) -> f32 {
    f32::from_str(attr_try_to_str(attr).unwrap_or("")).unwrap_or(0_f32)
}

pub(crate) fn attr_to_f32_option(attr: &Attribute) -> Option<f32> {
    match f32::from_str(attr_try_to_str(attr).unwrap_or("")) {
        Ok(f) => Some(f),
        Err(_) => None
    }
}

pub(crate) fn attr_to_string_option(attr: &Attribute) -> Option<String> {
    match attr_try_to_str(attr).unwrap_or("") {
        "" => None,
        s => Some(s.to_owned())
    }
}

pub(crate) fn attr_to_string(attr: &Attribute) -> String {
    attr_try_to_str(attr).unwrap_or("").to_owned()
}

pub(crate) fn attr_to_u8_option(attr: &Attribute) -> Option<u8> {
    match u8::from_str(attr_try_to_str(attr).unwrap_or("")) {
        Ok(f) => Some(f),
        Err(_) => None
    }
}

#[derive(Debug)]
pub enum GdtfDeparseError {
    QuickXmlError(quick_xml::Error),
    QuickXmlNodeNotFoundError(String),
    QuickXmlAttributeNotFoundError(String, String),
}

impl GdtfDeparseError {
    pub fn new_xml_node_not_found(node_name: &[u8]) -> Self {
        Self::QuickXmlNodeNotFoundError(u8_array_to_string(node_name))
    }
    pub fn new_xml_attribute_not_found(node_name: &[u8], attribute_name: &[u8]) -> Self {
        Self::QuickXmlAttributeNotFoundError(
            u8_array_to_string(node_name),
            u8_array_to_string(attribute_name),
        )
    }
}

fn u8_array_to_string(val: &[u8]) -> String {
    std::str::from_utf8(val).map_or("?".to_string(), |e| e.to_string())
}

impl Display for GdtfDeparseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            GdtfDeparseError::QuickXmlError(e) => write!(f, "GdtfDeparseError: {}", e),
            GdtfDeparseError::QuickXmlAttributeNotFoundError(node_name, attribute) => write!(f, "Could not find xml-attribute '{}' in '{}'", attribute, node_name),
            GdtfDeparseError::QuickXmlNodeNotFoundError(node_name) => write!(f, "Could not find xml-node name '{}'", node_name),
        }
    }
}

impl Error for GdtfDeparseError {}