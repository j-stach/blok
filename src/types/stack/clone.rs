
use super::*;
use crate::{ Layout, Row };

/// Methods for cloning blocks and setting stacks directly from blocks.
/// Useful when building or connecting asynchronously with blocks that are Sync + Send.
/// Partial clones are also found here.
impl<B: Block> Stack<B> {

    /// Clone the stack into an array of layers.
    /// Returns an empty Vec when the stack has no blocks.
    pub fn clone_into_layers(&self) -> Vec<Layer<B>> {

        let mut layers = Vec::new();
        let num_layers = self.layouts.len();

        // Clone each layer and push to the collection Vec.
        for l in 0..num_layers {
            // No need for checks when using the layouts for guidance.
            let layer_clone = self.clone_layer(l).expect("Layer exists");
            layers.push(layer_clone);
        }

        layers
    }

    /// Overwrite the stack's values with an array of layers.
    pub fn set_from_layers(&mut self, layers: Vec<Layer<B>>) -> &mut Self {

        // Set layouts directly from layer data, 
        self.layouts = layers.iter()
            .map(|layer| layer.layout().clone())
            .collect();

        // then flatten blocks into one vec.
        self.blocks = layers.iter()
            .flat_map(|layer| layer.blocks().clone())
            .collect();

        // Return the newly-configured stack.
        self
    }

    /// Clone the stack into a matrix of rows.
    /// Returns an empty Vec when the stack has no blocks.
    pub fn clone_into_rows(&self) -> Vec<Vec<Row<B>>> {

        let mut layers = Vec::new();
        let layouts = self.layouts();

        // Each layout represents the shape of a layer.
        for (l, layout) in layouts.iter().enumerate() {
            // Represent the layer as a Vec of rows.
            let mut rows = Vec::new();
            let num_rows = layout.len();

            // Clone each row in the layer. 
            for r in 0..num_rows {
                // No need for checks when using the layouts for guidance.
                let cloned_row = self.clone_row(l, r)
                    .expect("Row exists");
                rows.push(cloned_row);
            }

            layers.push(rows);
        }

        layers
    }

    /// Overwrite the stack's values with a matrix of rows.
    pub fn set_from_rows(&mut self, rows: Vec<Vec<Row<B>>>) -> &mut Self {

        // Set layouts based on matrix, 
        let mut layouts = Vec::new();
        rows.iter()
            .for_each(|l| {
                let mut layout = Layout::new();
                l.iter().for_each(|r| layout.push(r.len()));
                layouts.push(layout)
            });
        
        // then flatten blocks into one vec.
        let blocks: Vec<B> = rows.into_iter()
            .flat_map(|l| {
                l.into_iter()
                    .flat_map(|r| {
                        r.blocks
                    })
                    .collect::<Vec<B>>()
            })
            .collect();

        // Set directly and return the newly-configured stack.
        self.layouts = layouts;
        self.blocks = blocks;
        self
    }

    /// Clone the stack into a matrix of blocks.
    /// Returns an empty Vec when the stack has no blocks.
    pub fn clone_into_blocks(&self) -> Vec<Vec<Vec<B>>> {

        // Mirrors the structure of the partial reference, so we can just map it.
        self.get_all_ref()
            .into_iter()
            .map(|l| {
                l.into_iter()
                    .map(|r| {
                        r.into_iter()
                            .map(|b| b.clone())
                            .collect()
                    })
                    .collect()
            })
            .collect()
    }

    /// Overwrite the stack's values with a matrix of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<Vec<Vec<B>>>) -> &mut Self {

        // Set layouts based on matrix, 
        let mut layouts = Vec::new();
        blocks.iter()
            .for_each(|l| {
                let mut layout = Layout::new();
                l.iter().for_each(|r| layout.push(r.len()));
                layouts.push(layout)
            });
        
        // then flatten blocks into one vec.
        let blocks: Vec<B> = blocks.into_iter()
            .flat_map(|l| {
                l.into_iter()
                    .flatten()
            })
            .collect();

        // Set directly and return the newly-configured stack.
        self.layouts = layouts;
        self.blocks = blocks;
        self
    }

    /// Clone a layer from the stack and return it as a new structure.
    /// Returns None if the layer does not exist in the stack, or is empty.
    pub fn clone_layer(
        &self, 
        l: usize
    ) -> Option<Layer<B>> {

        // Use partial reference to perform checks & validations,
        let blocks: Vec<B> = self.get_layer_ref(l)?
            .into_iter()
            // then flatten reference vectors into a Vec of clones.
            .flat_map(|r| {
                r.into_iter()
                    .map(|b| b.clone())
                    .collect::<Vec<B>>()
            })
            .collect();

        // No need to repeat checks.
        let layout = self.layouts.get(l)
            .expect("Layout exists if layer ref exists")
            .clone();

        // Set a new layer using the cloned values.
        let mut layer = Layer::new();
        layer.set_from_layout(layout, blocks)
            .expect("Layout is not corrupted");

        Some(layer)
    }

    /// Clone a row from the stack and return it as a new structure.
    /// Returns None if the row does not exist in the stack, or is empty.
    pub fn clone_row(
        &self, 
        l: usize,
        r: usize
    ) -> Option<Row<B>> {

        // Use partial reference to perform checks & validations.
        let blocks: Vec<B> = self.get_row_ref(l, r)?
            .into_iter()
            .map(|b| b.clone())
            .collect();

        // Set a new row using the cloned values.
        let row = Row::wrap(blocks);
        Some(row)
    }

    /// Clone a block from the stack and return it as a new structure.
    /// Returns None if the block does not exist in the stack.
    pub fn clone_block(
        &self, 
        l: usize,
        r: usize,
        i: usize
    ) -> Option<B> {

        let block = self.get_block_ref(l, r, i)?;
        Some(block.clone())
    }

}



#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    use crate::types::stack::{ Stack, test::test_stack };
    
    /*
    #[test] fn clone_stack_into_blocks_test() {}
    
    #[test] fn clone_stack_into_rows_test() {}
    
    #[test] fn clone_stack_into_layers_test() {}

    #[test] fn partial_clone_test() {}
    */
}

