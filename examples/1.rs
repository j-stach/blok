
/* Example to show basic usage and features */

use blok::{
    Layout, layout,
    build::{ Block, Layer, Stack },
};


/// Block is the element type for the matrix.
#[derive(Clone)]
struct MyBlock {
    // It can have other types too, but Blok will ignore them.
    /// MyBlock contains its Props directly.
    props: Option<MyProps>
} impl MyBlock {
    fn new() -> Self { Self { props: Some(MyProps) }}
}
impl Block<MyProps> for MyBlock {
    // Boilerplate. Derive soon.
    type Constructor = fn() -> MyBlock; // TODO BuildArgs type
    fn properties(&self) -> &Option<MyProps> { &self.props }
    fn properties_mut(&mut self) -> &mut Option<MyProps> { &mut self.props }
    fn void() -> MyBlock { MyBlock { props: None }}
}

/// Stack is an array of Layers, therefore a 3-D matrix of Blocks.
#[derive(Clone)]
struct MyStack {
    layers: Vec<MyLayer>
}
impl Stack<MyProps, MyBlock, MyLayer> for MyStack {
    fn new() -> Self { Self { layers: Vec::new() }}
    fn layers(&self) -> &Vec<MyLayer> { &self.layers }
    fn layers_mut(&mut self) -> &mut Vec<MyLayer> { &mut self.layers }
}

/// Let's build a Pyramid.
fn main() {
    // Populate a layer using a constructor.
    // TODO BuildArgs type will be added to this constructor!!
    let mut layer1 = MyLayer::new();
    let layout1 = layout![7; 7];
    let new_block = MyBlock::new;
    layer1.populate(layout1, new_block);

    // Populate a layer with identical Blocks.
    let mut layer2 = MyLayer::new();
    let proto_block = MyBlock::new();
    layer2.populate_with_clones(layout![5; 5], &proto_block);

    // We'll get a little funky with this one.
    let mut layer3 = MyLayer::new();
    let new = MyBlock::new;
    layer3.populate(layout![3], new_block);
    layer3.add_row(vec![new(); 3]);
    layer3.new_row();
    layer3.add_block(new());
    layer3.add_block(new());
    layer3.insert_block(1, 0, new()).unwrap();

    // Please be careful if you do this. The layout and blocks must always align.
    let layer4 = MyLayer { layout: layout![1], blocks: vec![MyBlock::new()] };
    // Actually, try to not do that at all.

    // Here are some basic ways to stack layers.
    let mut pyramid = MyStack::new();
    pyramid.stack_all(vec![layer1, layer3]);
    pyramid.insert(1, layer2);
    pyramid.stack(layer4);

    // Pyramid layers are misaligned, so let's center it using offsets.
    let mut count = 0usize;
    for layer in pyramid.layers_mut().iter_mut() {
        layer.offset_x(count);
        layer.offset_y(count);
        count += 1
    }

    // And that's a pyramid! Have fun trying to pretty-print it!
    // Next up in example 2, learn how to connect blocks using procedural generation.
}
