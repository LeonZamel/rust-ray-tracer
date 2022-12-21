use crate::hittable::{hit_list, Hit, Hittable, ObjectContainer};
use crate::object::Object;
use crate::ray::Ray;
use crate::util::{EPSILON, INFINITY};
use crate::vec3::Vec3;

pub struct TDTree<'a> {
    root: Box<TDTreePart<'a>>,
}
unsafe impl<'a> Sync for TDTree<'a> {}
impl ObjectContainer for TDTree<'_> {
    fn get_object_hit(&self, ray: &Ray) -> Option<(&Object, Hit)> {
        fn _obj_hit<'a>(
            node: &'a TDTreePart<'a>,
            ray: &Ray,
            t_min: f64,
            t_max: f64,
        ) -> Option<(&'a Object, Hit)> {
            if t_min >= t_max {
                return None;
            }
            match node {
                TDTreePart::Leaf { children } => hit_list(children, ray, t_min.max(EPSILON), t_max),
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
                    if t > t_max {
                        // Plane intersection comes after the ray interval
                        _obj_hit(first.as_ref(), ray, t_min, t_max)
                    } else if t < t_min {
                        // Plane intersection comes before the ray interval
                        _obj_hit(second.as_ref(), ray, t_min, t_max)
                    } else {
                        // Plane intersection is in the ray interval
                        _obj_hit(first.as_ref(), ray, t_min, t)
                            .or_else(|| _obj_hit(second.as_ref(), ray, t, t_max))
                    }
                }
            }
        }
        _obj_hit(self.root.as_ref(), ray, EPSILON, INFINITY)
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
    // If there are not many objects, it will likely be better to not disect any further and just test all the objects
    if hittables.len() <= 8 || depth_remaining == 0 {
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
    // Alternatingly use lower and upper bound
    let eps: f64;
    let mut bound_points: Vec<Vec3> = {
        if depth_remaining % 6 == 0 {
            eps = EPSILON;
            hittables.iter().map(|h| h.get_bounds().higher()).collect()
        } else {
            eps = -EPSILON;
            hittables.iter().map(|h| h.get_bounds().lower()).collect()
        }
    };
    bound_points.sort_by(|p1, p2| p1.get_axis(&axis).partial_cmp(&p2.get_axis(&axis)).unwrap());

    // Add on a small epsilon so the object used for the bound doesn't get added to both sides
    let mid = bound_points[bound_points.len() / 2 - 1].get_axis(&axis) + eps;

    // Split objects into left and right subtree
    let left: Vec<&Object> = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().lower().get_axis(&axis) <= mid)
        .collect();
    let right: Vec<&Object> = hittables
        .clone()
        .into_iter()
        .filter(|&h| h.get_bounds().higher().get_axis(&axis) > mid)
        .clone()
        .collect();

    // Make sure the split actually reduces work by a reasonable amount
    if (left.len() + right.len()) as f64 / hittables.len() as f64 > 1.2 {
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
