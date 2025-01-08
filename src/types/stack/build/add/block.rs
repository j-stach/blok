
use super::*;
use crate::{ Block, Stack };

/// Methods for adding blocks:
impl<B: Block> Stack<B> {

    /// Add a block to the last row of the last layer.
    /// If the stack is empty, it will initialize a new layer and row to hold the block.
    pub fn add_block(&mut self, block: B) -> &mut Self {

        // If there are no layers in the stack, 
        // create a new one to accomodate the new block.
        if self.layouts.is_empty() {
            self.new_layer();
        }

        // Safe to expect because we created the missing layer above.
        let layout = self.layouts.last_mut()
            .expect("Layer exists");

        // If the last layer is empty, provide an empty row:
        layout.prep();

        // Add the block to the collection and the index record.
        self.blocks.push(block);
        *layout.last_mut().unwrap() += 1;

        self
    }

    /// Add a collection of blocks to the last row of the last layer.
    /// If the stack is empty, it will initialize a new layer and row to hold the blocks.
    pub fn add_blocks(&mut self, blocks: Vec<B>) -> &mut Self {

        // If there are no layers in the stack, 
        // create a new one to accomodate the new block.
        if self.layouts.is_empty() {
            self.new_layer();
        }

        // Safe to expect because we created the missing layer above.
        let layout = self.layouts.last_mut()
            .expect("Layer exists");

        // If the last layer is empty, provide an empty row:
        layout.prep();

        // Add the block to the collection and the index record.
        // Do it in this order to simplify borrowing.
        *layout.last_mut().unwrap() += blocks.len();
        for block in blocks.into_iter() {
            self.blocks.push(block)
        }

        self
    }

    /// Add a block to the last row of the given layer.
    /// If the layer is empty, it will initialize a new row to hold the block.
    pub fn add_block_to_layer(
        &mut self,
        l: usize,
        block: B
    ) -> anyhow::Result<&mut Self> {

        // This returns an error if the layer does not exist, 
        // and None if the layer is empty, so there's no need to repeat checks after this.
        let layer_end = self.find_layer_end(l)? 
            // If layer is empty, it should still be possible to add to the end of it. 
            // Use the helper function to find an index that can be used for reference.
            .unwrap_or(previous_available_layer_recursion_helper(self, l));

        // Do this here so we can immutably borrow for the recursion helper.
        let layout = self.layouts.get_mut(l)
            .expect("Layer exists");
        // If the last layer is empty, provide an empty row.
        // Using `prep` repeats checks unnecessarily but I'm lazy and it keeps the code dry.
        layout.prep();

        // Add the block to the collection and the index record.
        // Add 1 to insert after the reference index.
        self.blocks.insert(layer_end + 1, block);
        *layout.last_mut().unwrap() += 1;

        Ok(self)
    }

    /// Add a block to the end of the given row in the given layer.
    /// If the layer is empty, it will initialize a new row to hold the block.
    pub fn add_block_to_row(
        &mut self,
        l: usize,
        r: usize,
        block: B 
    ) -> anyhow::Result<&mut Self> {

        // This returns an error if the layer or row does not exist, 
        // and None if the row is empty, so there's no need to repeat checks after this.
        let row_end = self.find_row_end(l, r)?
            // If row is empty, it should still be possible to add to the end of it. 
            // Use the helper function to find an index that can be used for reference.
            .unwrap_or(previous_available_row_recursion_helper(self, l, r));

        // Do this here so we can immutably borrow for the recursion helper.
        let layout_row = self.layouts.get_mut(l)
            .expect("Layout exists")
            .get_mut(r)
            .expect("Row exists");

        // Add the block to the collection and the index record.
        // Add 1 to insert after the reference index.
        self.blocks.insert(row_end + 1, block);
        *layout_row += 1;

        Ok(self)
    }

    /// Insert a block at the specific layer, row, index.
    /// Blocks cannot be inserted where there is not already an existing block;
    /// in such a situation, use an "add" method.
    pub fn insert_block(
        &mut self,
        l: usize,
        r: usize,
        b: usize,
        block: B 
    ) -> anyhow::Result<&mut Self> {

        // This returns an error if the block does not exist, 
        // so there's no need to repeat checks after this.
        let index = self.find_block_index(l, r, b)?;

        self.blocks.insert(index, block);
        self.layouts[l][r] += 1;
        Ok(self)
    }

    /// Insert a collection of blocks beginning at the given layer, row, index.
    /// Blocks cannot be inserted where there is not already an existing block;
    /// in such a situation, use an "add" method.
    pub fn insert_blocks(
        &mut self,
        l: usize,
        r: usize,
        b: usize,
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        // This returns an error if the block does not exist, 
        // so there's no need to repeat checks after this.
        let index = self.find_block_index(l, r, b)?;

        let total = blocks.len();
        let mut tail = self.blocks.split_off(index);

        self.blocks.append(&mut blocks);
        self.blocks.append(&mut tail);
        self.layouts[l][r] += total;

        Ok(self)
    }
}



#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    use crate::types::stack::{ Stack, test::test_stack };
    
    #[test] fn add_block_test() {}
    #[test] fn add_block_to_layer_test() {}
    #[test] fn add_block_to_row_test() {}
    #[test] fn insert_block_test() {}
    
}

