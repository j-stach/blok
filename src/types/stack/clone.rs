
use super::*;
use crate::{ Layout, Row };

//
// TODO
// These are sloppy and inefficient. 
// Don't clone multiple times, rewrite the algorithms for efficiency.
//


/// Methods for cloning blocks and setting stacks directly from blocks.
/// Useful when building or connecting asynchronously with blocks that are Sync + Send.
impl<B: Block> Stack<B> {

    /// Clone the stack into an array of layers.
    pub fn clone_into_layers(&self) -> Vec<Layer<B>> {
        let mut layers = Vec::new();
        let mut blocks = self.blocks().clone();

        for layout in self.layouts().iter() {
            let t = layout.total();
            let rest = blocks.split_off(t);
            let mut layer = Layer::default();
            layer.set_from_layout(layout.clone(), blocks)
                .expect("Layout corrupted"); // TODO Error
            blocks = rest;
            layers.push(layer)
        }

        layers
    }

    /// Overwrite the stack's values from an array of layers.
    pub fn set_from_layers(&mut self, layers: Vec<Layer<B>>) {
        self.layouts = layers.iter()
            .map(|layer| layer.layout().clone())
            .collect();

        self.blocks = layers.iter()
            .flat_map(|layer| layer.blocks().clone())
            .collect();
    }

    /// Clone the stack into a matrix of rows.
    pub fn clone_into_rows(&self) -> Vec<Vec<Row<B>>> {

        let blocks = self.clone_into_blocks();

        blocks.into_iter()
            .map(|layer|
                layer.into_iter()
                    .map(|row| Row::wrap(row))
                    .collect::<Vec<Row<B>>>()
            )
            .collect::<Vec<Vec<Row<B>>>>()
    }

    /// Overwrite the stack's values from a matrix of rows.
    pub fn set_from_rows(&mut self, rows: Vec<Vec<Row<B>>>) {

        let mut layouts = Vec::new();

        for layer in rows.iter() {
            let layout: Vec<usize> = layer.iter().map(|r| r.len()).collect();
            let layout = Layout::wrap(layout);
            layouts.push(layout);
        }

        let blocks: Vec<B> = rows.into_iter()
            .flat_map(|layer| 
                layer.into_iter()
                    .flat_map(|row| row.0)
                    .collect::<Vec<B>>()
            )
            .collect();

        self.layouts = layouts;
        self.blocks = blocks;

    }

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
            let mut layer = Layer::default();
            layer.set_from_blocks(bb.to_owned());
            layers.push(layer)
        }

        self.set_from_layers(layers);
    }

}
