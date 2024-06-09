
/// The fundamental structure element for Blok.
pub trait Block: Clone {
    // TODO A lot of these are boilerplate; look into making a derive macro soon.

    /// Specify the block generator function type.
    /// For convenience, define a custom type using a fn pointer.
    // TODO BuildArgs type will be added to this constructor!!
    type Constructor: Fn() -> Self; // TODO: Needs "A" arg instructions generic

    /// Define the constructor for a block that represents empty space.
    fn void() -> Self;

    /// Checks if the block is void (has no properties).
    fn is_void(&self) -> bool;
    /// Removes the block's properties (data) while perserving its other values.
    fn to_void(&mut self);
    /// Replaces the block with a default void, overwriting all other values.
    fn to_new_void(&mut self) { *self = Self::void() }

    /// Overwrite the entire block with new data.
    fn overwrite(&mut self, other: Self) { *self = other }

    /// Swap the positions of two blocks.
    fn swap(&mut self, other: &mut Self) {
        let shelf = other.clone();
        other.overwrite(self.clone());
        self.overwrite(shelf);
    }

    /// Swaps this block with another block if the other is a void.
    fn shift(&mut self, other: &mut Self) -> Option<()> {
        match other.is_void() {
            true => { Some(self.swap(other)) },
            false => None
        }
    }
}
