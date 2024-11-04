
use super::*;

/// Methods for partial data access:
impl<B: Block> Layer<B> {

    /// Get the span of blocks representing the given row number.
    // TODO: Get a row, or ref a row?
    pub fn get_row(&self, row: usize) -> Option<&[B]> {
        let layout = self.layout();
        if *layout.get(row)? == 0 { 
            return None 
        }

        let start = layout.row_start(row)?;
        let end = layout.row_end(row)?;
        let row = &self.blocks()[start..=end]; 
        Some(row)
    }

    /// Get a singular block given the row and index in the layer.
    pub fn get_block(
        &self, 
        row: usize, 
        index: usize
    ) -> Option<&B> {

        if *self.layout().get(row)? <= index { 
            return None 
        };

        let mut cursor = 0usize;
        for l in &self.layout()[0..row] { 
            cursor += l 
        }

        cursor += index;
        self.blocks().get(cursor)
    }

}
