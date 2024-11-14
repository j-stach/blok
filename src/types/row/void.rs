
use super::*;
use crate::Block;

impl<B: Block> Row<B> {

    /// Add a number of voids to the start of the row.
    pub fn offset(&mut self, offset: usize) -> &mut Self {
        let voids = vec![B::void(); offset];
        self.insert_blocks(0, voids)
            .expect("Insert block at index 0");
        self
    }

    /// Add a number of voids to the end of the row.
    pub fn pad(&mut self, offset: usize) -> &mut Self {
        let voids = vec![B::void(); offset];
        self.add_blocks(voids);
        self
    }

    /// Remove all void blocks from the row, condensing it.
    pub fn compress(&mut self) -> &mut Self {
        self.retain(|block| !block.is_void());
        self
    }

    /// Replace voids with "real" blocks according to instructions.
    pub fn fill_voids(
        &mut self, 
        instructions: &B::CreationInstructions
    ) -> &mut Self {
        self.iter_mut()
            .filter(|block| block.is_void())
            .for_each(|void| *void = B::create(instructions));
        self
    }

    /// Replace voids by cloning an existing block.
    pub fn fill_with_clones(
        &mut self,
        block: &B
    ) -> &mut Self {
        self.iter_mut()
            .filter(|block| block.is_void())
            .for_each(|void| *void = block.clone());
        self
    }

}
