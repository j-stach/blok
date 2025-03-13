
/// Implement Block for a type in order to organize instances of that type 
/// into Layers and Stacks.
pub trait Block: Clone + Default {

    /// Define the argument struct for creating a block.
    type CreationInstructions: Clone + Default;

    /// Define the process for constructing a new block.
    fn create(instructions: &Self::CreationInstructions) -> Self;

    /// Create a block that represents empty space.
    fn void() -> Self;

    /// Check if a block represents empty space.
    fn is_void(&self) -> bool;



    /// Replace the block with the default void, overwriting all other values.
    fn to_void(&mut self) { 
        *self = Self::void() 
    }

    /// Overwrite the entire block with new data.
    fn overwrite(&mut self, other: Self) { 
        *self = other 
    }

    /// Swap the positions of two blocks.
    fn swap(&mut self, other: &mut Self) {
        let shelf = other.clone();
        other.overwrite(self.clone());
        self.overwrite(shelf);
    }

    /// Swap this block with another if the latter is a void.
    fn shift(&mut self, other: &mut Self) -> Option<()> {
        match other.is_void() {
            true => { Some(self.swap(other)) },
            false => None
        }
    }

    /// Create a new Block of a different type using the data from this block.
    fn map<C: Block, T: Fn(&Self) -> C>(&self, t: T) -> C {
        t(self)
    }

    // TBD Expand provided functions 

}


/*  UNIT TESTS  */
#[cfg(test)] pub(crate) mod test {

    use super::*;

    #[derive(
        Default, Clone,         // Necessary for Block impl
        Debug, Eq, PartialEq    // Nice to have for testing
    )]
    /// USE THIS STRUCT WHEN WRITING UNIT TESTS
    pub(crate) struct TestBlock {
        pub(crate) id: String,
        pub(crate) connections: Vec<String>    // This is for testing the impl for Node
    }

    /// Test implementation of Block.
    impl Block for TestBlock {
        type CreationInstructions = String;
        fn create(id: &Self::CreationInstructions) -> Self {
            TestBlock {
                id: id.to_owned(),
                connections: Vec::new()
            }
        }
        fn void() -> Self {
            TestBlock::default()
        }
        fn is_void(&self) -> bool {
            self.id.is_empty()
        }
    }

    /// Test of basic block creation.
    #[test] fn new_block_test() {
        let block = TestBlock::create(&"test".to_string());
        assert_eq!(&block.id, "test");
        assert!(!block.is_void());
    }
    
    /// Test for void block creation.
    #[test] fn void_block_test() {
        //
        let void = TestBlock::void();
        assert!(void.is_void());

        //
        let mut block = TestBlock::create(&"not_void".to_string());
        assert!(!block.is_void());

        //
        block.to_void();
        assert!(block.is_void());
        assert_eq!(block, void);
    }
    
    /// Test for swapping blocks.
    #[test] fn swap_block_test() {
        let mut block = TestBlock::create(&"block".to_string());
        let mut void = TestBlock::void();

        //
        block.swap(&mut void);
        assert!(block.is_void());
        assert!(!void.is_void());
    }
    
    /// Test for shifting blocks.
    #[test] fn shift_block_test() {
        //
        let mut block = TestBlock::create(&"block".to_string());
        let mut void = TestBlock::void();

        //
        block.shift(&mut void)
            .expect("Shift block into void position");
        assert!(block.is_void());
        assert!(!void.is_void());

        // 
        let mut other_block = TestBlock::create(&"other".to_string());
        assert!(void.shift(&mut other_block).is_none(), "Cannot shift into non-void block");

        //
        other_block.shift(&mut block)
            .expect("Shift block into void (block) position");

        //
        void.shift(&mut other_block)
            .expect("Shift block into void (other_block) position");

        //
        assert!(void.is_void());
        assert_eq!(&other_block.id, "block", "Other block is 'block'");
        assert_eq!(&block.id, "other", "Original block is 'other'");

    }
    
    /// Test for overwriting a block with new data.
    #[test] fn overwrite_block_test() {
        let mut block = TestBlock::create(&"block".to_string());
        block.overwrite(TestBlock::create(&"overwrite".to_string()));
        assert_eq!(&block.id, "overwrite", "Original block is overwritten");
    }
    
}



