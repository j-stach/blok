
/* Example to show basic usage */

use blok::{ Block, Stack, Layer, Layout, layout, Alignment, connect };

/// Simple implementation of a Block type.
#[derive(Clone)]
struct MyBlock {
    /// Data for block.
    data: String,
    /// Store "connections" as a collection of data from other blocks.
    connections: Vec<String>
} impl MyBlock {
    /// How u maek blok.
    fn new(data: &String) -> Self {
        MyBlock { data: data.to_owned(), connections: Vec::new() }
    }
}
impl Block for MyBlock {

    /// Encapsulate any arguments for the constructor into a single type.
    type ConstructionInstructions = String;
    /// Boilerplate, sorry.
    type Constructor = fn(&String) -> Self;
    /// Encapsulate any arguments for the connector into a single type.
    type ConnectionInstructions = ();

    /// Define the constructor for a non-data "void" block (placeholders & spacers).
    fn void() -> Self {
        MyBlock {
            data: String::new(),
            connections: Vec::new()
        }
    }

    /// Define the test to check for "void" blocks (placeholders & spacers).
    fn is_void(&self) -> bool {
        match self.data.as_str() {
            "" => true,
            _ => false
        }
    }

    /// Define the block-to-block connection procedure.
    fn connect(&mut self, other: &mut Self, _instr: &()) {
        self.connections.push(other.data.clone())
    }
}


/// Stack type represents a matrix of Blocks.
#[derive(Clone)]
struct MyStack {
    /// Stores the shape of the matrix as vectors of layer row lengths.
    layouts: Vec<Layout>,
    /// Stores the actual blocks together in memory.
    blocks: Vec<MyBlock>
}
impl Stack<MyBlock> for MyStack {
    /// Define the default constructor within the Stack implementation.
    fn new() -> Self {
        MyStack { layouts: vec![], blocks: vec![] }
    }

    // Boilerplate. "Derive" macro TBD.
    fn layouts(&self) -> &Vec<Layout> { &self.layouts }
    fn layouts_mut(&mut self) -> &mut Vec<Layout> { &mut self.layouts }
    fn blocks(&self) -> &Vec<MyBlock> { &self.blocks }
    fn blocks_mut(&mut self) -> &mut Vec<MyBlock> { &mut self.blocks }
}


fn main() {
    let mut pyramid = MyStack::new();
    let mut bottom = Layer::new();
    bottom.populate_with_clones(layout![3; 3], &MyBlock::new(&"bottom".to_string()));
    pyramid.stack(bottom);

    let mut middle = Layer::new();
    middle.populate_with_clones(layout![2; 2], &MyBlock::new(&"middle".to_string()));
    pyramid.stack(middle);

    let mut top = Layer::new();
    top.add_block(MyBlock::new(&"top".to_string()));
    pyramid.stack(top);

    connect::autoconnect_stack_uniformly(&mut pyramid, Alignment::dense, vec![(); 10]);

    // TODO Do something with pyramid.
}
