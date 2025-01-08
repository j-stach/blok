
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



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use crate::{ Block, Layer, Row };
    use crate::block::test::TestBlock;
    use crate::types::layer::test::test_layer;

    /// Test adding row(s) to a layer.
    #[test] fn add_row_test() {

        let mut layer = test_layer();
        let new_block = |id: &str| TestBlock::create(&id.to_string());

        layer.new_row();
        assert_eq!(layer.layout[2], 0);

        let row = Row::wrap(vec![
            new_block("new 0"),
            new_block("new 1"),
            new_block("new 2")
        ]);

        layer.add_row(row);
        assert_eq!(layer.layout[3], 3);
        assert_eq!(layer.get_block_ref(3, 0).unwrap().id, "new 0");
        assert_eq!(layer.get_block_ref(3, 1).unwrap().id, "new 1");
        assert_eq!(layer.get_block_ref(3, 2).unwrap().id, "new 2");

        let row0 = Row::wrap(vec![
            new_block("newer 00"),
            new_block("newer 01"),
            new_block("newer 02")
        ]);
        let row1 = Row::wrap(vec![
            new_block("newer 10"),
            new_block("newer 11"),
            new_block("newer 12")
        ]);

        //layer.add_rows(vec![row0, row1]);
    }

    /// Test inserting row(s) to a layer.
    #[test] fn insert_row_test() {

        let mut layer = test_layer();
        let new_block = |id: &str| TestBlock::create(&id.to_string());

        let row = Row::wrap(vec![
            new_block("new 0"),
            new_block("new 1"),
            new_block("new 2")
        ]);

        assert!(layer.insert_row(2, row.clone()).is_err());
        assert!(layer.insert_row(1, row).is_ok());

        assert_eq!(layer.layout[0], 1);
        assert_eq!(layer.layout[1], 3);
        assert_eq!(layer.layout[2], 2);
        assert_eq!(layer.get_block_ref(1, 0).unwrap().id, "new 0");
        assert_eq!(layer.get_block_ref(1, 1).unwrap().id, "new 1");
        assert_eq!(layer.get_block_ref(1, 2).unwrap().id, "new 2");

        let row0 = Row::wrap(vec![
            new_block("newer 00"),
            new_block("newer 01"),
            new_block("newer 02")
        ]);
        let row1 = Row::wrap(vec![
            new_block("newer 10"),
            new_block("newer 11"),
            new_block("newer 12")
        ]);

        //layer.insert_rows(0, vec![row0, row1]);
    }

}
