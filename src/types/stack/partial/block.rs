
use super::*;
use crate::Block;

/// Methods for referencing interior block elements:
impl<B: Block> Stack<B> {

    /// Find the block index given its position in the stack.
    /// Returns None if the block does not exist.
    pub fn find_block_index(
        &self,
        l: usize,
        r: usize,
        i: usize
    ) -> Option<usize> {

        let (start, end) = self.find_row_bounds(l, r)?;
        if end - start > i { 
            None 
        } else {
            Some(start + i)
        }
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

}

