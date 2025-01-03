
use super::Block;

pub trait Node: Block {
    
    /// Define the argument struct for connecting blocks.
    type ConnectionInstructions: Clone + Default;

    /// Connect a parent node to a new child.
    fn connect(
        &mut self, 
        other: &mut Self, 
        instructions: &Self::ConnectionInstructions
    ); 

    // TODO: disconnect

}



/*  UNIT TESTS  */
#[cfg(test)] pub(crate) mod test {

    use super::*;
    use crate::block::test::TestBlock;

    /// Test implementation of Node.
    impl Node for TestBlock {
        type ConnectionInstructions = u8;
        fn connect(
            &mut self, 
            other: &mut Self, 
            times: &Self::ConnectionInstructions
        ) {
            for _ in 0..*times {
                self.connections.push(other.id.clone()) 
            }
        } 
    }

    /// Test for connecting nodes.
    #[test] fn node_connect_test() {
        let mut a = TestBlock::create(&"a".to_string());
        let mut b = TestBlock::create(&"b".to_string());
        a.connect(&mut b, &1);
        assert_eq!(a.connections[0], "b".to_string());
    }

}
