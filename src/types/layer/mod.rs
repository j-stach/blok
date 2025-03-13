
pub mod build;
pub mod partial;
pub mod clone;

use crate::{ Block, Layout };


/// Holds a grid of blocks in a single vector, 
/// with the layout tracked separately.
#[derive(Debug, Default, Clone)]
pub struct Layer<B: Block> {
    /// Represents the organization of blocks in the array.
    pub(crate) layout: Layout,
    /// Blok assumes you want to store the block array contiguously in memory.
    pub(crate) blocks: Vec<B>
}

/// Field access methods:
impl<B: Block> Layer<B> {

    /// Create an empty layer.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the blocks of the layer.
    pub fn blocks(&self) -> &Vec<B> { 
        &self.blocks 
    }

    /// Get a mutable reference to the blocks of the layer.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> { 
        &mut self.blocks 
    }

    /// Get a reference to the layout of the layer.
    pub fn layout(&self) -> &Layout { 
        &self.layout 
    }

    /// Get a mutable reference to the layout of the layer.
    pub(crate) fn layout_mut(&mut self) -> &mut Layout { 
        &mut self.layout 
    }

    /// Set layer data arbitrarily.
    pub fn set_from_layout(
        &mut self, 
        layout: Layout, 
        blocks: Vec<B>
    ) -> anyhow::Result<()> { // TODO Error
        
        if layout.total() != blocks.len() { 
            return Err(anyhow::anyhow!("Size mismatch"))
            // TODO: Error
        }

        *self.layout_mut() = layout;
        *self.blocks_mut() = blocks;

        Ok(())
    }
    
    /// Create a new Layer of a different type of blocks using the rows from this layer.
    pub fn map<C: Block, T: Fn(&B) -> C>(&self, t: T) -> Layer<C> {
        let mapped_blocks: Vec<C> = self.blocks()
            .iter()
            .map(t)
            .collect();

        let mut mapped_layer = Layer::<C>::new();
        mapped_layer.layout = self.layout.clone();
        mapped_layer.blocks = mapped_blocks;

        mapped_layer
    }

}



/*  UNIT TESTS  */
#[cfg(test)] pub(crate) mod test {

    use super::*;
    use crate::{ layout, Layout };
    use crate::block::{ Block, test::TestBlock };
    
    /// Test for setting layer from given values.
    pub(crate) fn test_layer() -> Layer<TestBlock> {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        let mut blocks = vec![TestBlock::create(&"test".to_string()); 3];

        // Name the blocks using their index in order to differentiate them.
        for (b, block) in blocks.iter_mut().enumerate() {
            block.id = b.to_string()
        }

        layer.set_from_layout(layout, blocks)
            .expect("Set layer successful");

        layer
    }

    /// Test for creating new layer.
    #[test] fn new_layer_test() {
        let layer = Layer::<TestBlock>::new();
        let layout = layer.layout();
        let blocks = layer.blocks();
        assert!(layout.is_empty() && blocks.is_empty(), "Default layer is empty");
    }
    
    /// Test for setting layer from given values.
    #[test] fn set_layer_test() {
        test_layer();
    }
    
    /// Test for setting layer from given values.
    #[test] #[should_panic(expected = "Set layer fails")] fn bad_set_layer_test() {
        let mut layer = Layer::<TestBlock>::new();
        let layout = layout![1, 2];
        // The number of blocks is fewer than the layout total:
        let blocks = vec![TestBlock::create(&"test".to_string()); 2];

        layer.set_from_layout(layout, blocks)
            .expect("Set layer fails");
    }
    
}


