
use crate::{ Block, Props, Layer };

pub trait Stack<P: Props, B: Block<P>, L: Layer<P, B>>: Clone {
    ///
    fn new() -> Self;

    ///
    fn layers(&self) -> &Vec<L>;
    ///
    fn layers_mut(&mut self) -> &mut Vec<L>;

    ///
    fn get_layer(&self, layer: usize) -> Option<&L> {
        self.layers().get(layer)
    }

    ///
    fn get_layer_mut(&mut self, layer: usize) -> Result<&mut L, anyhow::Error> {
        self.layers_mut().get_mut(layer).ok_or(anyhow::anyhow!("Invalid layer index"))
    }

    ///
    fn get_block<'stack>(
        &'stack self,
        layer: usize,
        row: usize,
        index: usize
    ) -> Result<&'stack B, anyhow::Error>
        where L: 'stack
    {
        let layer = self.get_layer(layer).ok_or(anyhow::anyhow!("Invalid layer index"))?;
        layer.get_block(row, index).ok_or(anyhow::anyhow!("Invalid block index"))
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
    fn stack_all(&mut self, layers: Vec<L>) {
        for layer in layers { self.stack(layer) }
    }

    ///
    fn insert(&mut self, index: usize, layer: L) {
        self.layers_mut().insert(index, layer);
    }

    fn clone_into_blocks(&mut self) -> Vec<Vec<Vec<B>>> {
        todo![]
    }

    ///
    // TODO OFFSET xyz

    ///
    fn realize_voids(&mut self) -> &mut Self {
        let mut max_x = 0usize;
        let mut max_y = 0usize;
        for layer in self.layers().iter() {
            let row_count = layer.layout().len();
            let max_index = layer.layout().iter().max();
            if row_count > max_x { max_x = row_count }
            if let Some(index) = max_index {
                if index > &max_y { max_y = *index }
            }
        }

        for layer in self.layers_mut().iter_mut() {
            layer.realize_volume(max_x, max_y);
        }
        self
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
    fn compress(&mut self) -> &mut Self {
        for layer in self.layers_mut() {
            layer.remove_voids()
        }
        self
    }

    /// Removes voids by dropping "unsupported" blocks down from higher layers.
    fn collapse(&mut self) -> &mut Self {
        self.realize_voids();
        //let mut layers = self.clone_into_blocks();
        //assert!{layers.len() > 0, "A stack needs layers to use `Stack::collapse`"};

        // for each row in each layer
        // for each non-void block, shift down to the lower layer
        // if any shifted mark true
        // repeat for each layer
        // if true, collapse again

        // need "vertical" vectors transform


        // for each non-void block, if the layer below has a void block or no block in that row/index,
        // check the index/row of the layer below that, and so on, until one is found,
        // move the block to the empty layer/row/index above it & continue
        // NOTE: Do not remove voids, let collapse and compress be functionally separate

        todo![]
    }

    ///
    fn split_x(&mut self, split: usize) -> Self {
        let mut new = Self::new();
        for layer in self.layers_mut().iter_mut() {
            new.layers_mut().push(layer.split_x(split))
        }
        new
    }

    ///
    fn split_y(&mut self, split: usize) -> Self {
        let mut new = Self::new();
        for layer in self.layers_mut().iter_mut() {
            new.layers_mut().push(layer.split_y(split))
        }
        new
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
        let mut these = self.layers().clone();
        these.iter_mut()
            .for_each(|s| s.flip_x() );
        *self.layers_mut() = these
    }

    ///
    fn flip_y(&mut self) {
        let mut these = self.layers().clone();
        these.iter_mut()
            .for_each(|s| s.flip_y() );
        *self.layers_mut() = these
    }

    /// Reverses the order of stack layers.
    fn flip_z(&mut self) {
        let flipped = self.layers().clone().into_iter().rev().collect();
        *self.layers_mut() = flipped
    }

    ///
    fn stitch_x(&mut self, other: &mut Self) {
        let mut these = self.layers().clone();
        let those = other.layers().clone();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_x(&mut o) );
        *self.layers_mut() = these
    }

    ///
    fn stitch_y(&mut self, other: &mut Self) {
        let mut these = self.layers().clone();
        let those = other.layers().clone();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_y(&mut o) );
        *self.layers_mut() = these
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
        let mut these = self.layers().clone();
        let those = other.layers().clone();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_x(&mut o) );
        *self.layers_mut() = these
    }

    ///
    fn riffle_y(&mut self, other: &mut Self) {
        let mut these = self.layers().clone();
        let those = other.layers().clone();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_y(&mut o) );
        *self.layers_mut() = these
    }

    ///
    fn riffle_z(&mut self, other: &mut Self) {
        let these = self.layers().clone();
        let those = other.layers().clone();
        let riffled: Vec<L> = these.into_iter()
            .zip(those.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();
        *self.layers_mut() = riffled
    }


    // TODO FUSIONS vs MERGE overlap

    fn align_center(&mut self) {
        // TODO for each layer, determine the center of mass
    }

}
