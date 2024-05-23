

/// Specify the type to represent a block's internal data.
pub trait Props: Clone {
    fn merge(&mut self, other: &mut Self);
    // set? get?
}
