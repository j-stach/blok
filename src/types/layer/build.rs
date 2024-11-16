
use super::*;

use crate::{ Block, Row };

/// Functions for constructing layers:
impl<B: Block> Layer<B> {

    /// Add a new block to the last row in the layer.
    pub fn add_block(&mut self, block: B) -> &mut Self {

        if self.layout().len() == 0 { 
            self.new_row(); 
        }
        // We can unwrap here because we check length above.
        *self.layout_mut().last_mut().unwrap() += 1;
        self.blocks_mut().push(block);

        self
    }

    /// Add a collection of blocks as a new row.
    pub fn add_blocks(&mut self, mut blocks: Vec<B>) -> &mut Self {
        self.layout_mut().push(blocks.len());
        self.blocks_mut().append(&mut blocks);
        self
    }

    /// Add a block to the end of the given row.
    pub fn add_block_to_row(
        &mut self, 
        r: usize, 
        block: B
    ) -> anyhow::Result<&mut Self> {

        if self.layout().len() < r {
            return Err(anyhow::anyhow!("Row {} DNE", r)) 
        }

        let index = {
            let mut index = 0usize;
            for rr in 0..r {
                index += rr
            }
            index
        };

        self.blocks_mut().insert(index, block);
        self.layout_mut()[r] += 1;

        Ok(self)
    }

    /// Add a block to the end of the given row.
    pub fn add_blocks_to_row(
        &mut self, 
        r: usize, 
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        if self.layout().len() < r {
            return Err(anyhow::anyhow!("Row {} DNE", r)) 
        }

        let index = {
            let mut index = 0usize;
            for rr in 0..r {
                index += rr
            }
            index
        };

        self.layout_mut()[r] += blocks.len();
        self.blocks_mut().append(&mut blocks);

        Ok(self)
    }

    /// Insert a block into the given row, at the given index.
    pub fn insert_block(
        &mut self,
        r: usize,
        i: usize,
        block: B
    ) -> anyhow::Result<&mut Self> { 
        
        let index = self.find_block_index(r, i);
        if index.is_none() { 
            return Err(anyhow::anyhow!("Block index does not exist"))
        }

        self.layout[r] += 1;
        // Unwrap is safe because we check for it above.
        self.blocks.insert(index.unwrap(), block);

        Ok(self)
    }

    /// Insert a collection of blocks at the given index, in the given row.
    pub fn insert_blocks(
        &mut self, 
        r: usize, 
        i: usize, 
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        let index = self.find_block_index(r, i);
        if index.is_none() { 
            return Err(anyhow::anyhow!("Block index does not exist"))
        }

        let total = blocks.len();
        // Unwrap is safe because we check for it above.
        let mut tail = self.blocks.split_off(index.unwrap());

        self.blocks.append(&mut blocks);
        self.blocks.append(&mut tail);
        self.layout[r] += total;

        Ok(self)
    }

    /// Allocate a new row in the layer.
    pub fn new_row(&mut self) -> &mut Self { 
        self.layout_mut().push(0);
        self
    }

    /// Add a row to the end of the layer.
    pub fn add_row(&mut self, row: Row<B>) -> &mut Self {
        self.add_blocks(row.to_vec());
        self
    }

    /// Merge a row into the layer at the given index.
    pub fn insert_row(
        &mut self,
        r: usize,
        row: Row<B>
    ) -> anyhow::Result<&mut Self> {
        self.insert_blocks(r, 0, row.to_vec())
    }

    /// Create blocks using the given constructor,
    /// adding them in rows according to the given layout.
    pub fn populate(
        &mut self, 
        mut layout: Layout, 
        instructions: &B::CreationInstructions
    ) -> &mut Self {

        for row in layout.iter() {
            for _ in 0..*row {
                let block = B::create(instructions);
                self.blocks_mut().push(block)
            }
        }

        self.layout_mut().append(&mut layout);
        self
    }

    /// Create blocks by cloning a prototype,
    /// adding them in rows according to the given layout.
    pub fn populate_with_clones(
        &mut self, 
        mut layout: Layout, 
        block: &B
    ) -> &mut Self {

        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(block.clone());
            }
        }

        self.layout_mut().append(&mut layout);
        self
    }

    // TODO! Needs `disconnect` method for Block trait

    ///// Remove the block at the given row, index.
    //pub fn remove_block(
    //    &mut self,
    //    r: usize,
    //    i: usize
    //) -> anyhow::Result<()> {
    //    // TODO
    //    todo!{}
    //}

    ///// Remove the row from the given row.
    //pub fn remove_row(&mut self, r: usize) -> anyhow::Result<()> {
    //    // TODO
    //    todo!{}
    //}

}
