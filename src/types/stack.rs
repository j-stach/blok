
use crate::{ Block, Layer, Layout };

// TODO set_layer offset_xyz collapse fusion/merge_overlap realize_volume


/// Holds a grid layout of blocks in a single vector,
/// with row indexing stored separately.
pub trait Stack<B: Block>: Clone {
    /// Define a constructor for the empty stack.
    fn new() -> Self;

    /// Get a reference to the list of layouts for the stack layers.
    fn layouts(&self) -> &Vec<Layout>;
    /// Get a mutable reference to the list of layouts for the stack layers.
    fn layouts_mut(&mut self) -> &mut Vec<Layout>;
    /// Get a reference to the collection of blocks in the stack.
    fn blocks(&self) -> &Vec<B>;
    /// Get a mutable reference to the collection of blocks in the stack.
    fn blocks_mut(&mut self) -> &mut Vec<B>;

    /// Clone the stack into an array of layers.
    fn clone_into_layers(&self) -> Vec<Layer<B>> {
        let mut layers = Vec::new();
        let mut blocks = self.blocks().clone();
        for layout in self.layouts().iter() {
            let t = layout.total();
            let rest = blocks.split_off(t);
            let mut layer = Layer::new();
            layer.set_from_layout(layout.clone(), blocks);
            blocks = rest;
            layers.push(layer)
        }
        layers
    }

    /// Overwrite the stack's values from an array of layers.
    fn set_from_layers(&mut self, layers: Vec<Layer<B>>) {
        *self.layouts_mut() = layers.iter().map(|l| l.layout().clone()).collect();
        *self.blocks_mut() = layers.iter().flat_map(|l| l.blocks().clone()).collect();
    }

    /// Clone the stack into a matrix of blocks.
    fn clone_into_blocks(&self) -> Vec<Vec<Vec<B>>> {
        let layers = self.clone_into_layers();
        let mut blocks = Vec::new();
        for l in layers.into_iter() {
            blocks.push(l.clone_into_blocks())
        }
        blocks
    }

    /// Overwrite the stack's values from a matrix of blocks.
    fn set_from_blocks(&mut self, blocks: Vec<Vec<Vec<B>>>) {
        let mut layers = Vec::new();
        for bs in blocks.iter() {
            let mut layer = Layer::new();
            layer.set_from_blocks(bs.to_owned());
            layers.push(layer)
        }
        self.set_from_layers(layers);
    }

    /// Clone a layer from the stack and return it as a new entity.
    fn clone_layer(&self, l: usize) -> Option<Layer<B>> {
        let layouts = self.layouts();
        if l > layouts.len() { return None }
        let start = self.find_layer_start(l)?;
        let layout = &layouts[l];
        let end = start + layout.total();

        let mut layer = Layer::new();
        layer.set_from_layout(layout.clone(), self.blocks()[start..end].to_vec());
        Some(layer)
    }

    // TODO set_layer

    /// Find the block index for the start of the layer.
    fn find_layer_start(&self, l: usize) -> Option<usize> {
        let layouts = self.layouts();
        if l > layouts.len() { return None }
        let mut start = 0usize;
        for layout in &layouts[0..l] {
            start += layout.total()
        }
        Some(start)
    }

