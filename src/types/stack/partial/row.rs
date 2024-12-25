
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

        // Efficiently handles errors and emptiness checks.
        if !row_check_helper(self, l, r)? { return Ok(None) }

        // No need to repeat layer checks.  
        let layer_start = self.find_layer_start(l)
            .expect("Layer exists")
            .expect("Layer contains blocks");
        let layout = self.layouts.get(l)
            .expect("Layout exists");

        // Uses `row_range` instead of `find_row_start` and `find_row_end`
        // because although more complicated, I think it is more efficient?
        // The Rust compiler is good so it's probably unnecessary 
        // and this code could be simplified.
        let (mut start, mut end) = layout.row_range(r)
            .expect("Row exists")
            .expect("Row contains blocks");

        // Range is relative to layer start.
        start += layer_start;
        end += layer_start;

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
    
        // Efficiently handles errors and emptiness checks.
        if !row_check_helper(self, l, r)? { return Ok(None) }

        // No need to repeat checks.  
        let layer_start = self.find_layer_start(l)
            .expect("Layer exists")
            .expect("Layer contains blocks");
        let layout = self.layouts.get(l)
            .expect("Layout exists");

        let mut start = layout.row_start(r)
            .expect("Row exists")
            .expect("Row contains blocks");

        // Row start is relative to layer start.
        start += layer_start;
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
    
        // Efficiently handles errors and emptiness checks.
        if !row_check_helper(self, l, r)? { return Ok(None) }

        // No need to repeat checks.  
        let layer_start = self.find_layer_start(l)
            .expect("Layer exists")
            .expect("Layer contains blocks");
        let layout = self.layouts.get(l)
            .expect("Layout exists");

        let mut end = layout.row_end(r)
            .expect("Row exists")
            .expect("Row contains blocks");

        // Row start is relative to layer start.
        end += layer_start;
        Ok(Some(end))
    }

    /// Get a vector of references to the blocks that represent a layer row.
    /// Returns None if the row could not be found, or an empty Vec if the row is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_row_ref(
        &self, 
        l: usize, 
        r: usize
    ) -> Option<Vec<&B>> {
        
        // Convert errors into Option for top-level ease of use.
        if let Some((start, end)) = self.find_row_bounds(l, r).ok()? {
            self.get_range_ref(start, end)
        } else {
            // Return an empty Vec if there are no blocks found.
            Some(Vec::new())
        }
    }

    /// Get a vector of mutable references to the blocks that represent a layer row.
    /// Returns None if the row could not be found, or an empty Vec if the row is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_row_mut(
        &mut self,
        l: usize,
        r: usize
    ) -> Option<Vec<&mut B>> { 

        // Convert errors into Option for top-level ease of use.
        if let Some((start, end)) = self.find_row_bounds(l, r).ok()? {
            self.get_range_mut(start, end)
        } else {
            // Return an empty Vec if there are no blocks found.
            Some(Vec::new())
        }
    }

}

/// Returns an error if the layer or row is not present.
/// Returns false if the row is empty (None).
fn row_check_helper<B: Block>(stack: &Stack<B>, l: usize, r: usize) -> anyhow::Result<bool> {

    // Error if layer not found.
    let layout = stack.layouts.get(l)
        .ok_or(anyhow::anyhow!("Layer {} is not present in the stack", l))?;

    // Error if row not found,
    if layout.row_is_empty(r)? {
        // but empty if empty.
        return Ok(false)
    }

    // The row is there and has blocks.
    Ok(true)
}

