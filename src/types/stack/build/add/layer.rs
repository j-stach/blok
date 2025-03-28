
use super::*;
use crate::{ Block, Layer, Stack, Layout };

/// Methods for adding layers:
impl<B: Block> Stack<B> {

    /// Allocate a new layer on the stack.
    pub fn new_layer(&mut self) -> &mut Self {
        self.layouts_mut().push(Layout::new());
        self
    }

    /// Add a pre-existing layer to the top of the stack.
    pub fn add_layer(&mut self, mut layer: Layer<B>) -> &mut Self {
        self.layouts_mut().push(layer.layout().clone());
        self.blocks_mut().append(&mut layer.blocks_mut());
        self
    }

    /// Add an array of pre-existing layers to the top of the stack.
    pub fn add_layers(&mut self, layers: Vec<Layer<B>>) -> &mut Self {
        for layer in layers { 
            self.add_layer(layer); 
        }
        self
    }

    /// Add a pre-existing layer at a specific position in the stack.
    /// Blocks cannot be inserted where there is not an existing block\*;
    /// in such a situation, use an "add" method instead.
    ///
    /// \*i.e. in the last position
    pub fn insert_layer(
        &mut self, 
        l: usize, 
        mut layer: Layer<B>
    ) -> anyhow::Result<&mut Self> {

        // Check to make sure the index exists before attempting to find previous layer.
        // TBD: Insert at end is a work in progress.
        if self.layouts.len() <= l {
            return Err(anyhow::anyhow!("Stack could not be indexed at layer {l}"))
        }

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the previous layer.
        let index = previous_available_layer_recursion_helper(self, l) + 1;

        // Insert layout into record.
        self.layouts.insert(l, layer.layout().clone());

        // Insert blocks.
        let mut tail = self.blocks.split_off(index);
        self.blocks.append(layer.blocks_mut());
        self.blocks.append(&mut tail);

        Ok(self)
    }

    /// Add multiple layers at a specific position in the stack.
    /// Blocks cannot be inserted where there is not an existing block\*;
    /// in such a situation, use an "add" method instead.
    ///
    /// \*i.e. in the last position
    pub fn insert_layers(
        &mut self, 
        l: usize, 
        layers: Vec<Layer<B>>
    ) -> anyhow::Result<&mut Self> {

        // Check to make sure the index exists before attempting to find previous layer.
        // TBD: Insert at end is a work in progress.
        if self.layouts.len() <= l {
            return Err(anyhow::anyhow!("Stack could not be indexed at layer {l}"))
        }

        // Use the helper function to find an index that can be used for reference.
        // Gets the last block of the previous layer.
        let index = previous_available_layer_recursion_helper(self, l) + 1;

        // Extract/expand layers.
        let (mut layouts, blocks): (Vec<Layout>, Vec<Vec<B>>) = layers.into_iter()
            .map(|layer| (layer.layout, layer.blocks))
            .unzip();

        let mut blocks = blocks.into_iter().flatten().collect();

        // Insert layouts into record.
        let mut tail = self.layouts.split_off(l);
        self.layouts.append(&mut layouts);
        self.layouts.append(&mut tail);

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

    ///
    #[test] fn new_layer_test() {
        let mut stack = test_stack();
        stack.new_layer();
        assert_eq!(stack.layouts[3].len(), 0);
    }

    ///
    #[test] fn add_layer_test() {

        let mut stack = test_stack();

        stack.add_layer(test_layer());
        assert_eq!(stack.layouts[3].len(), 2);
        assert_eq!(stack.blocks.len(), 12);

        stack.add_layers(vec![test_layer(); 2]);
        assert_eq!(stack.layouts().len(), 6);
        assert_eq!(stack.blocks.len(), 18);

    }

    ///
    #[test] fn insert_layer_test() {

        let mut stack = test_stack();
        assert!(stack.insert_layer(3, test_layer()).is_err());
        assert!(stack.insert_layer(2, test_layer()).is_ok());
        assert!(stack.insert_layer(1, test_layer()).is_ok());
        assert!(stack.insert_layer(0, test_layer()).is_ok());
        assert_eq!(stack.layouts().len(), 6);

        assert!(stack.insert_layers(0, vec![test_layer(); 3]).is_ok());
        assert_eq!(stack.layouts().len(), 9);

    }
    
}


