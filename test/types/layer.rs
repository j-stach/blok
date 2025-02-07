
use blok::{ Block, Layout, layout, Layer };
use crate::block::TestBlock1;

//
// TODO
// Documentation comments
// Assert statements needed for some tests
// 

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
    let bad_layout3 = layout![3];
    let bad_layout4 = layout![1, 1, 1];

    assert!(layer.set_from_layout(bad_layout1, blocks.clone()).is_err());
    assert!(layer.set_from_layout(bad_layout2, blocks.clone()).is_err());
    assert!(layer.set_from_layout(bad_layout3, blocks.clone()).is_err());
    assert!(layer.set_from_layout(bad_layout4, blocks.clone()).is_err());

    // Values should be unchanged after returning error.
    assert_eq!(layer.layout().len(), 4);
    assert_eq!(layer.layout()[0], 1);
    assert_eq!(layer.blocks().len(), 4);
}


// populate
#[test] fn populate_layer_test() {
    let layout = layout![1; 4];

    let mut layer1 = Layer::<TestBlock1>::default();
    layer1.populate(layout.clone(), &"test".to_string());

    let mut layer2 = Layer::<TestBlock1>::default();
    let new_block = TestBlock1::create(&"test".to_string());
    layer2.populate_with_clones(layout, &new_block);

}


// clone
#[test] fn clone_layer_test() {
    let mut layer = Layer::<TestBlock1>::default();
    layer.populate(layout![4; 4], &"test".to_string());

    let blocks = layer.clone_into_blocks();
    layer.set_from_blocks(blocks);
}


// partial
#[test] fn partial_layer_reference_test() {
    let mut layer = Layer::<TestBlock1>::default();
    layer.populate(layout![4], &"test".to_string());

    let mut row0 = layer.get_row_mut(0)
        .expect("Should mutate the first row.");
    row0.iter_mut()
        .enumerate()
        .for_each(|(b, block)| block.id = format!("0{}", b));

    let row0 = layer.get_row_ref(0)
        .expect("Should read the first row.");
    let ids: Vec<_> = row0.iter()
        .map(|block| &block.id)
        .collect();

    assert!(layer.get_row_ref(1).is_none());
    assert!(layer.get_row_mut(1).is_none());

    let block_01 = layer.get_block_mut(0, 1)
        .expect("Should mutate the second block.");
    block_01.id = "test".to_string();
    let block_01 = layer.get_block_ref(0, 1)
        .expect("Should read the second block.");
    assert_eq!(&block_01.id, "test");

    assert!(layer.get_block_ref(1, 0).is_none());
    assert!(layer.get_block_mut(1, 0).is_none());
}

fn insert_row_test() {
    //
}
