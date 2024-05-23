
# blok (WIP)
Traits for 3D arrays with properties and attachments.

## How it works
Blok can be used to build arrays of "blocks" with special "properties".
It can also be used to define the relationships between different blocks,
by using "connections" to procedurally link their properties.

### Build:
1. Add `blok` to your project:
```
$ cargo add blok
```

2. Define a `Block` type and its associated `Props` type.
This is the element/particle for the matrix.
```
use blok::build::{ Props, Block };

#[derive(Clone)]
struct MyProps;
impl Props for MyProps {
    fn merge(&mut self, other: &mut Self) {
        // Combine the values however you like.
    }
}

#[derive(Clone)]
struct MyBlock {
    props: Option<MyProps>
}
// TODO: Boilerplate, we can derive this later:
impl Block<MyProps> for MyBlock {
    type Constructor = fn() -> MyBlock;
    fn properties(&self) -> &Option<MyProps> { &self.props }
    fn properties_mut(&mut self) -> &mut Option<MyProps> { &mut self.props }
    fn void() -> MyBlock { MyBlock { props: None }}
    fn is_void(&self) -> bool { self.properties().is_none() }
}
```

3. Define a `Layer` type for holding blocks in a 2-D array.
```
// Layers are organized with the layout stored separately from the collection of blocks.
// A `Layout` is a vector of row lengths that is used to index the blocks.
```
4. Define a `Stack` type that contains multiple layers (a 3-D array of blocks).
```
// TODO Copy example
```

### Connect:
1. TODO

## Contributing
Developing this to support another project I am working on, decided to split it off
since it could be used more generally and may come in handy elsewhere. <br>
Blok is still in its early phases, but feel free to hop on board.
Just looking to have some fun with it :) <br>
Current tasks can be tracked in [TODO.md]. <br>

Future directions include:
- GPU integration
- Volumetric-aware 3d modeling, property-based rendering
- TUI apps through `blok-tui`
- Games? through `blok-engine`
