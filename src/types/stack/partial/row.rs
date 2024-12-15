

use super::*;
use crate::Block;

/// Methods for referencing interior elements:
impl<B: Block> Stack<B> {

    /// Find the block indexes for the start and end of the row.
    /// Returns None if the row does not exist or is empty.
    pub fn find_row_bounds(
        &self, 
        l: usize,
        r: usize
    ) -> Option<(usize, usize)> {

        let start = self.find_row_start(l, r)?;
        let end = self.find_row_end(l, r)?;
        Some((start, end))
    }

    /// Find the block index for the start of the row.
    /// Returns None if the row does not exist or is empty.
    pub fn find_row_start(
        &self, 
        l: usize, 
        r: usize
    ) -> Option<usize> {

        let layer_start = self.find_layer_start(l)?;
        // This returns None if the row is empty.
        let row_start = &self.layouts[l].row_start(r)?;
        Some(layer_start + row_start)
    }

    /// Find the block index for the end of the row.
    /// Returns None if the row does not exist or is empty.
    pub fn find_row_end(
        &self, 
        l: usize, 
        r: usize
    ) -> Option<usize> {

        let layer_start = self.find_layer_start(l)?;
        // This returns None if the row is empty.
        let row_end = &self.layouts[l].row_end(r)?;
        Some(layer_start + row_end)
    }

    /// Get a vector of references to the blocks that represent a layer row.
    /// Returns None if the row could not be found.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_row_ref(
        &self, 
        l: usize, 
        r: usize
    ) -> Option<Vec<&B>> { // TBD RowRef type?
        
        let (start, end) = self.find_row_bounds(l, r)?;
        self.get_range_ref(start, end)
    }

    /// Get a vector of mutable references to the blocks that represent a layer row.
    /// Returns None if the row could not be found.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_row_mut(
        &mut self,
        l: usize,
        r: usize
    ) -> Option<Vec<&mut B>> { 

        let (start, end) = self.find_row_bounds(l, r)?;
        self.get_range_mut(start, end)
    }

}



