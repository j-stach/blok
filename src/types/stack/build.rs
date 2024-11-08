
use super::*;

/// Methods for building stacks:
impl<B: Block> Stack<B> {

    /// Add a block to the last row of the last layer.
    pub fn add_block(&mut self, block: B) {
        if self.layouts.is_empty() {
            self.new_layer()
        }

        let layout = self.layouts.last().unwrap();
        if layout.is_empty() {
            layout.push(0) // TODO function for this
        }

        layout.last().unwrap() += 1;

        self.blocks.push(block);
    }

    /// Add a collection of blocks to the last row of the last layer.
    pub fn add_blocks(&mut self, blocks: Vec<B>) {
        for block in blocks.into_iter() {
            self.add_block(block)
        }
    }

    /// Add a block to the last row of the given layer.
    pub fn add_block_to_layer(
        &mut self,
        l: usize,
        block: B
    ) -> anyhow::Error<()> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layout = &mut self.layouts[1];
        if layout.is_empty() {
            layout.push(0)
        }

        layout.last().unwrap() += 1;
        drop(layout);

        let layer_end = self.find_layer_end(l); // TODO Get index of last block in layer
        
        self.blocks.insert(layer_end, block);
        Ok(())
    }

    /// Add a block to the end of the given row in the given layer.
    pub fn add_block_to_row(
        &mut self,
        l: usize,
        r: usize,
        block: B 
    ) -> anyhow::Error<()> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layout = &mut self.layouts[1];

        if layout.len() < r {
            return Err(anyhow::anyhow!("Row does not exist")) 
        }

        // get row start?
        let row_end = self.find_row_end(l, r); // TODO get last block index by row

        // hmm
        Ok(())
    }

    /// Insert a block at the specific layer, row, index.
    pub fn insert_block(
        &mut self,
        l: usize,
        r: usize,
        i: usize,
        block: B 
    ) {

        // hmm
        todo![""]

    }

    /// Add a row to the last layer in the stack.
    pub fn add_row(
        &mut self,
        row: Row<B>
    ) {
        if self.layouts.len() == 0 {
            self.new_layer()
        }

        self.layouts.last().unwrap().push(row.len());
        self.blocks.append(*row);
    }

    /// Add a collection of rows to the last layer in the stack.
    pub fn add_rows(
        &mut self,
        rows: Vec<Row<B>>
    ) {
        for row in rows.into_iter() {
            self.add_row(row)
        }
    }

    /// Add a row to the end of the given layer.
    pub fn add_row_to_layer(
        &mut self,
        l: usize,
        row: Row<B>
    ) {

        // hmm
        todo![""]

    }

    /// Allocate a new layer on the stack.
    pub fn new_layer(&mut self) {
        self.layouts_mut().push(Layout::new())
    }

    /// Add a pre-existing layer to the top of the stack.
    pub fn add_layer(&mut self, mut layer: Layer<B>) {
        self.layouts_mut().push(layer.layout().clone());
        self.blocks_mut().append(&mut layer.blocks_mut());
    }

    /// Add an array of pre-existing layers to the top of the stack.
    pub fn add_layers(&mut self, layers: Vec<Layer<B>>) {
        for layer in layers { 
            self.add_layer(layer) 
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

    /// Create blocks using the given constructor,
    /// adding them in layers according to the given layouts.
    pub fn populate(
        &mut self,
        layouts: Vec<Layout>,
        instructions: &B::CreationInstructions
    ) {
        for layout in layouts.into_iter() {
            let total = layout.total();
            self.layouts.push(layout);
            self.blocks.append(vec![B::create(instructions); total]);
        }
    }

    /// Create blocks by cloning a prototype,
    /// adding them in layers according to the given layouts.
    pub fn populate_with_clones(
        &mut self,
        layouts: Vec<Layout>,
        block: &B
    ) {
        for layout in layouts.into_iter() {
            let total = layout.total();
            self.layouts.push(layout);
            self.blocks.append(vec![block.clone(); total]);
        }
    }

}
