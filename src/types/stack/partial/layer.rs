

use super::*;
use crate::Block;

/// Methods for referencing layers of blocks::
impl<B: Block> Stack<B> {

    /// Find the block indexes for the start and end of the layer.
    /// Returns None if the layer does not exist or is empty.
    pub fn find_layer_bounds(&self, l: usize) -> Option<(usize, usize)> {
        let start = self.find_layer_start(l)?;
        let end = self.find_layer_end(l)?;
        Some((start, end))
    }

    /// Find the block index for the start of the layer.
    /// Returns None if the layer does not exist or is empty.
    pub fn find_layer_start(&self, l: usize) -> Option<usize> {

        let layouts = self.layouts();
        // If there are no blocks in the layer, return None.
        if l > layouts.len() || layouts[l].total() == 0 { 
            return None 
        }

        let mut start = 0usize;
        for layout in &layouts[0..l] {
            start += layout.total()
        }

        Some(start)
    }

    /// Find the block index for the end of the layer.
    /// Returns None if the layer does not exist or is empty.
    pub fn find_layer_end(&self, l: usize) -> Option<usize> {
        
        // No need to repeat checks.
        let layer_start = self.find_layer_start(l)?;
        let total = &self.layouts[l].total();
        Some(layer_start + total) // BUG: -1?
    }

    /// Get a vector of vectors of references to the blocks that represent a layer.
    /// Returns None if the layer could not be found.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_layer_ref(
        &self,
        l: usize
    ) -> Option<Vec<Vec<&B>>> {

        let (start, end) = self.find_layer_bounds(l)?;
        let blocks = self.get_range_ref(start, end)?;
        let layout = &self.layouts[l];
    
        // TODO Dry
        let mut rows = Vec::new();
        let mut count = 0usize;

        for r in layout.iter() {
            let mut row = Vec::new();
            for _b in 0..*r { // TBD how to implement 1-based array indexing... worth it?
                row.push(blocks[count]);
                count += 1;
            }
            rows.push(row)
        }

        Some(rows)
    }

    /// Get a vector of vectors of mutable references to the blocks that represent a layer.
    /// Returns None if the layer could not be found.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_layer_mut (
        &mut self,
        l: usize
    ) -> Option<Vec<Vec<&mut B>>> {

        let layout = self.layouts[l].clone();
        let (start, end) = self.find_layer_bounds(l)?;
        let mut blocks = self.get_range_mut(start, end)?;
    
        // TODO Dry, & sync with ref version?
        let mut rows = Vec::new();
        for r in layout.iter() {
            let remainder = blocks.split_off(*r);
            rows.push(blocks);
            blocks = remainder;
        }

        Some(rows)
    }

}

