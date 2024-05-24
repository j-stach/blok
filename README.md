
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

3. Define a `Layer` type for holding Blocks in a 2-D array.
```
#[derive(Clone)]
struct MyLayer {
    /// Layout is a vector of row lengths recording the shape of the matrix.
    layout: Layout,
    /// Blok assumes you want to keep all of a Layer's Blocks continuous in memory.
    blocks: Vec<MyBlock>
}
impl Layer<MyProps, MyBlock> for MyLayer {
    fn new() -> MyLayer {
        MyLayer {
            layout: Layout::new(),
            blocks: Vec::new()
        }
    }
    // TODO: Boilerplate. Derive soon.
    fn layout(&self) -> &Layout { &self.layout }
    fn layout_mut(&mut self) -> &mut Layout { &mut self.layout }
    fn blocks(&self) -> &Vec<MyBlock> { &self.blocks }
    fn blocks_mut(&mut self) -> &mut Vec<MyBlock> { &mut self.blocks }
}
```

4. Define a `Stack` type that contains multiple Layers (a 3-D array of Blocks).
```
#[derive(Clone)]
struct MyStack {
    layers: Vec<MyLayer>
}
impl Stack<MyProps, MyBlock, MyLayer> for MyStack {
    fn new() -> Self { Self { layers: Vec::new() }}
    fn layers(&self) -> &Vec<MyLayer> { &self.layers }
    fn layers_mut(&mut self) -> &mut Vec<MyLayer> { &mut self.layers }
}

```

5. Build a Stack from Layers of Blocks.
```
/// Let's build a Pyramid.
fn main() {
    // Populate a layer using a constructor.
    // TODO BuildArgs type will be added to this constructor!!
    let mut layer1 = MyLayer::new();
    layer1.populate(layout![7; 7], MyBlock::new);

    // Populate a layer with cloned Blocks.
    let mut layer2 = MyLayer::new();
    layer2.populate_with_clones(layout![5; 5], &MyBlock::new());

    // Populate can be used to extend a layer as well.
    // See the examples directory for other ways to build layers.
    let mut layer3 = MyLayer::new();
    layer3.populate(layout![3, 3], MyBlock::new);
    layer3.populate(layout![3], MyBlock::new);

    // And to cap it off:
    let mut layer4 = MyLayer::new();
    layer4.add_block(MyBlock::new());

    // Here are some basic ways to stack layers.
    let mut pyramid = MyStack::new();
    pyramid.stack_all(vec![layer1, layer3]);
    pyramid.insert(1, layer2);
    pyramid.stack(layer4);

    // Pyramid layers are misaligned, so let's center it using voids as offsets.
    let mut count = 0usize;
    for layer in pyramid.layers_mut().iter_mut() {
        layer.offset_x(count);
        layer.offset_y(count);
        count += 1
    }

    // And that's a pyramid!
    // You can use realize_voids to fill the layers with "void" (property-less) blocks
    // until the layers are of equal dimesions.
    // TODO: Pretty-print it!

    // Next up, learn how to connect blocks using procedural generation.
}
```

### Connect:
1. TODO

## Future directions
That's just a taste. More to come! <br>
I'm developing this to support another project I am working on, and decided
to split it off since it could be used more generally and may come in handy elsewhere.
<br>
Current tasks can be tracked in [TODO.md](/TODO.md). <br>

Future directions may include:
- GPU integration
- Volumetric-aware 3d modeling, property-based rendering
- TUI apps through `blok-tui`, and why not GUI?
- Games? as `blok-engine`
