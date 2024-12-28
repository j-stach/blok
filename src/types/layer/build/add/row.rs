
use super::*;
use crate::{ Block, Row, Layer };

/// Functions for constructing layers:
impl<B: Block> Layer<B> {

    /// Allocate a new empty row in the layer.
    pub fn new_row(&mut self) -> &mut Self { 
        self.layout_mut().push(0);
        self
    }

    /// Add a row to the end of the layer.
    pub fn add_row(&mut self, row: Row<B>) -> &mut Self {
        self.new_row()
            .add_blocks(row.to_vec());
        self
    }

    // TODO: add_rows

    /// Merge a row into the layer at the given index.
    /// Blocks cannot be inserted where there is not an existing block;
    /// in such a situation, use an "add" method instead.
    pub fn insert_row(
        &mut self,
        r: usize,
        mut row: Row<B>
    ) -> anyhow::Result<&mut Self> {

        // Check to make sure the row index exists before attempting to find previous row.
        // TBD: Insert at end is a work in progress.
        self.layout.row_exists(r)?;

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the previous row, add 1 to find the "start" of the new row.
        let index = previous_available_row_recursion_helper(self, r) + 1;

        // Record the new row in the layout.
        self.layout.insert(r, row.len());

        // Insert blocks.
        let mut tail = self.blocks.split_off(index);
        self.blocks.append(&mut row.blocks);
        self.blocks.append(&mut tail);

        Ok(self)
    }

    // TODO: insert_rows

}
