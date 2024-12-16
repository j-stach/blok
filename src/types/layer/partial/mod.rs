
pub mod block;
pub mod row;

use crate::{ Block, Layer };

/// Methods for partial data access:
impl<B: Block> Layer<B> {

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

    /// Get a matrix of references to all blocks.
    /// Returns None if the layer is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_all_ref(&mut self) -> Option<Vec<Vec<&B>>> {
        if self.layout.len() == 0 { return None }

        let mut layer_ref = Vec::new();
        let mut blocks_ref: Vec<_> = self.blocks.iter().collect();

        for r in self.layout.iter() {
            let tail = blocks_ref.split_off(*r); // DEBUG?
            layer_ref.push(blocks_ref);
            blocks_ref = tail;
        }

        Some(layer_ref)
    }


    /// Get a matrix of mutable references to all blocks.
    /// Returns None if the layer is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_all_mut(&mut self) -> Option<Vec<Vec<&mut B>>> {
        if self.layout.len() == 0 { return None }

        let mut layer_ref = Vec::new();
        let mut blocks_ref: Vec<_> = self.blocks.iter_mut().collect();

        for r in self.layout.iter() {
            let tail = blocks_ref.split_off(*r); // DEBUG?
            layer_ref.push(blocks_ref);
            blocks_ref = tail;
        }

        Some(layer_ref)
    }

}
