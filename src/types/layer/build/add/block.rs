
use crate::{ Block, Layer };

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
        
        let mut tail = self.blocks_mut().split_off(index);
        let head = self.blocks_mut();
        head.append(&mut blocks);
        head.append(&mut tail);

        Ok(self)
    }

    /// Insert a block into the given row, at the given index.
    pub fn insert_block(
        &mut self,
        r: usize,
        i: usize,
        block: B
    ) -> anyhow::Result<&mut Self> { 
        
        // BUG
        let index = self.find_block_index(r, i);
        // BUG
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

}
