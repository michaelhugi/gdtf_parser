use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::errors::GdtfError;
use crate::errors::GdtfError::QuickXMLError;

pub trait DeparseSingle: std::fmt::Debug {
    fn single_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        if !Self::is_single_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::single_event_name());
        }
        Self::single_from_event_unchecked(reader, e)
    }

    fn single_from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized;

    fn is_single_event_name(event_name: &[u8]) -> bool;

    fn single_event_name() -> String;

    #[cfg(test)]
    fn single_from_reader(reader: &mut Reader<&[u8]>) -> Result<Self, GdtfError> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_single_event_name(e.name()) {
                        return Self::single_from_event(reader, e);
                    }
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            };
        }
        buf.clear();
        Err(GdtfError::RequiredValueNotFoundError("Could not find Feature".to_string()))
    }
    #[cfg(test)]
    fn single_from_xml(xml: &str) -> Result<Self, GdtfError>
        where Self: Sized {
        let mut reader = Reader::from_str(xml);
        Self::single_from_reader(&mut reader)
    }

    #[cfg(test)]
    fn is_single_eq(&self, other: &Self) -> bool;

    #[cfg(test)]
    fn test(&self, xml: &str) where Self: Sized {
        self.test_with_result(Self::single_from_xml(xml));
    }

    #[cfg(test)]
    fn test_with_result(&self, other: Result<Self, GdtfError>) where Self: Sized {
        let other = other.expect(&format!("Unexpected error in test of {}", Self::single_event_name())[..]);
        if !self.is_single_eq(&other) {
            println!("Gdtf Files in test were not equal: \n {:?} \n {:?}", self, other);
            assert!(false);
        } else {
            assert!(true);
        }
    }


    #[cfg(test)]
    fn is_vec_eq(one: &Vec<Self>, two: &Vec<Self>) -> bool where Self: Sized {
        if one.len() != two.len() {
            println!("Testing {} for vec returned not the same amount of items", Self::single_event_name());
            return false;
        }
        for o in one.iter() {
            let mut b = false;
            for t in two.iter() {
                if o.is_single_eq(&t) {
                    b = true;
                }
            }
            if !b {
                println!("Testing {} for vec returned different results: \n{:?}\n{:?}", Self::single_event_name(), one, two);
                return false;
            }
        }
        true
    }

}


pub trait DeparseVec: DeparseSingle {
    fn vec_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        if !Self::is_group_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::group_event_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_single_event_name(e.name()) {
                        out.push(Self::single_from_event(reader, e)?);
                    }
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
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

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_group_event_name(e.name()) {
                        return Self::vec_from_event(reader, e);
                    }
                }
                Ok(Event::Eof) | Ok(Event::End(_)) => {
                    break;
                }
                Err(e) => return Err(QuickXMLError(e)),
                _ => {}
            };
            buf.clear();
        }

        Err(GdtfError::RequiredValueNotFoundError("Could not find Feature".to_string()))
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