
/// Implement Block for a type in order to organize instances of that type 
/// into Layers and Stacks.
pub trait Block: Clone + Default {

    /// Define the argument struct for creating a block.
    type CreationInstructions: Clone + Default;
    
    /// Define the argument struct for connecting blocks.
    type ConnectionInstructions: Clone + Default;

    /// Define the process for constructing a new block.
    fn create(instructions: &Self::CreationInstructions) -> Self;

    /// Optional: Define the process for connecting blocks.
    /// Default behavior is to do nothing.
    // TBD May make connections a trait extension.
    fn connect(
        &mut self, 
        other: &mut Self, 
        instructions: &Self::ConnectionInstructions
    ) {
        // Do nothing
    }

    /// Create a block that represents empty space.
    fn void() -> Self;

    /// Check if a block represents empty space.
    fn is_void(&self) -> bool;



    /// Replace the block with the default void, overwriting all other values.
    fn to_void(&mut self) { 
        *self = Self::void() 
    }

    /// Overwrite the entire block with new data.
    fn overwrite(&mut self, other: Self) { 
        *self = other 
    }

    /// Swap the positions of two blocks.
    fn swap(&mut self, other: &mut Self) {
        let shelf = other.clone();
        other.overwrite(self.clone());
        self.overwrite(shelf);
    }

    /// Swap this block with another if the latter is a void.
    fn shift(&mut self, other: &mut Self) -> Option<()> {
        match other.is_void() {
            true => { Some(self.swap(other)) },
            false => None
        }
    }

    // TBD Expand provided functions 

}



