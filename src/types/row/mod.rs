
pub mod build;
pub mod partial;
pub mod clone;
//pub mod connect;
//pub mod transform;
//pub mod void;

use derive_more::{ Deref, DerefMut };
use crate::Block;

/// Used to distinguish from the vector of blocks 
/// that represents the entire collection
/// (i.e. all of the blocks in a layer or stack).
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Row<B: Block>(pub(crate) Vec<B>);

impl<B: Block> Row<B> {

    /// Wraps a simple vec of blocks into a formal Row type.
    pub fn wrap(blocks: Vec<B>) -> Self {
        Row(blocks)
    }

    /// Get a reference to the blocks in the row.
    /// Functions the same as deref.
    pub fn blocks(&self) -> &Vec<B> {
        &self.0
    }

    /// Get a mutable reference to the blocks in the row.
    /// Functions the same as deref_mut.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> {
        &mut self.0
    }

}
