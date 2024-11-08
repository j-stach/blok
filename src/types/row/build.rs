
use super::*;

/// Represents a 1D row of blocks in a positional context,
/// to distinguish it from other vectors of blocks.
impl<B: Block> Row<B> {

    /// Add a block to the end of the row. 
    pub fn add_block(&mut self, block: B) {
        self.push(block)
    }

    /// Insert a block into teh row at the given index.
    pub fn insert_block(
        &mut self, 
        index: usize, 
        block: B
    ) {
        self.insert(index, block)
    }

    /// Create a number of blocks using the given constructor,
    /// then add them to the end of the row.
    pub fn populate(
        &mut self,
        count: usize,
        instructions: &B::CreationInstructions
    ) {
        self.append(vec![B::create(instructions); count])
    }

    /// Create a number of blocks by cloning a prototype,
    /// then add them to the end of the row.
    pub fn populate_with_clones(
        &mut self,
        count: usize,
        block: &B
    ) {
        self.append(vec![block.clone(); count])
    }

    // TODO remove_block

}
