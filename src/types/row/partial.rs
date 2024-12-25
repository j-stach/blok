
use crate::{ Block, Row };

/// Methods for referencing interior elements:
impl<B: Block> Row<B> {

    /// Get a reference to the block at the given index.
    /// Returns None if the block does not exist in the row.
    pub fn get_block_ref(&self, i: usize) -> Option<&B> {
        self.get(i)
    }

    /// Get a mutable reference to the block at the given index.
    /// Returns None if the block does not exist in the row.
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

        // Check for bounds.
        if !range_boundary_check_helper(self.blocks.len(), start, end) {
            return None
        }

        // No need to repeat range checks.
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

        // Check for bounds.
        if !range_boundary_check_helper(self.blocks.len(), start, end) {
            return None
        }

        // No need to repeat range checks.
        let blocks = self.blocks[start..=end].iter_mut().collect();
        Some(blocks)
    }

    /// Get a vector of references to all blocks.
    /// Returns an empty Vec if the row is empty.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_all_ref(&self) -> Vec<&B> {
        self.iter().collect()
    }

    /// Get a vector of mutable references to all blocks.
    /// Returns an empty Vec if the row is empty.
    /// Use this for operations on a collection of blocks, not for building row structure.
    /// (Adding to this vector will not add blocks to the row.)
    pub fn get_all_mut(&mut self) -> Vec<&mut B> {
        self.iter_mut().collect()
    }

}

/// Check whether the range falls within the total number of blocks.
fn range_boundary_check_helper(total: usize, start: usize, end: usize) -> bool {
    if start > end || end >= total { false } 
    else { true }
}

