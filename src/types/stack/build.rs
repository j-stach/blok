
use super::*;
use crate::{ Block, Row };


/// Methods for building stacks:
impl<B: Block> Stack<B> {

    /// Add a block to the last row of the last layer.
    pub fn add_block(&mut self, block: B) {
        if self.layouts.is_empty() {
            self.new_layer()
        }

        let layout = self.layouts.last_mut().unwrap();
        // TODO function for this:
        if layout.is_empty() { layout.push(0) }
        *layout.last_mut().unwrap() += 1;

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
    ) -> anyhow::Result<()> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layout = &mut self.layouts[1];
        // TODO function for this:
        if layout.is_empty() { layout.push(0) }
        *layout.last_mut().unwrap() += 1;

        let layer_end = self.find_layer_end(l).unwrap(); 
        
        self.blocks.insert(layer_end, block);
        Ok(())
    }

    /// Add a block to the end of the given row in the given layer.
    pub fn add_block_to_row(
        &mut self,
        l: usize,
        r: usize,
        block: B 
    ) -> anyhow::Result<()> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let row_end = self.find_row_end(l, r);
        let row_layout = self.layouts[1].get_mut(r);

        if row_end.is_none() || row_layout.is_none() {
            return Err(anyhow::anyhow!("Row does not exist")) 
        }

        self.blocks.insert(row_end.unwrap() + 1, block);
        *row_layout.unwrap() += 1;

        Ok(())
    }

    /// Insert a block at the specific layer, row, index.
    pub fn insert_block(
        &mut self,
        l: usize,
        r: usize,
        i: usize,
        block: B 
    ) -> anyhow::Result<()> {

        let index = self.find_block_index(l, r, i);
        if index.is_none() {
            return Err(anyhow::anyhow!("Index does not exist"))
        }

        self.blocks.insert(index.unwrap(), block);
        self.layouts[l][r] += 1;
        Ok(())
    }

    /// Add a row to the last layer in the stack.
    pub fn add_row(
        &mut self,
        mut row: Row<B>
    ) {
        if self.layouts.len() == 0 {
            self.new_layer()
        }

        self.layouts.last_mut().unwrap().push(row.len());
        self.blocks.append(&mut row);
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
        mut row: Row<B>
    ) -> anyhow::Result<()> {

        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layer_end = self.find_layer_end(l).unwrap();

        self.layouts[l].push(row.len());
        let mut tail = self.blocks.split_off(layer_end);

        self.blocks.append(&mut row);
        self.blocks.append(&mut tail);

        Ok(())
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
        l: usize, 
        mut layer: Layer<B>
    ) -> anyhow::Result<()> {
        if self.layouts.len() < l { 
            return Err(anyhow::anyhow!("Layer does not exist")) 
        }

        let layer_end = self.find_layer_end(l).unwrap();

        self.layouts.push(layer.layout().clone());
        let mut tail = self.blocks.split_off(layer_end);

        self.blocks.append(layer.blocks_mut());
        self.blocks.append(&mut tail);

        Ok(())
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
            self.blocks.append(&mut vec![B::create(instructions); total]);
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
            self.blocks.append(&mut vec![block.clone(); total]);
        }
    }

}
