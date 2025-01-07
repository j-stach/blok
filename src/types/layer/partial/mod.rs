
pub mod block;
pub mod row;

pub mod helpers;
use helpers::*;


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



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use super::*;
    use crate::{ Block, Layer, Layout };
    use crate::types::layer::test::test_layer;

    ///
    #[test] fn get_range_test() {

        // Test layer layout is [1, 2]
        let mut layer = test_layer();

        let range_ref = layer.get_range_ref(0, 2);
        assert!(range_ref.is_some());

        let range_mut = layer.get_range_mut(1, 1);
        assert!(range_mut.is_some());

        let bad_range_ref = layer.get_range_ref(0, 3);
        assert!(bad_range_ref.is_none());

        let bad_range_mut = layer.get_range_mut(3, 2);
        assert!(bad_range_mut.is_none());

    }

    ///
    #[test] fn get_all_test() {

        // Test layer layout is [1, 2]
        let mut layer = test_layer();

        let ref_collection = layer.get_all_ref(); 
        // Should mirror the structure of the test layer (1, 2):
        assert_eq!(ref_collection.len(), 2);
        assert_eq!(ref_collection[0].len(), 1);
        assert_eq!(ref_collection[1].len(), 2);

        let mut_collection = layer.get_all_mut(); 
        // Should mirror the structure of the test layer (1, 2):
        assert_eq!(mut_collection.len(), 2);
        assert_eq!(mut_collection[0].len(), 1);
        assert_eq!(mut_collection[1].len(), 2);
    }
}

