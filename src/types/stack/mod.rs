
pub mod build;
pub mod partial;
pub mod clone;
pub mod meta;


use crate::{ Block, Layer, Layout };


/// Holds a 3D matrix of blocks in a single vector,
/// with layer/row indexing stored in a vector of layouts.
#[derive(Debug, Default, Clone)]
pub struct Stack<B: Block> {
    /// Represents the organization of blocks in the array.
    layouts: Vec<Layout>,
    /// Blok assumes you want to store the block array contiguously in memory.
    blocks: Vec<B>
}

/// Field access methods:
impl<B: Block> Stack<B> {

    /// Create an empty stack.
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to the list of layouts for the stack layers.
    pub fn layouts(&self) -> &Vec<Layout> {
        &self.layouts
    }

    /// Get a mutable reference to the list of layouts for the stack layers.
    pub(crate) fn layouts_mut(&mut self) -> &mut Vec<Layout> {
        &mut self.layouts
    }
    
    /// Get a reference to the blocks in the stack.
    pub fn blocks(&self) -> &Vec<B> {
        &self.blocks
    }
    
    /// Get a mutable reference to the blocks in the stack.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> {
        &mut self.blocks
    }

}

/// Utility functions:
impl<B: Block> Stack<B> {

    /// Create a new Layer of a different type of blocks using the rows from this layer.
    pub fn map<C: Block, T: Fn(&B) -> C>(&self, t: T) -> Stack<C> {
        let mapped_blocks: Vec<C> = self.blocks()
            .iter()
            .map(t)
            .collect();

        let mut mapped_stack = Stack::<C>::new();
        mapped_stack.blocks = mapped_blocks;
        mapped_stack.layouts = self.layouts.clone();

        mapped_stack
    }
}


#[cfg(test)] mod test {
    use crate::Stack;
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::{ Layer, test::test_layer };
    
    pub(crate) fn test_stack() -> Stack<TestBlock> {
        let mut stack = Stack::<TestBlock>::new();
        stack.add_layers(vec![test_layer(); 3]);
        stack
    }

    #[test] fn new_stack_test() {
        let stack = test_stack();
        assert_eq!(stack.layouts.len(), 3);
        assert_eq!(stack.blocks.len(), 9);
    }
}


