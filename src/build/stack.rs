
use crate::{ Block, Layer };

pub trait Stack<'b, B: Block<'b>, L: Layer<'b, B> + 'b> {
    fn layers(&self) -> &Vec<L>;
    fn layers_mut(&mut self) -> &mut Vec<L>;

    fn get_layer(&'b self, layer: usize) -> Option<&'b L> {
        self.layers().get(layer)
    }
    fn get_layer_mut(&mut self, layer: usize) -> Option<&mut L> {
        self.layers_mut().get_mut(layer)
    }

    fn get_block(&'b self, layer: usize, row: usize, index: usize) -> Option<&B> {
        self.get_layer(layer)?.get_block(row, index)
    }

    fn new_layer(&mut self) {
        self.layers_mut().push(L::new())
    }

    fn stack(&mut self, layer: L) {
        self.layers_mut().push(layer)
    }

    fn insert(&mut self, index: usize, layer: L) {
        self.layers_mut().insert(index, layer);
    }

    // TODO OFFSET xyz

    // TODO realize_voids, fill_voids, clones
    fn realize_voids(&'b mut self) -> &'b mut Self {
        // get the max dims x and y for the layers,
        // add voids to fill in
        todo![]
    }

    fn fill_voids(&mut self, constructor: &B::Constructor) {
        for layer in self.layers_mut() {
            layer.fill_voids(&constructor)
        }
    }

    fn fill_with_clones(&mut self, block: &B) {
        for layer in self.layers_mut() {
            layer.fill_with_clones(block)
        }
    }


    // COMPRESS VOIDS
    fn compress(&'b mut self) -> &'b mut Self {
        for layer in self.layers_mut() {
            layer.remove_voids()
        }
        self
    }

    // COLLAPSE (COMPRESS, WITH GRAVITY)
    fn collapse(&'b mut self) -> &'b mut Self {
        // realize voids
        // for each layer, starting with the last, except for the first,
        // for each non-void block, if the layer below has a void block or no block in that row/index,
        // check the index/row of the layer below that, and so on, until one is found,
        // move the block to the empty layer/row/index above it & continue
        // NOTE: Do not remove voids, let collapse and compress be functionally separate

        todo![]
    }

    // TODO STITCH FLIP SPLIT MIRROR
    // TODO RIFFLE x/y/z

    // TODO FUSIONS vs MERGE

}
