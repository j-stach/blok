
use super::*;

/// Methods for partial data access:
impl<B: Block> Layer<B> {

    /// Returns None if the row is empty or could not be found.
    pub fn find_row_bounds(&self, r: usize) -> Option<(usize, usize)> {
        // TODO Handle this as part of layout instead of running twice
        let start = self.find_row_start(r)?;
        let end = self.find_row_end(r)?;
        Some((start, end))
    }

    /// Finds the block index for the start of the row.
    /// Returns None if the row is empty or could not be found.
    pub fn find_row_start(&self, r: usize) -> Option<usize> {
        self.layout.row_start(r)
    }

    /// Finds the block index for the last in the row.
    /// Returns None if the row is empty or could not be found.
    pub fn find_row_end(&self, r: usize) -> Option<usize> {
        self.layout.row_end(r)
    }

    /// Finds the index for a block given its placement in the layer.
    /// Returns None if the block could not be found.
    pub fn find_block_index(
        &self,
        r: usize,
        i: usize
    ) -> Option<usize> {

        let (start, end) = self.find_row_bounds(r)?;
        if end - start < i {
            return None
        }

        Some(start + i)
    }

    /// Get a reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_ref(
        &self, 
        r: usize, 
        i: usize
    ) -> Option<&B> {

        let index = self.find_block_index(r, i)?;
        let block = &self.blocks[index];
        Some(block)
    }

    /// Get a mutable reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_mut(
        &mut self, 
        r: usize, 
        i: usize
    ) -> Option<&mut B> {

        let index = self.find_block_index(r, i)?;
        let block = &mut self.blocks[index];
        Some(block)
    }

    /// Get a vector of references to consecutive blocks.
    /// Returns None if the range does not exist in the layer.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_range_ref(
        &self,
        start: usize,
        end: usize,
    ) -> Option<Vec<&B>> {

        let total = self.blocks.len();
        if start > end || end >= total { 
            return None 
        }

        let blocks = self.blocks[start..=end].iter().collect();
        Some(blocks)
    }

    /// Get a vector of mutable references to consecutive blocks.
    /// Returns None if the range does not exist in the layer.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
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

    /// Get a vector of references to the blocks that represent a layer row.
    /// Returns None if the row could not be found.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_row_ref(&self, r: usize) -> Option<Vec<&B>> {
        let (start, end) = self.find_row_bounds(r)?;
        let row = self.get_range_ref(start, end)?;
        Some(row)
    }

    /// Get a vector of mutable references to the blocks that represent a layer row.
    /// Returns None if the row could not be found.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_row_mut(&mut self, r: usize) -> Option<Vec<&mut B>> {
        let (start, end) = self.find_row_bounds(r)?;
        let row = self.get_range_mut(start, end)?;
        Some(row)
    }

}
