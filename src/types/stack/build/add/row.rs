

use crate::{ Block, Row, Stack };

//
// TODO: Clean up and helper functions 
//

/// Methods for adding rows:
impl<B: Block> Stack<B> {
    /// Add a row to the last layer in the stack.
    pub fn add_row(
        &mut self,
        mut row: Row<B>
    ) -> &mut Self {

        if self.layouts.len() == 0 {
            self.new_layer();
        }

        self.layouts.last_mut().unwrap().push(row.len());
        self.blocks.append(&mut row);

        self
    }

    /// Add a collection of rows to the last layer in the stack.
    pub fn add_rows(
        &mut self,
        rows: Vec<Row<B>>
    ) -> &mut Self {

        for row in rows.into_iter() {
            self.add_row(row);
        }

        self
    }

    /// Add a row to the end of the given layer.
    pub fn add_row_to_layer(
        &mut self,
        l: usize,
        mut row: Row<B>
    ) -> anyhow::Result<&mut Self> {

        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layer_end = self.find_layer_end(l).unwrap();

        self.layouts[l].push(row.len());
        let mut tail = self.blocks.split_off(layer_end);

        self.blocks.append(&mut row);
        self.blocks.append(&mut tail);

        Ok(self)
    }

    /// Insert a row to the given layer, at the given index.
    pub fn insert_row(
        &mut self, 
        l: usize,
        r: usize,
        row: Row<B>
    ) -> anyhow::Result<&mut Self> {
        
        //
        // TODO
        // Use add_row_to_layer as reference to build, 
        // but debug it first
        //

        Ok(self)

    }
}

