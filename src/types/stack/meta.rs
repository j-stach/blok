
use super::Stack;
use crate::{ Block, Layout };

/// Meta-implementation for nested Stack types.
impl<B: Block> Block for Stack<B> {

    type CreationInstructions = Vec<Layout>;

    fn create(layouts: &Vec<Layout>) -> Self {
        let num_blocks = layouts.iter().map(|l| l.total()).sum();
        Self {
            layouts: layouts.to_owned(),
            blocks: vec![B::default(); num_blocks]
        }
    }

    fn void() -> Self { Self::default() }

    fn is_void(&self) -> bool { self.blocks().is_empty() }

}


