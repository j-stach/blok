
use blok::{ Block, Node };

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


fn build_cube() -> Stack<MyBlock> {
    let mut stack = Stack::new();
    stack
        .populate_with_clones(vec!{ layout![4; 4]; 4 }, &MyBlock::void())
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
    // TODO: Test/example for row, layer, and stack connection.
}


