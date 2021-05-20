#![cfg(test)]

use std::borrow::Cow;

use quick_xml::events::attributes::Attribute;

pub mod sgm_light_at_g_7_spot_at_rev_a;
pub mod acme_at_acme_ae_610_beam_at_acme_ae_610_beam;
pub mod robe_lighting_at_robin_viva_cmy_at_13042021;
pub mod jb_lighting_at_p12_spot_hp_at_v_1_15;


pub fn to_attr_borrowed(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value) }
}

pub fn to_attr_owned(value: &[u8]) -> Attribute {
    Attribute { key: &[], value: Cow::Borrowed(value).to_owned() }
}