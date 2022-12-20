use crate::hittable::ObjectContainer;
use crate::light::Light;

pub struct Scene<'a> {
    pub objects: &'a dyn ObjectContainer,
    pub lights: &'a Vec<Box<dyn Light>>,
}

unsafe impl<'a> Sync for Scene<'a> {}
