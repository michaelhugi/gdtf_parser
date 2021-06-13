#![cfg(test)]
//! Module only used to provide functions to remove noise from tests
use std::borrow::Cow;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

use quick_xml::events::attributes::Attribute;
use xmltree::{Element, EmitterConfig};

use crate::utils::errors::GdtfError;

/// Creates a new quick-xml-attribute with borrowed value
pub fn to_attr_borrowed(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value) }
}

/// Creates a new quick-xml-attribute with owned value
pub fn to_attr_owned(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value).to_owned() }
}

/// Joins two vecs to a hash-map. Fist vec contains keys, second contains values
pub(crate) fn vec_to_hash_map<K: Eq + Hash + Clone, T>(keys: Vec<K>, input: Vec<T>) -> HashMap<K, T> {
    if keys.len() != input.len() {
        panic!("Vec length must be the same!");
    }
    let mut output: HashMap<K, T> = HashMap::new();

    for value in input.into_iter().enumerate() {
        let key = keys.get(value.0).unwrap().clone();
        output.insert(key.clone(), value.1);
    }
    output
}

#[allow(dead_code)]
pub(crate) fn print_xml_to_file(out_path: &str, xml: &str, pretty: bool) -> Result<(), GdtfError> {
    if !pretty {
        let mut file = File::create(out_path)?;
        file.write_all(xml.as_bytes())?;
    } else {
        let el = Element::parse(xml.as_bytes()).expect("parsexml");
        let mut cfg = EmitterConfig::new();
        cfg.perform_indent = true;

        let mut out_file = File::create(out_path).expect("createoutfile");
        el.write_with_config(&mut out_file, cfg).expect("writexml");
        let _ = out_file.write("\n".as_bytes());
    }
    Ok(())
}