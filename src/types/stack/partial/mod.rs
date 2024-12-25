
pub mod block;
pub mod row;
pub mod layer;


use super::*;
use crate::Block;


//
// TODO:
// - References for vertical slices.
// - Adjacent blocks 
// -- OR!! -- 
// - A path-walking module for building different collections.
//


/// Methods for referencing interior elements:
impl<B: Block> Stack<B> {

    /// Get a vector of references to consecutive blocks.
    /// Returns None if the range does not exist in the stack.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_range_ref(
        &self,
        start: usize,
        end: usize
    ) -> Option<Vec<&B>> {

        // Check for bounds.
        let total = self.blocks.len();
        if start > end || end >= total { 
            return None 
        }

        let blocks = self.blocks[start..=end].iter().collect();
        Some(blocks)
    }

    /// Get a vector of mutable references to consecutive blocks.
    /// Returns None if the range does not exist in the stack.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_range_mut(
        &mut self,
        start: usize,
        end: usize
    ) -> Option<Vec<&mut B>> {

        // Check for bounds.
        let total = self.blocks.len();
        if start > end || end >= total { 
            return None 
        }

        let blocks = self.blocks[start..=end].iter_mut().collect();
        Some(blocks)
    }

    /// Get a matrix of references to all blocks.
    /// Returns None if the stack is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_all_ref(&self) -> Option<Vec<Vec<Vec<&B>>>> {
        if self.layouts.len() == 0 { return None }

        let mut stack_ref = Vec::new();
        let mut blocks_ref: Vec<_> = self.blocks.iter().collect();

        for layout in self.layouts.iter() {

            let mut layer_ref = Vec::new();

            let tail = blocks_ref.split_off(layout.total()); // DEBUG?
            let mut layer_blocks_ref = blocks_ref;
            blocks_ref = tail;

            for r in layout.iter() {
                let tail = layer_blocks_ref.split_off(*r); // DEBUG?
                layer_ref.push(layer_blocks_ref);
                layer_blocks_ref = tail;
            }

            stack_ref.push(layer_ref)
        
        }

        Some(stack_ref)
    }

    /// Get a matrix of mutable references to all blocks.
    /// Returns None if the stack is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_all_mut(&mut self) -> Option<Vec<Vec<Vec<&mut B>>>> {
        if self.layouts.len() == 0 { return None }

        let mut stack_ref = Vec::new();
        let mut blocks_ref: Vec<_> = self.blocks.iter_mut().collect();

        for layout in self.layouts.iter() {

            let mut layer_ref = Vec::new();

            let tail = blocks_ref.split_off(layout.total()); // DEBUG?
            let mut layer_blocks_ref = blocks_ref;
            blocks_ref = tail;

            for r in layout.iter() {
                let tail = layer_blocks_ref.split_off(*r); // DEBUG?
                layer_ref.push(layer_blocks_ref);
                layer_blocks_ref = tail;
            }

            stack_ref.push(layer_ref)
        
        }

        Some(stack_ref)
    }

}

// TODO 
// range_collection_helper
// all_collection_helper
// with iter and iter_mut insertable


