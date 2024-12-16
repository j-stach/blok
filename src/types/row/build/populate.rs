
use crate::{ Block, Row };

impl<B: Block> Row<B> {

    /// Create a number of blocks using the given constructor,
    /// then add them to the end of the row.
    pub fn populate(
        &mut self,
        count: usize,
        instructions: &B::CreationInstructions
    ) -> &mut Self {

        self.append(&mut vec![B::create(instructions); count]);
        self
    }

    /// Create a number of blocks by cloning a prototype,
    /// then add them to the end of the row.
    pub fn populate_with_clones(
        &mut self,
        count: usize,
        block: &B
    ) -> &mut Self {

        self.append(&mut vec![block.clone(); count]);
        self
    }

}

