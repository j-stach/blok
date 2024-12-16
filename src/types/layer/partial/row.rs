
use crate::{ Block, Layer };

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
