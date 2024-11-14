
use super::*;

impl<B: Block> Row<B> {
    /// Clones the row into a new naked vector.
    pub fn clone_into_blocks(&self) -> Vec<B> {
        self.blocks.clone()
    }

    /// Set a row's values from a vector of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<B>) {
        self.blocks = blocks
    }
}
