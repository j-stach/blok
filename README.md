
# blok (WIP)
Traits for 3D arrays with attachments.

## How it works
Blok can be used to build arrays of "blocks".
It can also be used to define the relationships between different blocks,
by using "connections" to procedurally link their properties.

1. Add `blok` to your project:
```
$ cargo add blok
```
```
use blok::{ Block, Stack, Layout, layout, Alignment, connect::* };
```

2. Define a `Block` type. This is the element/particle for the matrix.
```
/// Simple implementation of a Block type.
#[derive(Clone)]
struct MyBlock {
    /// Data for block.
    data: String,
    /// Store "connections" as a collection of data from other blocks.
    connections: Vec<String>
}
impl Block for MyBlock {

    /// Encapsulate any arguments for the constructor into a single type.
    type ConstructionInstructions = String;
    /// Boilerplate, sorry.
    type Constructor = fn(Self::ConstructionInstructions) -> Self;
    /// Encapsulate any arguments for the connector into a single type.
    type ConnectionInstructions = ();

    /// How u maek blok.
    fn new(data: Self::ConstructionInstructions) -> Self {
        MyBlock { data, connections: Vec::new() }
    }

    /// Define the constructor for a non-data "void" block (placeholders & spacers).
    fn void() -> Self {
        MyBlock {
            data: String::new(),
            connections: Vec::new()
        }
    }

    /// Define the test to check for "void" blocks (placeholders & spacers).
    fn is_void(&self) -> bool {
        match &self.data {
            "" => true,
            _ => false
        }
    }

    /// Define the block-to-block connection procedure.
    fn connect(&mut self, other: &mut Self, _instr: ()) {
        self.connections.push(other.data)
    }
}

```

3. Define a `Stack` type (a 3-D array of Blocks).
```
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
    fn blocks(&self) -> &Vec<Block> { &self.blocks }
    fn blocks_mut(&mut self) -> &mut Vec<Block> { &mut self.blocks }
}
```

4. Build a Stack from Layers of Blocks.
```
fn main() {
    let mut pyramid = MyStack::new();
    let mut bottom = Layer::new();
    bottom.populate_with_clones(layout![3; 3], MyBlock::new("bottom"));
    pyramid.stack(bottom);

    let mut middle = Layer::new();
    middle.populate_with_clones(layout![2; 2], MyBlock::new("middle"));
    pyramid.stack(middle);

    let mut top = Layer::new();
    top.add_block(MyBlock::new("top"));
    pyramid.stack(top);

    ...

```

5. Connect the Stack to make its Blocks aware of one another.
```
TODO
```

## Future directions
I'm developing this crate to support another project I am working on, and decided
to split it off since it could be used more generally and may come in handy elsewhere.
<br>
Current tasks can be tracked in [TODO.md](/TODO.md). <br>
