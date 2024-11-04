
use super::*;

impl<B: Block> Stack<B> {

    /// Clone the stack into an array of layers.
    pub fn clone_into_layers(&self) -> Vec<Layer<B>> {
        let mut layers = Vec::new();
        let mut blocks = self.blocks().clone();

        for layout in self.layouts().iter() {
            let t = layout.total();
            let rest = blocks.split_off(t);
            let mut layer = Layer::new();
            layer.set_from_layout(layout.clone(), blocks)
                .expect("Layout corrupted"); // TODO Error
            blocks = rest;
            layers.push(layer)
        }

        layers
    }

    /// Overwrite the stack's values from an array of layers.
    pub fn set_from_layers(&mut self, layers: Vec<Layer<B>>) {
        *self.layouts_mut() = layers.iter()
            .map(|layer| layer.layout().clone())
            .collect();

        *self.blocks_mut() = layers.iter()
            .flat_map(|layer| layer.blocks().clone())
            .collect();
    }

    // TODO:
    // clone_into_rows
    // set_from_rows

    /// Clone the stack into a matrix of blocks.
    pub fn clone_into_blocks(&self) -> Vec<Vec<Vec<B>>> {
        let layers = self.clone_into_layers();
        let mut blocks = Vec::new();

        for layer in layers.into_iter() {
            blocks.push(layer.clone_into_blocks())
        }

        blocks
    }

    /// Overwrite the stack's values from a matrix of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<Vec<Vec<B>>>) {
        let mut layers = Vec::new();

        for bb in blocks.iter() {
            let mut layer = Layer::new();
            layer.set_from_blocks(bb.to_owned());
            layers.push(layer)
        }

        self.set_from_layers(layers);
    }

}
