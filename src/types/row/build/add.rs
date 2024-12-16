
use crate::{ Block, Row };

/// Represents a 1D row of blocks in a positional context,
/// to distinguish it from other vectors of blocks.
impl<B: Block> Row<B> {

    /// Add a block to the end of the row. 
    pub fn add_block(&mut self, block: B) -> &mut Self {
        self.push(block);
        self
    }

    /// Add a collection of blocks to the end of the row.
    pub fn add_blocks(&mut self, mut blocks: Vec<B>) -> &mut Self {
        self.append(&mut blocks);
        self
    }

    /// Insert a block into the row at the given index.
    pub fn insert_block(
        &mut self, 
        i: usize, 
        block: B
    ) -> anyhow::Result<&mut Self> {

        if self.len() < i {
            return Err(anyhow::anyhow!("Bad block index"))
        }

        self.insert(i, block);
        Ok(self)
    }
    
    /// Insert a collection of blocks into the row at the given index.
    pub fn insert_blocks(
        &mut self, 
        i: usize, 
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        if self.len() < i {
            return Err(anyhow::anyhow!("Bad block index"))
        }

        let mut tail = self.split_off(i);
        self.append(&mut blocks);
        self.append(&mut tail);
        Ok(self)
    }

}
