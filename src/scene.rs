use crate::hittable::ObjectContainer;
use crate::light::Light;
use crate::three_d_tree::TDTree;

pub struct Scene<'a> {
    pub objects: &'a dyn ObjectContainer,
    pub lights: Vec<Box<dyn Light>>,
}
