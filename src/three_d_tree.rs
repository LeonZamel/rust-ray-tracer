struct ThreeDTree<T> {
    left: Option<Box<ThreeDTree<T>>>,
    right: Option<Box<ThreeDTree<T>>>,
    axis: Axis,
    split_pos: f64,
}
impl ThreeDTree<T> {
    fn 
}

enum Axis {
    X,
    Y,
    Z,
}