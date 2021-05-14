use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::errors::GdtfError;
use crate::errors::GdtfError::QuickXMLError;

pub trait Deparse: std::fmt::Debug {
    fn from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized {
        if !Self::is_event_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::event_name());
        }
        Self::from_event_unchecked(reader, e)
    }

    fn from_event_unchecked(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Self, GdtfError> where
        Self: Sized;

    fn is_event_name(event_name: &[u8]) -> bool;

    fn event_name() -> String;

    #[cfg(test)]
    fn from_reader(reader: &mut Reader<&[u8]>) -> Result<Self, GdtfError> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_event_name(e.name()) {
                        return Self::from_event(reader, e);
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
    fn from_xml(xml: &str) -> Result<Self, GdtfError>
        where Self: Sized {
        let mut reader = Reader::from_str(xml);
        Self::from_reader(&mut reader)
    }

    #[cfg(test)]
    fn loose_eq_test(&self, other: &Self) -> bool;
    #[cfg(test)]
    fn print_not_eq(&self, other: &Self) {
        println!("Gdtf Files in test were not equal: \n {:?} \n {:?}", self, other);
    }
    #[cfg(test)]
    fn test_eq(&self, other: &Self) {
        if !self.loose_eq_test(other) {
            self.print_not_eq(other);
            assert!(false);
        } else {
            assert!(true);
        }
    }
}


pub trait DeparseList: Deparse {
    fn vec_from_event(reader: &mut Reader<&[u8]>, e: BytesStart<'_>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        if !Self::is_event_group_name(&e.name()) {
            panic!("Wrong event passed for reading {}", Self::event_group_name());
        }

        let mut buf: Vec<u8> = Vec::new();
        let mut out: Vec<Self> = Vec::new();
        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_event_name(e.name()) {
                        out.push(Self::from_event(reader, e)?);
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


    fn is_event_group_name(event_name: &[u8]) -> bool;

    fn event_group_name() -> String;

    #[cfg(test)]
    fn vec_from_reader(reader: &mut Reader<&[u8]>) -> Result<Vec<Self>, GdtfError> where
        Self: Sized {
        reader.trim_text(true);

        let mut buf: Vec<u8> = Vec::new();

        loop {
            match reader.read_event(&mut buf) {
                Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                    if Self::is_event_group_name(e.name()) {
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
    fn vec_test_eq(one: Result<Vec<Self>, GdtfError>, two: Vec<Self>) where Self: Sized {
        let one = one.expect(&format!("Testing {} for list raised an unexpected error", &Self::event_group_name()[..])[..]);
        if one.len() != two.len() {
            println!("Testing {} for list returned not the same amount of items", Self::event_group_name());
            assert!(false);
            return;
        }
        for o in one.iter() {
            let mut b = false;
            for t in two.iter() {
                if o.loose_eq_test(&t) {
                    b = true;
                }
            }
            if !b {
                println!("Testing {} for list returned different results: \n{:?}\n{:?}", Self::event_group_name(), one, two);
                assert!(false);
                return;
            }
        }
        assert!(true);
    }
}