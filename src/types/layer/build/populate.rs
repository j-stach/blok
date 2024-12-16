
use crate::{ Block, Layer, Layout };

/// Functions for constructing layers:
impl<B: Block> Layer<B> {
    
    /// Create blocks using the given constructor,
    /// adding them in rows according to the given layout.
    pub fn populate(
        &mut self, 
        mut layout: Layout, 
        instructions: &B::CreationInstructions
    ) -> &mut Self {

        for row in layout.iter() {
            for _ in 0..*row {
                let block = B::create(instructions);
                self.blocks_mut().push(block)
            }
        }

        self.layout_mut().append(&mut layout);
        self
    }

    /// Create blocks by cloning a prototype,
    /// adding them in rows according to the given layout.
    pub fn populate_with_clones(
        &mut self, 
        mut layout: Layout, 
        block: &B
    ) -> &mut Self {

        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(block.clone());
            }
        }

        self.layout_mut().append(&mut layout);
        self
    }

}
