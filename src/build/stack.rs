
use crate::{ Block, Props, Layer };

pub trait Stack<'b, P: Props, B: Block<'b, P>, L: Layer<'b, P, B> + 'b>: Clone {
    ///
    fn new() -> Self;

    ///
    fn layers(&self) -> &Vec<L>;
    ///
    fn layers_mut(&mut self) -> &mut Vec<L>;

    ///
    fn get_layer(&'b self, layer: usize) -> Option<&'b L> {
        self.layers().get(layer)
    }

    ///
    fn get_layer_mut(&mut self, layer: usize) -> Option<&mut L> {
        self.layers_mut().get_mut(layer)
    }

    ///
    fn get_block(&'b self, layer: usize, row: usize, index: usize) -> Option<&B> {
        self.get_layer(layer)?.get_block(row, index)
    }

    ///
    fn new_layer(&mut self) {
        self.layers_mut().push(L::new())
    }

    ///
    fn stack(&mut self, layer: L) {
        self.layers_mut().push(layer)
    }

    ///
    fn insert(&mut self, index: usize, layer: L) {
        self.layers_mut().insert(index, layer);
    }

    ///
    // TODO OFFSET xyz
    fn realize_voids(&'b mut self) -> &'b mut Self {
        // get the max dims x and y for the layers,
        // add voids to fill in
        todo![]
    }

    ///
    fn fill_voids(&mut self, constructor: &B::Constructor) {
        for layer in self.layers_mut() {
            layer.fill_voids(&constructor)
        }
    }

    ///
    fn fill_with_clones(&mut self, block: &B) {
        for layer in self.layers_mut() {
            layer.fill_with_clones(block)
        }
    }


    /// Removes voids by layer while preserving non-void block count and ordering.
    fn compress(&'b mut self) -> &'b mut Self {
        for layer in self.layers_mut() {
            layer.remove_voids()
        }
        self
    }

    /// Removes voids by dropping "unsupported" blocks down from higher layers.
    fn collapse(&'b mut self) -> &'b mut Self {
        // realize voids
        // for each layer, starting with the last, except for the first,
        // for each non-void block, if the layer below has a void block or no block in that row/index,
        // check the index/row of the layer below that, and so on, until one is found,
        // move the block to the empty layer/row/index above it & continue
        // NOTE: Do not remove voids, let collapse and compress be functionally separate

        todo![]
    }

    ///
    fn split_x(&mut self) -> Self {
        // new stack
        // for each layer, split x and push to new stack
        // return new stack
        todo![]
    }

    ///
    fn split_y(&mut self) -> Self {
        // new stack
        // for each layer, split y and push to new stack
        // return new stack
        todo![]

    }

    ///
    fn split_z(&mut self, split: usize) -> Self {
        let split = self.layers_mut().split_off(split);
        let mut new = Self::new();
        *new.layers_mut() = split;
        new
    }

    ///
    fn flip_x(&mut self) {
        // flip each layer in place
    }

    ///
    fn flip_y(&mut self) {
        // flip each layer in place
    }

    /// Reverses the order of stack layers.
    fn flip_z(&mut self) {
        let flipped = self.layers().clone().into_iter().rev().collect();
        *self.layers_mut() = flipped
    }

    ///
    fn stitch_x(&mut self, other: &mut Self) {
        // for each corresponding layer
        // zip & stitch_x
    }

    ///
    fn stitch_y(&mut self, other: &mut Self) {
        // for each corresponding layer
        // zip & stitch_x
    }

    /// Stack the entire other stack atop this stack's layers.
    fn stitch_z(&mut self, other: &mut Self) {
        self.layers_mut().append(other.layers_mut())
    }

    ///
    fn mirror_x(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_x();
        self.stitch_x(&mut mirror)
    }

    ///
    fn mirror_y(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_y();
        self.stitch_y(&mut mirror)
    }

    ///
    fn mirror_z(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_z();
        self.stitch_z(&mut mirror)
    }

    ///
    fn riffle_x(&mut self, other: &mut Self) {
        let these = self.layers().clone();
        let those = other.layers().clone();
        these.into_iter()
            .zip(those.into_iter())
            .for_each(|(mut s, mut o)| s.riffle_x(&mut o) )
    }

    ///
    fn riffle_y(&mut self, other: &mut Self) {
        let these = self.layers().clone();
        let those = other.layers().clone();
        these.into_iter()
            .zip(those.into_iter())
            .for_each(|(mut s, mut o)| s.riffle_y(&mut o) )
    }

    ///
    fn riffle_z(&mut self, other: &mut Self) {
        let these = self.layers().clone();
        let those = other.layers().clone();
        let riffled: Vec<L> = these.into_iter()
            .zip(those.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();
        *self.layers_mut() = riffled;
    }


    // TODO FUSIONS vs MERGE overlap

}
