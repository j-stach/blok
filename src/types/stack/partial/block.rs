
use super::*;
use crate::Block;

/// Methods for referencing interior block elements:
impl<B: Block> Stack<B> {

    /// Find the block index given its position in the stack.
    /// Returns an error if the block does not exist at that index.
    pub fn find_block_index(
        &self,
        l: usize,
        r: usize,
        b: usize
    ) -> anyhow::Result<usize> {

        let row_bounds = self.find_row_bounds(l, r)?;
        // If the row contains blocks,
        if let Some((start, end)) = row_bounds {
            // and if the index is within the row,
            if end - start >= b { 
                // Calculate the index for the block.
                let index = start + b;
                return Ok(index)
            }
        }

        // TODO Descriptive errors
        Err(anyhow::anyhow!("Block index does not exist"))
    }

    /// Get a reference to the block at the given index.
    /// Returns None if the block could not be found.
    pub fn get_block_ref(
        &self,
        l: usize,
        r: usize,
        b: usize
    ) -> Option<&B> {

        let index = self.find_block_index(l, r, b).ok()?;
        let block = &self.blocks[index];
        Some(block)
    }

    /// Get a mutable reference to the block at the given index.
    /// Returns None if the block could not be found.
    pub fn get_block_mut(
        &mut self,
        l: usize,
        r: usize,
        b: usize
    ) -> Option<&mut B> {

        let index = self.find_block_index(l, r, b).ok()?;
        let block = &mut self.blocks[index];
        Some(block)
    }

}

