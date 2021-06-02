#![cfg(test)]

use std::borrow::Cow;
use std::collections::HashMap;

use quick_xml::events::attributes::Attribute;

use crate::utils::deparse::DeparseHashMap;

pub fn to_attr_borrowed(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value) }
}

pub fn to_attr_owned(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value).to_owned() }
}

pub(crate) fn vec_to_hash_map<T: DeparseHashMap>(keys: Vec<T::PrimaryKey>, input: Vec<T>) -> HashMap<T::PrimaryKey, T> {
    if keys.len() != input.len() {
        panic!("Vec length must be the same!");
    }
    let mut output: HashMap<T::PrimaryKey, T> = HashMap::new();

    for value in input.into_iter().enumerate() {
        let key = keys.get(value.0).unwrap().clone();
        output.insert(key.clone(), value.1);
    }
    output
}