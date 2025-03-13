
pub mod build;
pub mod partial;
pub mod clone;
//pub mod connect;

use derive_more::{ Deref, DerefMut };
use crate::Block;

/// Represents a 1D row of blocks in a positional context,
/// to distinguish it from vectors of blocks that represent an entire collection
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

    /// Create a new Row of a different type of blocks using the blocks from this row.
    pub fn map<C: Block, T: Fn(&B) -> C>(&self, t: T) -> Row<C> {
        let mapped_blocks = self.blocks()
            .iter()
            .map(t)
            .collect();

        Row::wrap(mapped_blocks)
    }


}
