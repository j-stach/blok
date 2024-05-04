
use crate::Props;

pub trait Block<'b>: Clone {
    type Constructor: Fn() -> Self;
    type Props: Props;
    /// Define the constructor for a block that represents empty space.
    fn void() -> Self;
    fn is_void(&self) -> bool;

    fn properties(&self) -> Option<Self::Props>;


    // functions to merge, move, replace properties, and trait for properties
}
