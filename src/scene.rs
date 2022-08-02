use crate::light::Light;
use crate::object::Object;

pub struct Scene {
    pub objects: Vec<Object>,
    pub lights: Vec<Box<dyn Light>>,
}
