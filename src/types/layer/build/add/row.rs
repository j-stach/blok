
use crate::{ Block, Row, Layer };

/// Functions for constructing layers:
impl<B: Block> Layer<B> {

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

    // TODO: add_rows

    /// Merge a row into the layer at the given index.
    pub fn insert_row(
        &mut self,
        r: usize,
        row: Row<B>
    ) -> anyhow::Result<&mut Self> {
        self.insert_blocks(r, 0, row.to_vec())
    }

    // TODO: insert_rows

}
