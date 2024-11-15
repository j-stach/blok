
use blok::{ Block, Row, Layout, layout, Layer };
use crate::block::TestBlock1;

// new layer
#[test] fn new_layer_test() {
    let mut layer = Layer::<TestBlock1>::default();
    assert!(layer.layout().is_empty());
    assert!(layer.blocks().is_empty());

    let new_block = |id: &str| TestBlock1::create(&id.to_string());
    let mut blocks = Vec::new();
    for n in 0..4 {
        blocks.push(new_block(&format!("{}", n)))
    }

    // Should be able to set blocks from a layout if the totals are equal.
    let layout1 = layout![4];
    let layout2 = layout![1, 1, 1, 1];

    layer.set_from_layout(layout1, blocks.clone())
        .expect("Should accept the formatting of these blocks");
    assert_eq!(layer.layout().len(), 1);
    assert_eq!(layer.layout()[0], 4);
    assert_eq!(layer.blocks().len(), 4);

    layer.set_from_layout(layout2, blocks.clone())
        .expect("Should accept the formatting of these blocks");
    assert_eq!(layer.layout().len(), 4);
    assert_eq!(layer.layout()[0], 1);
    assert_eq!(layer.blocks().len(), 4);

    // Bad layouts should be rejected.
    let bad_layout1 = layout![5];
    let bad_layout2 = layout![1, 1, 1, 1, 1];

    assert!(layer.set_from_layout(bad_layout1, blocks.clone()).is_err());
    assert!(layer.set_from_layout(bad_layout2, blocks.clone()).is_err());

    // Values should be unchanged after returning error.
    assert_eq!(layer.layout().len(), 4);
    assert_eq!(layer.layout()[0], 1);
    assert_eq!(layer.blocks().len(), 4);
}

// build
// populate
// clone
// voids
// partial
