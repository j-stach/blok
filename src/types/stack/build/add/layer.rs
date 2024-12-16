
use crate::{ Block, Layer, Stack, Layout };

//
// TODO: Clean up and helper functions 
// insert_layers
//

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
    pub fn insert_layer(
        &mut self, 
        l: usize, 
        mut layer: Layer<B>
    ) -> anyhow::Result<&mut Self> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layer_end = self.find_layer_end(l).unwrap();

        self.layouts.push(layer.layout().clone());
        let mut tail = self.blocks.split_off(layer_end);

        self.blocks.append(layer.blocks_mut());
        self.blocks.append(&mut tail);

        Ok(self)
    }

    // TODO insert_layers
}

