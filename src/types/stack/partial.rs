
use super::*;
use crate::{ Block, Row, Layer };


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

    /// Get a reference to the block at the given index.
    /// Returns None if the block could not be found.
    pub fn get_block_ref(
        &self,
        l: usize,
        r: usize,
        i: usize
    ) -> Option<&B> {

        let row_start = self.find_row_start(l, r)?;

        if i >= self.layouts[l][r] {
            return None
        }

        let block = &self.blocks[row_start + i];
        Some(block)
    }

    /// Get a vector of references to consecutive blocks.
    /// Returns None if the range does not exist in the stack.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
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
            for i in 0..*r { // TBD how to implement 1-based array indexing... worth it?
                row.push(blocks[count]);
                count += 1;
            }
            rows.push(row)
        }

        Some(rows)
    }

    /// Get a mutable reference to the block at the given index.
    /// Returns None if the block could not be found.
    pub fn get_block_mut(
        &mut self,
        l: usize,
        r: usize,
        i: usize
    ) -> Option<&mut B> {

        let row_start = self.find_row_start(l, r)?;

        if i >= self.layouts[l][r] {
            return None
        }

        let block = &mut self.blocks[row_start + i];
        Some(block)
    }

    /// Get a vector of references to consecutive blocks.
    /// Returns None if the range does not exist in the stack.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
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
        let blocks = self.get_range_mut(start, end)?;
    
        // TODO Dry
        let mut rows = Vec::new();
        let mut count = 0usize;

        for r in layout.iter() {
            let mut row = Vec::new();
            for i in 0..*r { // TBD how to implement 1-based array indexing... worth it?
                row.push(blocks[count]);
                count += 1;
            }
            rows.push(row)
        }

        Some(rows)
    }


    //
    // TODO:
    // - Correspinding indices by layer, by row in layer, etc, for vertical slices.
    // - Adjacent blocks -- OR!! -- A path-walking module for building different collections.
    //


    /// Clone a layer from the stack and return it as a new structure.
    pub fn clone_layer(&self, l: usize) -> Option<Layer<B>> {

        let layout = self.layouts.get(l)?;
        let (start, end) = self.find_layer_bounds(l)?;

        let mut layer = Layer::default();
        // TODO Revisit this:
        layer.set_from_layout(
            layout.clone(), 
            self.blocks()[start..end].to_vec()
        )
        .unwrap();

        Some(layer)
    }

    /// Clone a row from the stack and return it as a new structure.
    pub fn clone_row (
        &self, 
        l: usize,
        r: usize
    ) -> Option<Row<B>> {

        let blocks: Vec<B> = self.get_row_ref(l, r)?
            .into_iter()
            .map(|b| b.clone())
            .collect();

        let row = Row::wrap(blocks);
        Some(row)
    }

    /// Clone a block from the stack and return it as a new structure.
    pub fn clone_block (
        &self, 
        l: usize,
        r: usize,
        i: usize
    ) -> Option<B> {

        let block = self.get_block_ref(l, r, i)?;
        Some(block.clone())
    }

}

