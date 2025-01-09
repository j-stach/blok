
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
        range_boundary_check_helper(self.blocks.len(), start, end)?;

        // No need to repeat range checks.
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
        range_boundary_check_helper(self.blocks.len(), start, end)?;

        // No need to repeat range checks.
        let blocks = self.blocks[start..=end].iter_mut().collect();
        Some(blocks)
    }

    /// Get a matrix of references to all blocks.
    /// Returns an empty Vec if the stack is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_all_ref(&self) -> Vec<Vec<Vec<&B>>> {
        let blocks_ref: Vec<_> = self.blocks.iter().collect();
        // Use layouts to represent stack structure as nested vectors.
        collection_organization_helper::<&B>(self.layouts(), blocks_ref)
    }

    /// Get a matrix of mutable references to all blocks.
    /// Returns an empty Vec if the stack is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_all_mut(&mut self) -> Vec<Vec<Vec<&mut B>>> {
        let layouts = self.layouts().clone(); // Clone for borrowing reasons.
        let blocks_ref: Vec<_> = self.blocks.iter_mut().collect();
        // Use layouts to represent stack structure as nested vectors.
        collection_organization_helper::<&mut B>(&layouts, blocks_ref)
    }

}


/// Check whether the range falls within the total number of blocks.
fn range_boundary_check_helper(total: usize, start: usize, end: usize) -> Option<()> {
    if start > end || end >= total { None } 
    else { Some(()) }
}

/// Helps to organize blocks using layouts, 
/// so that reference reflects stack structure.
/// Returns an empty Vec when the stack is empty.
fn collection_organization_helper<T>(
    layouts: &Vec<Layout>,
    mut blocks_ref: Vec<T>,
) -> Vec<Vec<Vec<T>>> {

    let mut stack_ref = Vec::new();

    for layout in layouts.iter() {

        let mut layer_ref = Vec::new();

        let tail = blocks_ref.split_off(layout.total());
        let mut layer_blocks_ref = blocks_ref;
        blocks_ref = tail;

        for r in layout.iter() {
            let tail = layer_blocks_ref.split_off(*r);
            layer_ref.push(layer_blocks_ref);
            layer_blocks_ref = tail;
        }

        stack_ref.push(layer_ref)
    
    }

    stack_ref
}



#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    use crate::types::stack::{ Stack, test::test_stack };
 
    ///
    #[test] fn get_range_test() {

        let mut stack = test_stack();

        assert!(stack.get_range_ref(0, 2).is_some());
        assert!(stack.get_range_ref(1, 2).is_some());
        assert!(stack.get_range_ref(8, 8).is_some());

        assert!(stack.get_range_mut(0, 0).is_some());
        assert!(stack.get_range_mut(0, 1).is_some());

        assert!(stack.get_range_mut(1, 0).is_none());
        assert!(stack.get_range_ref(0, 10).is_none());

    }

    ///
    #[test] fn get_all_test() {

        let mut stack = test_stack();

        let stack_ref = stack.get_all_ref();
        assert_eq!(stack_ref.len(), stack.layouts.len());
        assert_eq!(stack_ref[0].len(), 2, "Each layer has 2 rows");
        assert_eq!(stack_ref[1].len(), 2, "Each layer has 2 rows");
        assert_eq!(stack_ref[2].len(), 2, "Each layer has 2 rows");
        //
    }

}
