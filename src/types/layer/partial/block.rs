
use crate::{ Block, Layer };

/// Methods for partial data access:
impl<B: Block> Layer<B> {

    /// Returns None if the block could not be found.
    pub fn find_block_index(
        &self,
        r: usize,
        i: usize
    ) -> Option<usize> {

        let (start, end) = self.find_row_bounds(r)?;
        if end - start < i {
            return None
        }

        Some(start + i)
    }

    /// Get a reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_ref(
        &self, 
        r: usize, 
        i: usize
    ) -> Option<&B> {

        let index = self.find_block_index(r, i)?;
        let block = &self.blocks[index];
        Some(block)
    }

    /// Get a mutable reference to a block at the given row and index.
    /// Returns None if the block could not be found.
    pub fn get_block_mut(
        &mut self, 
        r: usize, 
        i: usize
    ) -> Option<&mut B> {

        let index = self.find_block_index(r, i)?;
        let block = &mut self.blocks[index];
        Some(block)
    }

}

