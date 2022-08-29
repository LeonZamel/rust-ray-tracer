use crate::light::Light;
use crate::three_d_tree::TDTree;

pub struct Scene<'a> {
    pub objects: TDTree<'a>,
    pub lights: Vec<Box<dyn Light>>,
}
