
pub mod build;
pub mod void;
pub mod partial;
pub mod clone;
//pub mod connect;
//pub mod transform;

use derive_more::{ Deref, DerefMut };
use crate::Block;

/// Used to distinguish from the vector of blocks 
/// that represents the entire collection
/// (i.e. all of the blocks in a layer or stack).
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Row<B: Block> {
    pub(crate) blocks: Vec<B>
}

impl<B: Block> Row<B> {

    /// Creates an empty row upon which to build.
    pub fn new() -> Self {
        Self::default()
    }

    /// Wraps a simple vec of blocks into a formal Row type.
    pub fn wrap(blocks: Vec<B>) -> Self {
        Row { blocks }
    }

    /// Get a reference to the blocks in the row.
    /// Functions the same as deref.
    pub fn blocks(&self) -> &Vec<B> {
        &self.blocks
    }

}
