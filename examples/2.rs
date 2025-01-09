
use blok::*;

#[derive(
    Default, Clone,         // Necessary for Block impl
    Debug, Eq, PartialEq    // Nice to have for testing
)]
pub struct MyNode {
    pub id: String,
    pub connections: Vec<String>,
}

impl Block for MyNode {

    type CreationInstructions = String;

    fn create(id: &String) -> Self {
        MyNode {
            id: id.to_owned(),
            connections: Vec::new(),
        }
    }

    fn void() -> Self {
        Self::default()
    }

    fn is_void(&self) -> bool {
        self.id.is_empty()
    }

}


impl Node for MyNode {

    type ConnectionInstructions = u32;

    fn connect(
        &mut self, 
        other: &mut Self, 
        times: &u32
    ) {
        for _ in 0..*times {
            self.connections.push(other.id.clone())
        }
    }

}


// Borrowing this from example 1.
fn build_cube() -> Stack<MyNode> {
    let mut stack = Stack::new();
    stack
        .populate_with_clones(vec!{ layout![4; 4]; 4 }, &MyNode::void())
        .get_all_mut()
            .into_iter()
            .enumerate()
            .for_each(|(l, layer_mut)| { 
                layer_mut.into_iter()
                    .enumerate()
                    .for_each(|(r, row_mut)| {
                        row_mut.into_iter()
                            .enumerate()
                            .for_each(|(b, block_mut)| {
                                block_mut.id = format!("{}{}{}", l, r, b)
                            });
                    });
            });
    stack
}


pub fn main() {

    // Two identical cube stacks:
    let (mut cube1, mut cube2) = (build_cube(), build_cube());

    cube1.connect(&mut cube2,
        // Alignments for the layer, row, and block;
        // these are used to organize the pairings for procedural connection generation.
        Alignment::corresponding,
        Alignment::corresponding,
        Alignment::corresponding,
        // By breaking instructions into vectors, 
        // we are able to customize the instruction for each connection.
        // If you don't provide enough instructions to cover each connection,
        // the last instruction will be repeated for the remaining connections.
        // If you don't provide any connection instructions, it will use the default.
        // (This syntax will be made cleaner at some point.)
        vec![vec![vec![1]]], 
    );

    // TBD: In the future, iterating over types will be much easier.
    cube1.get_all_mut()
        .into_iter()
        .for_each(|layer_mut| {
            layer_mut.into_iter()
                .for_each(|row_mut| {
                    row_mut.into_iter()
                        .for_each(|block_mut| {
                            // Each block should have one connection,
                            assert_eq!(block_mut.connections.len(), 1);
                            // and it should be connected to a block that shares its ID,
                            // since it connects to a corresponding block in an idential stack.
                            assert_eq!(
                                block_mut.id, 
                                block_mut.connections[0]
                            );
                        });
                });
        });


    //let mut cube3 = build_cube();
    // TODO: connect rows and layers within stack 

    println!("That's all folks!");
}


