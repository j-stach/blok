
use super::*;
use crate::{ Block, Layout };

/// Methods for rapidly populating stacks with similar blocks:
impl<B: Block> Stack<B> {
    /// Create blocks using the given constructor,
    /// adding them in layers according to the given layouts.
    pub fn populate(
        &mut self,
        layouts: Vec<Layout>,
        instructions: &B::CreationInstructions
    ) -> &mut Self {

        for layout in layouts.into_iter() {
            let total = layout.total();
            self.layouts.push(layout);
            self.blocks.append(&mut vec![B::create(instructions); total]);
        }
        self
    }

    /// Create blocks by cloning a prototype,
    /// adding them in layers according to the given layouts.
    pub fn populate_with_clones(
        &mut self,
        layouts: Vec<Layout>,
        block: &B
    ) -> &mut Self {

        for layout in layouts.into_iter() {
            let total = layout.total();
            self.layouts.push(layout);
            self.blocks.append(&mut vec![block.clone(); total]);
        }
        self
    }

}
