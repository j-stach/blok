
use super::*;
use crate::{ Block, Row, Stack };


/// Methods for adding rows:
impl<B: Block> Stack<B> {
    /// Add a row to the last layer in the stack.
    pub fn add_row(
        &mut self,
        mut row: Row<B>
    ) -> &mut Self {

        // If there are no layers in the stack, 
        // create a new one to accomodate the new block.
        if self.layouts.is_empty() {
            self.new_layer();
        }

        // Add blocks to collection and record the row.
        // Do it in this order to simplify borrowing.
        self.layouts.last_mut()
            .expect("Stack contains layers")
            .push(row.len());
        self.blocks.append(&mut row);

        self
    }

    /// Add a collection of rows to the last layer in the stack.
    pub fn add_rows(
        &mut self,
        rows: Vec<Row<B>>
    ) -> &mut Self {

        // If there are no layers in the stack, 
        // create a new one to accomodate the new block.
        if self.layouts.is_empty() {
            self.new_layer();
        }

        // Extract row lengths & flatten blocks.
        let mut lengths = rows.iter().map(|r| r.len()).collect();
        let mut blocks = rows.into_iter().flat_map(|r| r.blocks).into_iter().collect();

        // Add blocks.
        self.blocks.append(&mut blocks);

        // Record the rows.
        self.layouts.last_mut()
            .expect("Stack contains layers")
            .append(&mut lengths);

        self
    }

    /// Add a row to the end of the given layer.
    pub fn add_row_to_layer(
        &mut self,
        l: usize,
        mut row: Row<B>
    ) -> anyhow::Result<&mut Self> {

        // Helper function requires we check first.
        // Do it in this order to simplify borrowing.
        self.layouts.get_mut(l)
            .ok_or(anyhow::anyhow!("Stack could not be indexed at layer {}", l))?
            .push(row.len());

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the current layer.
        let index = previous_available_layer_recursion_helper(self, l + 1) + 1; // TEST / DEBUG

        let mut tail = self.blocks.split_off(index);
        self.blocks.append(&mut row);
        self.blocks.append(&mut tail);

        Ok(self)
    }

    /// Insert a row to the given layer, at the given index.
    /// Blocks cannot be inserted where there is not an existing block;
    /// in such a situation, use an "add" method instead.
    ///
    /// \*i.e. in the last position
    pub fn insert_row(
        &mut self, 
        l: usize,
        r: usize,
        mut row: Row<B>
    ) -> anyhow::Result<&mut Self> {

        // Check for layer's existence.
        let layout = self.layouts.get_mut(l)
            .ok_or(anyhow::anyhow!("Stack could not be indexed at layer {}", l))?;

        // Lazy idk
        layout.prep();
        layout.insert(r, row.len());

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the previous layer.
        let index = previous_available_row_recursion_helper(self, l, r) + 1;

        let mut tail = self.blocks.split_off(index);
        self.blocks.append(&mut row.blocks);
        self.blocks.append(&mut tail);

        Ok(self)

    }

    /// Insert multiple rows to the given layer, at the given index.
    /// Blocks cannot be inserted where there is not an existing block;
    /// in such a situation, use an "add" method instead.
    ///
    /// \*i.e. in the last position
    pub fn insert_rows(
        &mut self, 
        l: usize,
        r: usize,
        rows: Vec<Row<B>>
    ) -> anyhow::Result<&mut Self> {

        // Do it in this order to simplify borrowing.
        let (mut lengths, blocks): (Vec<usize>, Vec<Vec<B>>) = rows.into_iter()
            .map(|row| (row.len(), row.blocks))
            .unzip();

        let mut blocks = blocks.into_iter().flatten().collect();

        // Helper function requires we check first.
        let layout = self.layouts.get_mut(l)
            .ok_or(anyhow::anyhow!("Stack could not be indexed at layer {}", l))?;

        if layout.len() <= r {
            return Err(anyhow::anyhow!("Layer {} could not be indexed at row {}", l, r))
        }
            
        // Record the rows.
        let mut tail = layout.split_off(r);
        layout.append(&mut lengths);
        layout.append(&mut tail);

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the previous layer.
        let index = previous_available_row_recursion_helper(self, l, r) + 1;

        // Insert blocks.
        let mut tail = self.blocks.split_off(index);
        self.blocks.append(&mut blocks);
        self.blocks.append(&mut tail);

        Ok(self)
    }

}



#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    use crate::types::stack::{ Stack, test::test_stack };

    #[test] fn add_row_test() {}

    #[test] fn add_row_to_layer_test() {}

    #[test] fn insert_row_test() {}
    
}

