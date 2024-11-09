
pub mod build;
pub mod partial;
pub mod clone;
//pub mod connect;
//pub mod transform;
//pub mod void;

use crate::{ Block, Layer, Layout };


/// Holds a 3D matrix of blocks in a single vector,
/// with layer/row indexing stored in a vector of layouts.
#[derive(Debug, Default, Clone)]
pub struct Stack<B: Block> {
    /// Represents the organization of blocks in the array.
    layouts: Vec<Layout>,
    /// Blok assumes you want to store the block array contiguously in memory.
    blocks: Vec<B>
}

/// Field access methods:
impl<B: Block> Stack<B> {

    /// Get a reference to the list of layouts for the stack layers.
    pub fn layouts(&self) -> &Vec<Layout> {
        &self.layouts
    }

    /// Get a mutable reference to the list of layouts for the stack layers.
    pub(crate) fn layouts_mut(&mut self) -> &mut Vec<Layout> {
        &mut self.layouts
    }
    
    /// Get a reference to the blocks in the stack.
    pub fn blocks(&self) -> &Vec<B> {
        &self.blocks
    }
    
    /// Get a mutable reference to the blocks in the stack.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> {
        &mut self.blocks
    }

}

