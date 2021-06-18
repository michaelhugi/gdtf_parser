use crate::fixture_type::wheel::slot::Slot;

pub mod slot;

pub struct Wheel{
    pub slots: Vec<Slot>
}

impl Wheel{
    const NODE_NAME: &'static [u8] = b"Wheel";
}