
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

    // TODO OFFSET

    // TODO realize_voids, fill_voids, clones
    fn realize_voids(&'b mut self) -> &'b mut Self {
        // get the max dims x and y for the layers,
        // add voids to fill in
        todo![]
    }
    // TODO FILL VOIDS, FILL CLONES
    // COLLAPSE (COMPRESS, WITH GRAVITY)
    // COMPRESS VOIDS

    // TODO STITCH FLIP SPLIT MIRROR
    // TODO RIFFLE x/y/z

    // TODO FUSIONS vs MERGE

}
