
use super::*;
use crate::types::Row;


/// Functions for cloning the layer to and from different divisions.
impl<B: Block> Layer<B> {

    /// Clone the layer into an array of rows.
    pub fn clone_into_rows(&self) -> Vec<Row<B>> {
        // Cannot clone into blocks if Layout is corrupted.
        assert_eq!(self.layout().total(), self.blocks().len());
        
        let mut clone = self.blocks().clone();
        let mut rows = Vec::new();

        for r in self.layout().iter() {
            let (row, rest) = clone.split_at(*r);
            rows.push(Row::wrap(row.to_vec()));
            clone = rest.to_vec();
        }

        rows
    }

    /// Overwrite a layer's values from an array of rows.
    pub fn set_from_rows(&mut self, rows: Vec<Row<B>>) {
        self.layout = rows.iter()
            .map(|v| v.len())
            .collect();

        self.blocks = rows.into_iter()
            .flat_map(|r| r.blocks)
            .collect();
    }

    /// Clone the layer into a matrix of blocks.
    pub fn clone_into_blocks(&self) -> Vec<Vec<B>> {
        // Cannot clone into blocks if Layout is corrupted.
        assert_eq!(self.layout().total(), self.blocks().len());
        
        let mut clone = self.blocks().clone();
        let mut blocks = Vec::new();

        for r in self.layout().iter() {
            let (row, rest) = clone.split_at(*r);
            blocks.push(row.to_vec());
            clone = rest.to_vec();
        }

        blocks
    }

    /// Overwrite a layer's values from a matrix of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<Vec<B>>) {
        self.layout = blocks.iter()
            .map(|v| v.len())
            .collect();

        self.blocks = blocks.into_iter()
            .flatten()
            .collect();
    }

}


/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use crate::{ layout, Layout, Layer, Block };
    use crate::block::test::TestBlock;

    ///
    #[test] fn clone_layer_into_blocks_test() {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        let blocks = vec![TestBlock::create(&"test".to_string()); 3];
        layer.set_from_layout(layout, blocks)
            .expect("Set layer successful");
        
        let cloned_blocks = layer.clone_into_blocks();
        assert_eq!(cloned_blocks[0].len(), 1);
        assert_eq!(cloned_blocks[1].len(), 2);

        let mut other_layer = Layer::<TestBlock>::new();
        other_layer.set_from_blocks(cloned_blocks);

        assert_eq!(other_layer.layout[0], 1);
        assert_eq!(other_layer.layout[1], 2);
        assert_eq!(other_layer.blocks.len(), 3);
    }

    ///
    #[test] #[should_panic] fn bad_clone_layer_into_blocks_test() {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        let blocks = vec![TestBlock::create(&"test".to_string()); 3];
        layer.set_from_layout(layout, blocks)
            .expect("Set layer successful");

        // Should panic if the blocks and layout do not match:
        layer.blocks.pop();
        let _cloned_blocks = layer.clone_into_blocks();
    }

    ///
    #[test] fn clone_layer_into_rows_test() {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        let blocks = vec![TestBlock::create(&"test".to_string()); 3];
        layer.set_from_layout(layout, blocks)
            .expect("Set layer successful");
        
        let cloned_rows = layer.clone_into_rows();
        assert_eq!(cloned_rows[0].len(), 1);
        assert_eq!(cloned_rows[1].len(), 2);

        let mut other_layer = Layer::<TestBlock>::new();
        other_layer.set_from_rows(cloned_rows);

        assert_eq!(other_layer.layout[0], 1);
        assert_eq!(other_layer.layout[1], 2);
        assert_eq!(other_layer.blocks.len(), 3);
    }

    ///
    #[test] #[should_panic] fn bad_clone_layer_into_rows_test() {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        let blocks = vec![TestBlock::create(&"test".to_string()); 3];
        layer.set_from_layout(layout, blocks)
            .expect("Set layer successful");

        // Should panic if the blocks and layout do not match:
        layer.blocks.pop();
        let _cloned_blocks = layer.clone_into_rows();
    }

}

