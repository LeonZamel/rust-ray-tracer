use crate::hittable::{hit_list2, Hit, Hittable, ObjectContainer};
use crate::object::Object;
use crate::ray::Ray;
use crate::util::{EPSILON, INFINITY};
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct TDTree<'a> {
    root: Box<TDTreePart<'a>>,
}
impl ObjectContainer for TDTree<'_> {
    fn get_object_hit(&self, ray: &Ray) -> Option<(&Object, Hit)> {
        fn _obj_hit<'a>(
            node: &'a TDTreePart<'a>,
            ray: &Ray,
            t_min: f64,
            t_max: f64,
        ) -> Option<(&'a Object, Hit)> {
            match node {
                TDTreePart::Leaf { children } => {
                    let ret =
                        hit_list2(children, ray, t_min.max(EPSILON), t_max).map(|(o, h)| (o, h));
                    return ret;
                }
                TDTreePart::Node {
                    axis,
                    h,
                    left,
                    right,
                } => {
                    let t = ray.intersect_axis_plane(&axis, *h);
                    let (first, second) = {
                        if ray.direction.get_axis(&axis) >= 0.0 {
                            (left, right)
                        } else {
                            (right, left)
                        }
                    };
                    if t < 0.0 {
                        return _obj_hit(second.as_ref(), ray, t, t_max);
                    }
                    _obj_hit(first.as_ref(), ray, t_min, t.min(t_max))
                        .or_else(|| _obj_hit(second.as_ref(), ray, t.max(t_min), t_max))
                }
            }
        }
        _obj_hit(self.root.as_ref(), ray, -INFINITY, INFINITY)
    }
}
#[derive(Debug)]
pub enum TDTreePart<'a> {
    Node {
        axis: Axis,
        h: f64,
        left: Box<TDTreePart<'a>>,
        right: Box<TDTreePart<'a>>,
    },
    Leaf {
        children: Vec<&'a Object>,
    },
}
#[derive(Debug)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub fn build_tdtree<'a>(hittables: &'a Vec<Object>) -> TDTree<'a> {
    TDTree {
        root: _build_tdtree(hittables.iter().collect(), 0),
    }
}

fn _build_tdtree<'a>(hittables: Vec<&'a Object>, depth: i32) -> Box<TDTreePart<'a>> {
    if hittables.len() < 3 || depth >= 10 {
        return Box::new(TDTreePart::Leaf {
            children: hittables,
        });
    }
    let axis = match depth % 3 {
        0 => Axis::X,
        1 => Axis::Y,
        2 => Axis::Z,
        _ => panic!(),
    };
    let mut bound_points: Vec<Vec3> = hittables
        .iter()
        .flat_map(|h| vec![h.get_bounds().lower(), h.get_bounds().higher()])
        .collect();

    bound_points.sort_by(|p1, p2| p1.get_axis(&axis).partial_cmp(&p2.get_axis(&axis)).unwrap());
    let mid = bound_points[bound_points.len() / 2].get_axis(&axis);

    let left = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().lower().get_axis(&axis) <= mid)
        .collect();
    let right = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().higher().get_axis(&axis) >= mid)
        .clone()
        .collect();

    Box::new(TDTreePart::Node {
        axis: axis,
        h: mid,
        left: _build_tdtree(left, depth + 1),
        right: _build_tdtree(right, depth + 1),
    })
}
