
use crate::{ Block, Stack };

//
// TODO: Clean up and helper functions 
//


/// Methods for adding blocks:
impl<B: Block> Stack<B> {

    /// Add a block to the last row of the last layer.
    pub fn add_block(&mut self, block: B) -> &mut Self {
        if self.layouts.is_empty() {
            self.new_layer();
        }

        let layout = self.layouts.last_mut().unwrap();
        // TODO function for this:
        if layout.is_empty() { layout.push(0) }
        *layout.last_mut().unwrap() += 1;

        self.blocks.push(block);

        self
    }

    /// Add a collection of blocks to the last row of the last layer.
    pub fn add_blocks(&mut self, blocks: Vec<B>) -> &mut Self {
        for block in blocks.into_iter() {
            self.add_block(block);
        }
        self
    }

    /// Add a block to the last row of the given layer.
    pub fn add_block_to_layer(
        &mut self,
        l: usize,
        block: B
    ) -> anyhow::Result<&mut Self> {

        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layout = &mut self.layouts[1];
        // TODO function for this:
        if layout.is_empty() { layout.push(0) }
        *layout.last_mut().unwrap() += 1;

        let layer_end = self.find_layer_end(l).unwrap(); 
        
        self.blocks.insert(layer_end, block);
        Ok(self)
    }

    /// Add a block to the end of the given row in the given layer.
    pub fn add_block_to_row(
        &mut self,
        l: usize,
        r: usize,
        block: B 
    ) -> anyhow::Result<&mut Self> {

        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let row_end = self.find_row_end(l, r);
        let row_layout = self.layouts[1].get_mut(r);

        if row_end.is_none() || row_layout.is_none() {
            return Err(anyhow::anyhow!("Row does not exist")) 
        }

        self.blocks.insert(row_end.unwrap() + 1, block);
        *row_layout.unwrap() += 1;

        Ok(self)
    }

    /// Insert a block at the specific layer, row, index.
    pub fn insert_block(
        &mut self,
        l: usize,
        r: usize,
        i: usize,
        block: B 
    ) -> anyhow::Result<&mut Self> {

        let index = self.find_block_index(l, r, i);
        if index.is_none() {
            return Err(anyhow::anyhow!("Index does not exist"))
        }

        self.blocks.insert(index.unwrap(), block);
        self.layouts[l][r] += 1;
        Ok(self)
    }

    /// Insert a collection of blocks beginning at the given layer, row, index.
    pub fn insert_blocks(
        &mut self,
        l: usize,
        r: usize,
        i: usize,
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        let index = self.find_block_index(l, r, i);
        if index.is_none() {
            return Err(anyhow::anyhow!("Index does not exist"))
        }

        let total = blocks.len();
        let mut tail = self.blocks.split_off(index.unwrap());

        self.blocks.append(&mut blocks);
        self.blocks.append(&mut tail);
        self.layouts[l][r] += total;

        Ok(self)
    }
}


