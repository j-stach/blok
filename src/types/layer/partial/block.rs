
use crate::{ Block, Layer };


/// Methods for partial data access:
impl<B: Block> Layer<B> {

    /// Returns None if the block could not be found.
    /// Returns an error if the row doesn't exist within the layer, 
    /// or if the block does not exist within the row.
    pub fn find_block_index(
        &self,
        r: usize,
        i: usize
    ) -> anyhow::Result<usize> {

        // If the row is empty, it will not have a start or end (None).
        if let Some((start, end)) = self.find_row_bounds(r)? {
            if end - start < i {
                return Err(anyhow::anyhow!("Block index exceeds length of row"))
            }
            Ok(start + i)
        } else {
            Err(anyhow::anyhow!("Row contains no blocks"))
        }
    }

    /// Get a reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_ref(
        &self, 
        r: usize, 
        i: usize
    ) -> Option<&B> {

        if let Ok(index) = self.find_block_index(r, i) {
            let block = self.blocks.get(index).expect("Block exists");
            Some(block)
        } else {
            None
        }
    }

    /// Get a mutable reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_mut(
        &mut self, 
        r: usize, 
        i: usize
    ) -> Option<&mut B> {

        if let Ok(index) = self.find_block_index(r, i) {
            let block = self.blocks.get_mut(index).expect("Block exists");
            Some(block)
        } else {
            None
        }
    }

}



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use super::*;
    use crate::{ Block, Layer, Layout };
    use crate::types::layer::test::test_layer;

    ///
    #[test] fn find_block_index_test() {

        // Test layer layout is [1, 2]
        let layer = test_layer();

        let index1 = layer.find_block_index(0, 0);
        let index2 = layer.find_block_index(1, 1);
        assert!(index1.is_ok() && index2.is_ok());

        let bad_index = layer.find_block_index(0, 1);
        assert!(bad_index.is_err());
    }

    ///
    #[test] fn get_block_test() {

        // Test layer layout is [1, 2]
        let mut layer = test_layer();

        let ref1 = layer.get_block_ref(0, 0);
        let ref2 = layer.get_block_ref(1, 1);
        assert!(ref1.is_some() && ref2.is_some());

        let bad_ref = layer.get_block_ref(0, 1);
        assert!(bad_ref.is_none());

        let mut1 = layer.get_block_mut(0, 0);
        assert!(mut1.is_some());

        let bad_mut = layer.get_block_mut(0, 1);
        assert!(bad_mut.is_none());
    }
}

