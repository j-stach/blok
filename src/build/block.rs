
use crate::Props;

/// The fundamental structure element for Blok.
pub trait Block<'b, P: Props>: Clone {
    // TODO A lot of these are boilerplate; look into making a derive macro soon.

    /// Specify the block generator function type.
    /// For convenience, define a custom type using a fn pointer.
    type Constructor: Fn() -> Self;

    /// Define how properties are read from the block.
    /// For convenience, store in a field rather than a new, separate representation.
    fn properties(&self) -> &Option<P>;
    /// Define how properties are mutably read from the block.
    /// For convenience, store in a field rather than a new, separate representation.
    fn properties_mut(&mut self) -> &mut Option<P>;

    /// Define the constructor for a block that represents empty space.
    fn void() -> Self;
    /// Define the check for a void block.
    fn is_void(&self) -> bool;
    /// Removes the block's properties (data) while perserving its structural effect.
    fn to_void(&mut self) { *self = Self::void() }

    /// Merge the properties of another block into this one.
    // TODO overload + for merge?
    fn merge(&mut self, other: &mut Self) {
        if self.properties().is_some() && other.properties().is_some() {
            let other_props = other.properties_mut().as_mut().unwrap();
            self.properties_mut().as_mut().unwrap().merge(other_props)
        } else if other.properties().is_some() {
            let other_props = other.properties_mut().as_mut().unwrap();
            self.transmute(other_props)
        }
    }

    /// Change the block's properties to some new value(s).
    fn transmute(&mut self, new_properties: &P) {
        *self.properties_mut() = Some(new_properties.clone())
    }

    /// Overwrite the entire block with new data.
    fn overwrite(&mut self, other: Self) { *self = other }

    /// Swap the positions of two blocks.
    fn swap(&mut self, other: &mut Self) {
        let shelf = other.clone();
        other.overwrite(self.clone());
        self.overwrite(shelf);
    }
}
