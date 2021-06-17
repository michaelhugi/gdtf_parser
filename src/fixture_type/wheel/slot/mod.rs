pub mod facet;
pub mod animation_system;

use crate::utils::units::color_cie::ColorCie;
use crate::utils::units::resource::Resource;
use crate::utils::units::node::Node;

pub struct Slot{
    pub color: ColorCie,
    pub filter: Node,
    pub media_file_name: Resource,
   // pub prism_facets: Option<Vec<Facet>>,
    //pub animation_whee: Option<AnimationSystem>
}

impl Slot{
    const NODE_NAME: &'static [u8] = b"Slot";
}