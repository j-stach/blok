
use super::*;

use crate::{ Block, Row };

/// Functions for constructing layers:
impl<B: Block> Layer<B> {

    /// Add a new block to the last row in the layer.
    pub fn add_block(&mut self, block: B) {
        self.blocks_mut().push(block);
        if self.layout().len() == 0 { 
            self.new_row() 
        }
        // We can unwrap here because we check length above.
        *self.layout_mut().last_mut().unwrap() += 1;
    }

    /// Add a collection of blocks as a new row.
    pub fn add_blocks(&mut self, mut collection: Vec<B>) {
        self.layout_mut().push(collection.len());
        self.blocks_mut().append(&mut collection)
    }

    /// Add a block to the end of the given row.
    pub fn add_block_to_row(
        &mut self, 
        row: usize, 
        block: B
    ) -> anyhow::Result<&mut Self> { // TODO Error
        if self.layout().len() < row {

            let index = {
                let mut index = 0usize;
                for r_len in 0..row {
                    index += r_len
                }
                index
            };

            self.blocks_mut().insert(index, block);
            self.layout_mut()[row] += 1;
            Ok(self)

        } else { 
            Err(anyhow::anyhow!("Row {} DNE", row)) 
        }
    }

    /// Insert a block into the given row, at the given index.
    pub fn insert_block(
        &mut self,
        row: usize,
        index: usize,
        block: B
    ) -> anyhow::Result<&mut Self> { // TODO Error
        let row_max = self.layout().len();
        if row_max > row && row_max > 0  {

            let index = {
                let mut i = 0usize;
                for r_len in 0..row - 1 {
                    i += r_len
                }
                let row_target = self.layout()[row];
                if row_target > index {
                    i += index;
                    i
                } else {
                    return Err(anyhow::anyhow!("Row {} could not be indexed at {}", row, index)) // TODO Error
                }
            };

            self.blocks_mut().insert(index, block);
            self.layout_mut()[row] += 1;
            Ok(self)

        } else { 
            Err(anyhow::anyhow!("Row {} could not be indexed", row))
        }
    }

    /// Insert a collection of blocks as a new row at the given index.
    pub fn insert_blocks(
        &mut self, 
        index: usize, 
        collection: Vec<B>
    ) {
        if self.layout().len() >= index {
            // TBD This may be unnecessary
            self.layout_mut().insert(index, collection.len());
            
            let mut rows = self.clone_into_blocks();
            rows.insert(index, collection);
            
            self.set_from_blocks(rows);
        }
    }

    /// Allocate a new row in the layer.
    pub fn new_row(&mut self) { 
        self.layout_mut().push(0) 
    }

    /// Add a row to the end of the layer.
    pub fn add_row(&mut self, row: Row<B>) {
        self.add_blocks(row.to_vec())
    }

    /// Merge a row into the layer at the given index.
    pub fn insert_row(
        &mut self,
        index: usize,
        row: Row<B>
    ) {
        self.insert_blocks(index, row.to_vec())
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
