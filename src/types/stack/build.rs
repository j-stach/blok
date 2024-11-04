
use super::*;

/// Methods for building stacks:
impl<B: Block> Stack<B> {

    /// Add a new, empty layer to the stack.
    pub fn new_layer(&mut self) {
        self.layouts_mut().push(Layout::new())
    }

    /// Add a pre-existing layer to the top of the stack.
    pub fn stack(&mut self, mut layer: Layer<B>) {
        self.layouts_mut().push(layer.layout().clone());
        self.blocks_mut().append(&mut layer.blocks_mut());
    }

    /// Add an array of pre-existing layers to the top of the stack.
    pub fn stack_all(&mut self, layers: Vec<Layer<B>>) {
        for layer in layers { 
            self.stack(layer) 
        }
    }

    /// Add a pre-existing layer at a specific position in the stack.
    pub fn insert_layer(
        &mut self, 
        index: usize, 
        layer: Layer<B>
    ) {
        let mut layers = self.clone_into_layers();
        layers.insert(index, layer);
        self.set_from_layers(layers)
    }

}