    /// Get a reference to the block at the given index.
    fn get_block<'stack>(
        &'stack self,
        layer: usize,
        row: usize,
        mut index: usize
    ) -> Option<&'stack B> {
        let layout = &self.layouts()[layer];
        let l_start = self.find_layer_start(layer)?;
        if row > layout.len() { return None }
        let r_start = {
            let mut sum = 0usize;
            layout[0..row].iter().for_each(|r| sum += r);
            sum
        };
        if index > layout[row] { return None }
        index += l_start + r_start;
        Some(&self.blocks()[index])
    }

    /// Add a new, empty layer to the stack.
    fn new_layer(&mut self) {
        self.layouts_mut().push(Layout::new())
    }

    /// Add a pre-existing layer to the top of the stack.
    fn stack(&mut self, mut layer: Layer<B>) {
        self.layouts_mut().push(layer.layout().clone());
        self.blocks_mut().append(&mut layer.blocks_mut());
    }

    /// Add an array of pre-existing layers to the top of the stack.
    fn stack_all(&mut self, layers: Vec<Layer<B>>) {
        for layer in layers { self.stack(layer) }
    }

    /// Add a pre-existing layer at a specific position in the stack.
    fn insert_layer(&mut self, index: usize, layer: Layer<B>) {
        let mut layers = self.clone_into_layers();
        layers.insert(index, layer);
        self.set_from_layers(layers)
    }

    // TODO OFFSET xyz

    /// Square off the matrix to the highest row length,
    /// by inserting void blocks into the empty indices.
    fn realize_voids(&mut self) -> &mut Self {
        let mut max_x = 0usize;
        let mut max_y = 0usize;
        let mut layers = self.clone_into_layers();
        for layer in layers.iter() {
            let row_count = layer.layout().len();
            let max_index = layer.layout().iter().max();
            if row_count > max_x { max_x = row_count }
            if let Some(index) = max_index {
                if index > &max_y { max_y = *index }
            }
        }

        for layer in layers.iter_mut() {
            layer.realize_volume(max_x, max_y);
        }
        self.set_from_layers(layers);
        self
    }

    // TODO realize_volume

    /// Replace all void blocks with ones generated by the given constructor.
    fn fill_voids(&mut self, c: &B::Constructor, i: &B::ConstructionInstructions) {
        let mut layers = self.clone_into_layers();
        for layer in layers.iter_mut() {
            layer.fill_voids(c, i)
        }
        self.set_from_layers(layers)
    }

    /// Replace all void blocks with ones cloned from a prototype.
    fn fill_with_clones(&mut self, block: &B) {
        let mut layers = self.clone_into_layers();
        for layer in layers.iter_mut() {
            layer.fill_with_clones(block)
        }
        self.set_from_layers(layers)
    }


    /// Removes voids by layer while preserving non-void block count and ordering.
    fn compress(&mut self) -> &mut Self {
        let mut layers = self.clone_into_layers();
        for layer in layers.iter_mut() {
            layer.remove_voids()
        }
        self.set_from_layers(layers);
        self
    }

    /// Removes voids by dropping "unsupported" blocks down from higher layers.
    fn collapse(&mut self) -> &mut Self {
        self.realize_voids();
        //let mut layers = self.clone_into_blocks();
        //assert!{layers.len() > 0, "A stack needs layers to use `Stack::collapse`"};
        // TODO

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

    /// Split each layer into two at the given row number. Leaves the original in place.
    fn split_x(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let mut new = Vec::new();
        for layer in old.iter_mut() {
            new.push(layer.split_x(split))
        }
        self.set_from_layers(old);
        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Split each layer into two by splitting each row at index given.
    fn split_y(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let mut new = Vec::new();
        for layer in old.iter_mut() {
            new.push(layer.split_y(split))
        }
        self.set_from_layers(old);
        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Split the stack in two at the given layer.
    fn split_z(&mut self, split: usize) -> Self {
        let mut old = self.clone_into_layers();
        let new = old.split_off(split);
        self.set_from_layers(old);
        let mut stack = Self::new();
        stack.set_from_layers(new);
        stack
    }

    /// Flip each layer across the Y axis, reversing the sequence of rows.
    fn flip_x(&mut self) {
        let mut flipped = self.clone_into_layers();
        flipped.iter_mut()
            .for_each(|s| s.flip_x() );
        self.set_from_layers(flipped)
    }

    /// Flip each layer across the X axis, reversing the order of blocks within the rows.
    fn flip_y(&mut self) {
        let mut flipped = self.clone_into_layers();
        flipped.iter_mut()
            .for_each(|s| s.flip_y() );
        self.set_from_layers(flipped)
    }

    /// Reverses the order of stack layers.
    fn flip_z(&mut self) {
        let flipped = self.clone_into_layers().into_iter().rev().collect();
        self.set_from_layers(flipped)
    }

    /// Stitch each layer to the corresponding layer in the other stack.
    fn stitch_x(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_x(&mut o) );
        self.set_from_layers(these)
    }

    /// Stitch rows from each layer to the corresponding layer in the other stack.
    fn stitch_y(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.stitch_y(&mut o) );
        self.set_from_layers(these)
    }

    /// Stack the entire other stack atop this stack's layers.
    fn stitch_z(&mut self, other: &mut Self) {
        self.layouts_mut().append(other.layouts_mut());
        self.blocks_mut().append(other.blocks_mut());
    }

    /// Stitch an x-flipped clone (after this stack's existing rows).
    fn mirror_x(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_x();
        self.stitch_x(&mut mirror)
    }

    /// Stitch an y-flipped clone (after this stack's existing rows).
    fn mirror_y(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_y();
        self.stitch_y(&mut mirror)
    }

    /// Stitch an z-flipped clone (atop this stack's existing rows).
    fn mirror_z(&mut self) {
        let mut mirror = self.clone();
        mirror.flip_z();
        self.stitch_z(&mut mirror)
    }

    /// TODO Documentation
    fn riffle_x(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_x(&mut o) );
        self.set_from_layers(these)
    }

    /// TODO Documentation
    fn riffle_y(&mut self, other: &mut Self) {
        let mut these = self.clone_into_layers();
        let those = other.clone_into_layers();
        these.iter_mut()
            .zip(those.into_iter())
            .for_each(|(s, mut o)| s.riffle_y(&mut o) );
        self.set_from_layers(these)
    }

    /// TODO Documentation
    fn riffle_z(&mut self, other: &mut Self) {
        let these = self.clone_into_layers();
        let those = other.clone_into_layers();
        let riffled: Vec<Layer<B>> = these.into_iter()
            .zip(those.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();
        self.set_from_layers(riffled)
    }

    // TODO FUSIONS vs MERGE overlap
}
