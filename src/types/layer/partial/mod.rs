
pub mod block;
pub mod row;

use crate::{ Block, Layer, Layout };

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

        // Check for bounds.
        if !range_boundary_check_helper(self.blocks.len(), start, end) {
            return None
        }

        // No need to repeat range checks.
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

        // Check for bounds.
        if !range_boundary_check_helper(self.blocks.len(), start, end) {
            return None
        }

        // No need to repeat range checks.
        let blocks = self.blocks[start..=end].iter_mut().collect();
        Some(blocks)
    }

    /// Get a matrix of references to all blocks.
    /// Returns an empty Vec if the layer is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_all_ref(&mut self) -> Vec<Vec<&B>> {
        let blocks_ref: Vec<_> = self.blocks.iter().collect();
        // Use layout to represent layer structure as nested vectors.
        collection_organization_helper::<&B>(self.layout(), blocks_ref)
    }


    /// Get a matrix of mutable references to all blocks.
    /// Returns an empty Vec if the layer is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_all_mut(&mut self) -> Vec<Vec<&mut B>> {
        let layout = self.layout().clone(); // Clone for borrowing reasons.
        let blocks_ref: Vec<_> = self.blocks.iter_mut().collect();
        // Use layout to represent layer structure as nested vectors.
        collection_organization_helper::<&mut B>(&layout, blocks_ref)
    }

}

/// Check whether the range falls within the total number of blocks.
fn range_boundary_check_helper(total: usize, start: usize, end: usize) -> bool {
    if start > end || end >= total { false } 
    else { true }
}

/// Helps to organize blocks using the layout, 
/// so that reference reflects layer structure.
/// Returns an empty Vec when the layer is empty.
fn collection_organization_helper<T>(
    layout: &Layout,
    mut blocks_ref: Vec<T>,
) -> Vec<Vec<T>> {

    let mut layer_ref = Vec::new();

    for r in self.layout.iter() {
        let tail = blocks_ref.split_off(*r); // DEBUG?
        layer_ref.push(blocks_ref);
        blocks_ref = tail;
    }
    
    layer_ref
}

