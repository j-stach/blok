

use super::*;
use crate::Block;

/// Methods for referencing interior elements:
impl<B: Block> Stack<B> {

    /// Find the block indexes for the start and end of the row.
    /// Returns an error if the row doesn't exist within the stack.
    /// Returns None if the row is empty.
    pub fn find_row_bounds(
        &self, 
        l: usize,
        r: usize
    ) -> anyhow::Result<Option<(usize, usize)>> {

        // TODO: Use Layout::row_range instead

        // If there are no blocks in the row, return None.
        let start = self.find_row_start(l, r)?;
        if start.is_none() {
            return Ok(None)
        }

        // No need to repeat checks if a start block is found.
        let start = start
            .expect("Layer contains blocks");
        let end = self.find_row_end(l, r)?
            .expect("Layer contains blocks");

        Ok(Some((start, end)))
    }

    /// Find the block index for the start of the row.
    /// Returns an error if the row doesn't exist within the stack.
    /// Returns None if the row is empty.
    pub fn find_row_start(
        &self, 
        l: usize, 
        r: usize
    ) -> anyhow::Result<Option<usize>> {
    
        // No need to repeat layer checks.  
        let layer_start = self.find_layer_start(l)?;
        let layout = self.layouts.get(l).expect("Layout exists");

        // If the layer exists but has no start, it should be empty,
        if layer_start.is_none() {
            // but we still have to check if the row exists.
            layout.row_exists(r)?;
            return Ok(None)
        }

        // This returns None if the row is empty.
        let row_start = layout.row_start(r)?;
        if row_start.is_none() {
            return Ok(None)
        }

        // Row start is relative to layer start.
        let start = layer_start.expect("Layer contains blocks") + row_start.expect("Row contains blocks");
        Ok(Some(start))
    }

    /// Find the block index for the end of the row.
    /// Returns an error if the row doesn't exist within the stack.
    /// Returns None if the row is empty.
    pub fn find_row_end(
        &self, 
        l: usize, 
        r: usize
    ) -> anyhow::Result<Option<usize>> {

        // No need to repeat layer checks.  
        let layer_start = self.find_layer_start(l)?;
        let layout = self.layouts.get(l).expect("Layout exists");

        // If the layer exists but has no start, it should be empty,
        if layer_start.is_none() {
            // but we still have to check if the row exists.
            layout.row_exists(r)?;
            return Ok(None)
        }

        let row_end = layout.row_end(r)?
            .expect("Row contains blocks");

        // Row end is relative to layer start.
        let end = layer_start.expect("Layer contains blocks") + row_end;
        Ok(Some(end))
    }

    /// Get a vector of references to the blocks that represent a layer row.
    /// Returns None if the row could not be found.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_row_ref(
        &self, 
        l: usize, 
        r: usize
    ) -> Option<Vec<&B>> {
        
        // Merge errors into Option for top-level ease of use.
        let (start, end) = self.find_row_bounds(l, r).ok()??;
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

        // Merge errors into Option for top-level ease of use.
        let (start, end) = self.find_row_bounds(l, r).ok()??;
        self.get_range_mut(start, end)
    }

}

// TBD Helper for find start/end checks?

