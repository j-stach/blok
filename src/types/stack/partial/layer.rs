
use super::*;
use crate::Block;

/// Methods for referencing layers of blocks:
impl<B: Block> Stack<B> {

    /// Find the block indexes for the start and end of the layer.
    /// Returns an error if the layer doesn't exist within the stack.
    /// Returns None if the layer is empty.
    pub fn find_layer_bounds(
        &self, 
        l: usize
    ) -> anyhow::Result<Option<(usize, usize)>> {

        // If there are no blocks in the layer, return None.
        let start = self.find_layer_start(l)?;
        if start.is_none() {
            return Ok(None)
        }

        // No need to repeat checks if a start block is found.
        let start = start
            .expect("Layer contains blocks");
        let end = self.find_layer_end(l)?
            .expect("Layer contains blocks");

        Ok(Some((start, end)))
    }

    /// Find the block index for the start of the layer.
    /// Returns an error if the layer doesn't exist within the stack.
    /// Returns None if the layer is empty.
    pub fn find_layer_start(
        &self, 
        l: usize
    ) -> anyhow::Result<Option<usize>> {

        // Each layer has a representation within this vec.
        let layouts = self.layouts();

        // If the layer cannot be indexed, it is an error.
        if layouts.len() <= l {
            return Err(anyhow::anyhow!("The stack can not be indexed at layer {}", l))
        }

        // If there are no blocks in the layer, return None.
        if layouts[l].total() == 0 { 
            return Ok(None)
        }

        // Calculate the index for layer's first block.
        let mut start = 0usize;
        for layout in &layouts[0..l] {
            start += layout.total()
        }

        Ok(Some(start))
    }

    /// Find the block index for the end of the layer.
    /// Returns an error if the layer doesn't exist within the stack.
    /// Returns None if the layer is empty.
    pub fn find_layer_end(
        &self, 
        l: usize
    ) -> anyhow::Result<Option<usize>> {
        
        // No need to repeat checks if a start block is found.
        let layer_start = self.find_layer_start(l)?;
        if layer_start.is_none() {
            return Ok(None)
        }

        // If the layer has no blocks, None will already be returned, 
        // so we can safely subtract 1 from the total to get the last index.
        let total = &self.layouts[l].total();
        let end = layer_start.expect("Block exists") + total - 1;

        Ok(Some(end))
    }

    /// Get a vector of vectors of references to the blocks that represent a layer.
    /// Returns None if the layer could not be found, or is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_layer_ref(
        &self,
        l: usize
    ) -> Option<Vec<Vec<&B>>> {

        // Convert errors into Option for top-level ease of use.
        if let Some((start, end)) = self.find_layer_bounds(l).ok()? {

            // Get the corresponding blocks and layout.
            let blocks = self.get_range_ref(start, end)?;
            let layout = self.layouts.get(l)
                .expect("Layout exists");
        
            // Use the layout to organize blocks to reflect the layer structure.
            let rows = layer_ref_organization_helper::<&B>(layout, blocks);

            Some(rows)

        } else { 
            // Return an empty Vec if there are no blocks found.
            Some(Vec::new())
        }
    }

    /// Get a vector of vectors of mutable references to the blocks that represent a layer.
    /// Returns None if the layer could not be found, or is empty.
    /// Use this for operations on a collection of blocks, not for building stack structure.
    /// (Adding to this vector will not add blocks to the stack.)
    pub fn get_layer_mut (
        &mut self,
        l: usize
    ) -> Option<Vec<Vec<&mut B>>> {

        // Convert errors into Option for top-level ease of use.
        if let Some((start, end)) = self.find_layer_bounds(l).ok()? {

            // Clone layout here because of borrowing rules.
            // It only includes positional data so the blocks themselves are safe.
            let layout = self.layouts.get(l)
                .expect("Layout exists")
                .clone();
            // Get the corresponding blocks and layout.
            let blocks = self.get_range_mut(start, end)?;
        
            // Use the layout to organize blocks to reflect the layer structure.
            let rows = layer_ref_organization_helper::<&mut B>(&layout, blocks);

            Some(rows)

        } else { 
            // Return an empty Vec if there are no blocks found.
            Some(Vec::new())
        }
    }

}

/// Helps to organize block refs from 1D to 2D array using a layout.
/// Expects that T is either &B or &mut B.
/// Expects that the layout total is equal to the block range 
/// (i.e. the layout is not corrupted.)
fn layer_ref_organization_helper<T>(layout: &Layout, mut blocks: Vec<T>) -> Vec<Vec<T>> {

    // TODO Assert total matches length?

    let mut rows = Vec::new();
    for r in layout.iter() {
        let remainder = blocks.split_off(*r);
        rows.push(blocks);
        blocks = remainder;
    }

    rows
}



/*  UNIT TESTS  */
#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    use crate::types::stack::{ Stack, test::test_stack };

    ///
    #[test] fn find_layer_bounds_test() {

        let mut stack = test_stack();

        assert_eq!(
            stack.find_layer_bounds(0)
                .expect("Layer exists") 
                .expect("Layer has blocks"), 
            (0,2)
        );
        assert_eq!(
            stack.find_layer_bounds(1)
                .expect("Layer exists") 
                .expect("Layer has blocks"), 
            (3,5)
        );
        assert_eq!(
            stack.find_layer_bounds(2)
                .expect("Layer exists") 
                .expect("Layer has blocks"), 
            (6,8)
        );

        stack.new_layer();

        assert_eq!(
            stack.find_layer_bounds(3)
                .expect("Layer exists"),
            None
        );

    }

    ///
    #[test] fn get_layer_test() {

        let mut stack = test_stack();
        stack.new_layer();

        assert!(stack.get_layer_ref(0).is_some());
        assert!(stack.get_layer_ref(1).is_some());
        assert!(stack.get_layer_ref(2).is_some());
        assert!(stack.get_layer_ref(3).is_some());

        assert!(stack.get_layer_ref(4).is_none());
        //
    }

} 
