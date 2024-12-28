
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

