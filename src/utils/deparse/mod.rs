use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;

pub(crate) trait DeparseSingle: std::fmt::Debug + Sized {
    type PrimaryKey: Eq + Hash + Debug + Clone;
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError>;

    fn is_single_event_name(event_name: &[u8]) -> bool;

    fn single_event_name() -> String;
}

#[cfg(test)]
pub(crate) trait TestDeparseSingle: Debug + PartialEq<Self> + Sized + DeparseSingle {
    fn single_from_reader(reader: &mut Reader<&[u8]>) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_single_event_name(e.name()) {
                        return Self::single_from_event(reader, e);
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
        Err(GdtfError::RequiredValueNotFoundError(format!("Could not find {}", Self::single_event_name())))
    }

    fn single_from_xml(xml: &str) -> Result<(Self, Option<Self::PrimaryKey>), GdtfError> {
        let mut reader = Reader::from_str(xml);
        Self::single_from_reader(&mut reader)
    }

    fn test(&self, primary_key: Option<Self::PrimaryKey>, xml: &str) {
        self.test_with_result(primary_key, Self::single_from_xml(xml));
    }

    fn test_with_result(&self, primary_key: Option<Self::PrimaryKey>, other: Result<(Self, Option<Self::PrimaryKey>), GdtfError>) {
        let other = other.expect(&format!("Unexpected error in test of {}", Self::single_event_name())[..]);
        assert_eq!(self, &other.0);
        match (primary_key, other.1) {
            (Some(primary_key), Some(other)) => assert_eq!(primary_key, other),
            (None, None) => {}
            (primary_key, other) => panic!("Primary Keys not equal \nleft: {:?}\nright: {:?}", primary_key, other)
        }
    }
}

pub(crate) trait DeparseHashMap: DeparseSingle {
    fn hash_map_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<HashMap<Self::PrimaryKey, Self>, GdtfError> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", std::any::type_name::<Self>());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: HashMap<Self::PrimaryKey, Self> = HashMap::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_single_event_name(e.name()) {
                        let val = Self::single_from_event(reader, e)?;
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
    fn primary_key_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<P, GdtfError>;


    fn primary_key_vec_from_event(reader: &mut Reader<&[u8]>, _: BytesStart<'_>) -> Result<Vec<P>, GdtfError> where
        Self: Sized {
        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<P> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
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
    fn vec_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", std::any::type_name::<Self>());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_single_event_name(e.name()) {
                        out.push(Self::single_from_event(reader, e)?.0);
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
    fn vec_from_reader(reader: &mut Reader<&[u8]>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
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
        Err(GdtfError::RequiredValueNotFoundError(format!("Could not find {}", Self::single_event_name())))
    }

    fn vec_from_xml(xml: &str) -> Result<Vec<Self>, GdtfError>
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
    fn hash_map_from_reader(reader: &mut Reader<&[u8]>) -> Result<HashMap<Self::PrimaryKey, Self>, GdtfError> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
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
        Err(GdtfError::RequiredValueNotFoundError(format!("Could not find {}", Self::single_event_name())))
    }

    fn hash_map_from_xml(xml: &str) -> Result<HashMap<Self::PrimaryKey, Self>, GdtfError>
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