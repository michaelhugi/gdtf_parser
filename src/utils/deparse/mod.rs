use std::borrow::Borrow;
use std::convert::TryInto;
use std::str::FromStr;

use quick_xml::events::attributes::Attribute;
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::utils::errors::GdtfError;
use crate::utils::units::name::Name;

pub(crate) trait DeparseSingle: std::fmt::Debug + Sized {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> {
        if !Self::is_single_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::single_event_name());
        }
        Self::single_from_event_unchecked(reader, e)
    }

    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> ;

    fn is_single_event_name(event_name: &[u8]) -> bool;

    fn single_event_name() -> String;

    #[cfg(test)]
    fn single_from_reader(reader: &mut Reader<&[u8]>) -> Result<Self, GdtfError> {
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
    #[cfg(test)]
    fn single_from_xml(xml: &str) -> Result<Self, GdtfError> {
        let mut reader = Reader::from_str(xml);
        Self::single_from_reader(&mut reader)
    }

    #[cfg(test)]
    fn is_single_eq_log(&self, other: &Self) -> bool {
        let b = self.is_single_eq_no_log(other);
        if !b {
            println!("Gdtf Files in test were not equal: \n {:?} \n {:?}", self, other);
        }
        b
    }

    #[cfg(test)]
    fn is_single_eq_no_log(&self, other: &Self) -> bool;

    #[cfg(test)]
    fn test(&self, xml: &str) {
        self.test_with_result(Self::single_from_xml(xml));
    }

    #[cfg(test)]
    fn test_with_result(&self, other: Result<Self, GdtfError>)  {
        let other = other.expect(&format!("Unexpected error in test of {}", Self::single_event_name())[..]);
        if !self.is_single_eq_no_log(&other) {
            println!("Gdtf Files in test were not equal: \n {:?} \n {:?}", self, other);
            assert!(false);
        } else {
            assert!(true);
        }
    }

    #[cfg(test)]
    fn is_same_item_identifier(&self, compare: &Self) -> bool;

    #[cfg(test)]
    fn is_vec_eq(one: &Vec<Self>, two: &Vec<Self>) -> bool  {
        if one.len() != two.len() {
            println!("Testing {} for vec returned not the same amount of items", Self::single_event_name());
            return false;
        }
        for o in one.iter() {
            let mut t = o.clone();
            let mut item_found = false;

            for tw in two.iter() {
                if o.is_same_item_identifier(tw) {
                    t = tw;
                    item_found = true;
                }
            }
            if !item_found {
                println!("Did not find {} that corresponds to \n{:?}", Self::single_event_name(), o);
                return false;
            }

            if !o.is_single_eq_log(t) {
                return false;
            }
        }
        true
    }
}


pub(crate) trait DeparseVec: DeparseSingle {
    fn vec_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::group_event_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        let mut tree_down = 0;
        loop {
            match reader.read_event(&mut buf)? {
                Event::Start(e) | Event::Empty(e) => {
                    if Self::is_single_event_name(e.name()) {
                        out.push(Self::single_from_event(reader, e)?);
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

    fn group_event_name() -> String;

    #[cfg(test)]
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
    #[cfg(test)]
    fn vec_from_xml(xml: &str) -> Result<Vec<Self>, GdtfError>
        where Self: Sized {
        let mut reader = Reader::from_str(xml);
        Self::vec_from_reader(&mut reader)
    }
    #[cfg(test)]
    fn test_group(one: Vec<Self>, xml: &str) where Self: Sized {
        let two = Self::vec_from_xml(xml);
        let two = two.expect(&format!("Testing {} for list raised an unexpected error", Self::group_event_name())[..]);
        assert!(Self::is_vec_eq(&one, &two));
    }
}


pub(crate) fn attr_to_str<'a>(attr: &'a Attribute) -> &'a str {
    std::str::from_utf8(attr.value.borrow()).unwrap_or_else(|_| "")
}

pub(crate) fn attr_try_to_str<'a>(attr: &'a Attribute) -> Result<&'a str, GdtfError> {
    Ok(std::str::from_utf8(attr.value.borrow())?)
}

pub(crate) fn attr_to_f32(attr: &Attribute) -> f32 {
    f32::from_str(attr_try_to_str(attr).unwrap_or_else(|_| "")).unwrap_or_else(|_| 0.)
}

pub(crate) fn attr_to_f32_option(attr: &Attribute) -> Option<f32> {
    match f32::from_str(attr_try_to_str(attr).unwrap_or_else(|_| "")) {
        Ok(f) => Some(f),
        Err(_) => None
    }
}

pub(crate) fn attr_to_string_option(attr: &Attribute) -> Option<String> {
    match attr_try_to_str(attr).unwrap_or_else(|_| "") {
        "" => None,
        s => Some(s.to_owned())
    }
}

pub(crate) fn attr_to_string(attr: &Attribute) -> String {
    attr_try_to_str(attr).unwrap_or_else(|_| "").to_owned()
}

pub(crate) fn attr_try_to_name(attr: &Attribute) -> Result<Name, GdtfError> {
    Ok(attr_to_str(attr).try_into()?)
}

pub(crate) fn attr_to_str_option<'a>(attr: &'a Attribute) -> Option<&'a str> {
    match attr_try_to_str(attr).unwrap_or_else(|_| "") {
        "" => None,
        s => Some(s)
    }
}

pub(crate) fn attr_to_u8_option(attr: &Attribute) -> Option<u8> {
    match u8::from_str(attr_try_to_str(attr).unwrap_or_else(|_| "")) {
        Ok(f) => Some(f),
        Err(_) => None
    }
}