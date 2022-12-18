use crate::hittable::{hit_list, Hit, Hittable, ObjectContainer};
use crate::object::Object;
use crate::ray::Ray;
use crate::util::{EPSILON, INFINITY};
use crate::vec3::Vec3;

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
                        hit_list(children, ray, t_min.max(EPSILON), t_max).map(|(o, h)| (o, h));
                    return ret;
                }
                TDTreePart::Node {
                    axis,
                    h,
                    left,
                    right,
                } => {
                    if t_min > t_max {
                        return None;
                    }
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
pub enum Axis {
    X,
    Y,
    Z,
}

pub fn build_tdtree<'a>(hittables: &'a Vec<Object>, max_depth: i32) -> TDTree<'a> {
    TDTree {
        root: _build_tdtree(hittables.iter().collect(), max_depth),
    }
}

fn _build_tdtree<'a>(hittables: Vec<&'a Object>, depth_remaining: i32) -> Box<TDTreePart<'a>> {
    // We stop at 0 instead of 1 because cutting down the space of a 1 object box can still yield performance increase
    if hittables.len() == 0 || depth_remaining == 0 {
        return Box::new(TDTreePart::Leaf {
            children: hittables,
        });
    }
    let axis = match depth_remaining % 3 {
        0 => Axis::X,
        1 => Axis::Y,
        2 => Axis::Z,
        _ => panic!(),
    };

    // Get all bounds of the objects to find a good point for the plane
    let mut bound_points: Vec<Vec3> = hittables
        .iter()
        .flat_map(|h| vec![h.get_bounds().lower(), h.get_bounds().higher()])
        .collect();

    bound_points.sort_by(|p1, p2| p1.get_axis(&axis).partial_cmp(&p2.get_axis(&axis)).unwrap());
    let mid = bound_points[bound_points.len() / 2].get_axis(&axis);

    // Split objects into left and right subtree
    let left: Vec<&Object> = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().lower().get_axis(&axis) <= mid)
        .collect();
    let right: Vec<&Object> = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().higher().get_axis(&axis) >= mid)
        .clone()
        .collect();

    // Some objects we cannot split into just one subtree, make sure this doesn't happen too often or we are actually doing work multiple times
    if ((left.len() + right.len()) as f64 / hittables.len() as f64) > 1.5 {
        _build_tdtree(hittables, depth_remaining - 1)
    } else {
        Box::new(TDTreePart::Node {
            axis: axis,
            h: mid,
            left: _build_tdtree(left, depth_remaining - 1),
            right: _build_tdtree(right, depth_remaining - 1),
        })
    }
}
