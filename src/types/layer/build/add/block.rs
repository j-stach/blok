
use super::*;
use crate::{ Block, Layer };

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

    /// Add a collection of blocks to the last row in the layer.
    pub fn add_blocks(&mut self, mut blocks: Vec<B>) -> &mut Self {

        if self.layout().len() == 0 { 
            self.new_row(); 
        }

        // We can unwrap here because we check length above.
        *self.layout_mut().last_mut().unwrap() += blocks.len();
        self.blocks_mut().append(&mut blocks);

        self
    }

    /// Add a block to the end of the given row.
    pub fn add_block_to_row(
        &mut self, 
        r: usize, 
        block: B
    ) -> anyhow::Result<&mut Self> {

        let row_end = self.find_row_end(r)?
            .unwrap_or(previous_available_row_recursion_helper(self, r));

        self.blocks_mut().insert(row_end + 1, block);
        self.layout_mut()[r] += 1;

        Ok(self)
    }

    /// Add a block to the end of the given row.
    pub fn add_blocks_to_row(
        &mut self, 
        r: usize, 
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        let row_end = self.find_row_end(r)?
            .unwrap_or(previous_available_row_recursion_helper(self, r));

        self.layout_mut()[r] += blocks.len();
        
        let mut tail = self.blocks_mut().split_off(row_end + 1);
        let head = self.blocks_mut();
        head.append(&mut blocks);
        head.append(&mut tail);

        Ok(self)
    }

    /// Insert a block into the given row, at the given index.
    /// Blocks cannot be inserted where there is not an existing block;
    /// in such a situation, use an "add" method instead.
    pub fn insert_block(
        &mut self,
        r: usize,
        i: usize,
        block: B
    ) -> anyhow::Result<&mut Self> { 
        
        let index = self.find_block_index(r, i)?;

        self.layout[r] += 1;
        self.blocks.insert(index, block);

        Ok(self)
    }

    /// Insert a collection of blocks at the given index, in the given row.
    /// Blocks cannot be inserted where there is not an existing block;
    /// in such a situation, use an "add" method instead.
    pub fn insert_blocks(
        &mut self, 
        r: usize, 
        i: usize, 
        mut blocks: Vec<B>
    ) -> anyhow::Result<&mut Self> {

        let index = self.find_block_index(r, i)?;

        let total = blocks.len();
        let mut tail = self.blocks.split_off(index);

        self.blocks.append(&mut blocks);
        self.blocks.append(&mut tail);
        self.layout[r] += total;

        Ok(self)
    }

}



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use crate::{ Block, Layer };
    use crate::block::test::TestBlock;
    use crate::types::layer::test::test_layer;

    /// Test adding block(s) to a layer.
    #[test] fn add_block_test() {

        let mut layer = test_layer();
        let new_block = |id: &str| TestBlock::create(&id.to_string());

        layer.add_block(new_block("new block"));
        assert_eq!(layer.layout[0], 1);
        assert_eq!(layer.layout[1], 3);
        assert_eq!(layer.get_block_ref(1, 2).unwrap().id, "new block");

        layer.new_row();
        layer.add_blocks(vec![]);
        assert_eq!(layer.layout[2], 0);

        layer.add_blocks(vec![
            new_block("new 0"),
            new_block("new 1"),
            new_block("new 2")
        ]);
        assert_eq!(layer.layout[1], 3);
        assert_eq!(layer.layout[2], 3);
        assert_eq!(layer.get_block_ref(2, 0).unwrap().id, "new 0");
        assert_eq!(layer.get_block_ref(2, 1).unwrap().id, "new 1");
        assert_eq!(layer.get_block_ref(2, 2).unwrap().id, "new 2");

    }

    /// Test inserting block(s) to a layer.
    #[test] fn insert_block_test() {

        let mut layer = test_layer();
        let new_block = |id: &str| TestBlock::create(&id.to_string());

        assert!(layer.insert_block(1, 2, new_block("bad index")).is_err());
        assert!(layer.insert_block(1, 1, new_block("new block")).is_ok());
        assert_eq!(layer.layout[0], 1);
        assert_eq!(layer.layout[1], 3);
        assert_eq!(layer.get_block_ref(1, 1).unwrap().id, "new block");

        assert!(layer.insert_block(1, 0, new_block("newer block")).is_ok());
        assert_eq!(layer.layout[1], 4);
        assert_eq!(layer.get_block_ref(1, 2).unwrap().id, "new block");
        assert_eq!(layer.get_block_ref(1, 0).unwrap().id, "newer block");

        assert!(
            layer.insert_blocks(0, 0, vec![
                new_block("new 0"),
                new_block("new 1"),
                new_block("new 2")
            ]).is_ok()
        );
        assert_eq!(layer.layout[0], 4);
        assert_eq!(layer.get_block_ref(0, 0).unwrap().id, "new 0");
        assert_eq!(layer.get_block_ref(0, 1).unwrap().id, "new 1");
        assert_eq!(layer.get_block_ref(0, 2).unwrap().id, "new 2");

    }

    /// Test adding block(s) to a row within a layer.
    #[test] fn add_block_to_row_test() {

        let mut layer = test_layer();
        let new_block = |id: &str| TestBlock::create(&id.to_string());

        assert!(
            layer.add_block_to_row(0, new_block("new block")).is_ok()
        );
        assert_eq!(layer.layout[0], 2);
        assert_eq!(layer.get_block_ref(0, 1).unwrap().id, "new block");

        assert!(
            layer.add_block_to_row(1, new_block("newer block")).is_ok()
        );
        assert_eq!(layer.layout[1], 3);
        assert_eq!(layer.get_block_ref(1, 2).unwrap().id, "newer block");

        assert!(
            layer.add_block_to_row(2, new_block("bad index")).is_err()
        );

        layer.new_row();
        assert!(
            layer.add_blocks_to_row(2, vec![
                new_block("new 0"),
                new_block("new 1"),
                new_block("new 2")
            ]).is_ok()
        );
        assert_eq!(layer.layout[2], 3);
        assert_eq!(layer.get_block_ref(2, 0).unwrap().id, "new 0");
        assert_eq!(layer.get_block_ref(2, 1).unwrap().id, "new 1");
        assert_eq!(layer.get_block_ref(2, 2).unwrap().id, "new 2");

    }

}
