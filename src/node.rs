
use super::Block;

pub trait Node: Block {
    
    /// Define the argument struct for connecting blocks.
    type ConnectionInstructions: Clone + Default;

    ///
    fn connect(
        &mut self, 
        other: &mut Self, 
        instructions: &Self::ConnectionInstructions
    ); 

    // ///
    // fn disconnect(&mut self);

}
