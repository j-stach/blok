
use super::*;

impl<B: Block> Row<B> {

    /// Get a reference to the block at the given index.
    pub fn get_block_ref(&self, i: usize) -> Option<&B> {
        self.get(i)
    }

    /// Get a mutable reference to the block at the given index.
    pub fn get_block_mut(&mut self, i: usize) -> Option<&mut B> {
        self.get_mut(i)
    }

    /// Get a vector of references to consecutive blocks.
    /// Returns None if the range does not exist in the row.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_range_ref(
        &self,
        start: usize,
        end: usize
    ) -> Option<Vec<&B>> {

        let total = self.blocks.len();
        if start > end || end >= total { 
            return None 
        }

        let blocks = self.blocks[start..=end].iter().collect();
        Some(blocks)
    }

    /// Get a vector of mutable references to consecutive blocks.
    /// Returns None if the range does not exist in the row.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_range_mut(
        &mut self,
        start: usize,
        end: usize
    ) -> Option<Vec<&mut B>> {

        let total = self.blocks.len();
        if start > end || end >= total { 
            return None 
        }

        let blocks = self.blocks[start..=end].iter_mut().collect();
        Some(blocks)
    }

    /// Get a vector of references to all blocks.
    /// Returns None if the range does not exist in the row.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_all_ref(&self) -> Option<Vec<&B>> {
        if self.len() == 0 { return None }
        let row_ref = self.iter().collect();
        Some(row_ref)
    }

    /// Get a vector of mutable references to all blocks.
    /// Returns None if the range does not exist in the row.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_all_mut(&mut self) -> Option<Vec<&mut B>> {
        if self.len() == 0 { return None }
        let row_ref = self.iter_mut().collect();
        Some(row_ref)
    }

}
