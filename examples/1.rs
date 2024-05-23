
/* Example to show basic usage and features */

use blok::{
    Layout, layout,
    build::{ Props, Block, Layer, Stack },
};

/// Props type holds the Block values that are relevant to the overall structure.
/// These don't have to be stored directly in the Block, but if they are created
/// independently, they should be able to set changes to the Block via reference.
#[derive(Clone)]
struct MyProps;
impl Props for MyProps {
    /// Combine the inner values however you like. MyProps has none in this case.
    fn merge(&mut self, _other: &mut Self) {}
    // TODO set props
}

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

/// Layer is a 2-D matrix of Blocks.
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
    // Boilerplate. Derive soon.
    fn layout(&self) -> &Layout { &self.layout }
    fn layout_mut(&mut self) -> &mut Layout { &mut self.layout }
    fn blocks(&self) -> &Vec<MyBlock> { &self.blocks }
    fn blocks_mut(&mut self) -> &mut Vec<MyBlock> { &mut self.blocks }
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

    // Pyramid layers are misaligned, so let's center it manually by using offsets.
    let mut count = 0usize;
    for layer in pyramid.layers_mut().iter_mut() {
        layer.offset_x(count);
        layer.offset_y(count);
        count += 1
    }

    /* TODO:
    // That's convenient for our pyramid, but complicated for less regular shapes.
    // There are a few simple auto-alignments for Stack that can get you started.
    // Here, we can stick with a central alignment.

    // First, we reset our pyramid by removing the voids we added to offset:
    // This calls `Layer::remove_voids()` for each Layer.
    pyramid.compress();

    // This determines the "center of mass" for each layer, in both X and Y dimensions,
    // then automatically adjusts each layer so that the COM for all layers are aligned.
    pyramid.align_center();
    */

    // And that's a pyramid! Have fun trying to pretty-print it!
    // Next up in example 2, learn how to connect blocks using procedural generation.
}
