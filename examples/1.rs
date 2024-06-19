
/* Example to show basic usage and features */

use blok::{ Block, Stack, layout };

struct MyBlock {
    id: String,
    connections: Vec<String>
} impl MyBlock {
    fn new(id: String) -> Self {
        MyBlock { id, connections: Vec::new() }
    }
}
impl Block for MyBlock {
    fn void() -> Self {
        MyBlock {
            id: String::new(),
            connections: Vec::new()
        }
    }

    fn is_void(&self) -> bool {
        match &self.id {
            "" => true,
            _ => false
        }
    }

    type ConnectionInstructions = ();

    fn connect(&mut self, other: &mut Self, _instr: ()) {
        self.connections.push(other.id)
    }
}

struct MyStack {
    layouts: Vec<Layout>,
    blocks: Vec<MyBlock>
}
impl Stack<MyBlock> for MyStack {
    fn new() -> Self {
        MyStack { layouts: vec![], blocks: vec![] }
    }
    fn layouts(&self) -> &Vec<Layout> { &self.layouts }
    fn layouts_mut(&mut self) -> &mut Vec<Layout> { &mut self.layouts }
    fn blocks(&self) -> &Vec<Block> { &self.blocks }
    fn blocks_mut(&mut self) -> &mut Vec<Block> { &mut self.blocks }
}

fn main() {}
