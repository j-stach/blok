
use blok::{ Block, Row, Layout, layout, Layer, Stack };
use crate::block::TestBlock1;


#[test] fn new_stack_test() {
    let stack = Stack::<TestBlock1>::new();
    let layouts = stack.layouts();
    let blocks = stack.blocks();
    // TODO Assert
}

#[test] fn build_stack_test() {
    let mut stack = Stack::<TestBlock1>::new();
    let new_block = |id: &str| TestBlock1::create(&id.to_string());

    stack.add_block(new_block("000"));
    stack.add_blocks(vec![
        new_block("001"),
        new_block("002"),
        new_block("003"),
    ]);
    // TODO Assert

    /*
    let row = Row::wrap(vec![
        new_block("010"),
        new_block("011"),
    ]);
    stack.add_row(row);
    stack.add_block_to_layer(0, new_block("012"));
    stack.add_block_to_row(0, 1, new_block("013"));
    // TODO Assert

    stack.new_layer();
    let row0 = Row::wrap(vec![
        new_block("200"),
        new_block("201"),
        new_block("202"),
        new_block("203"),
    ]);
    let row1 = Row::wrap(vec![
        new_block("210"),
        new_block("211"),
        new_block("212"),
        new_block("213"),
    ]);
    stack.add_rows(vec![row0, row1]);
    // TODO Assert

    let layer1 = Layer::<TestBlock1>::new();
    stack.insert_layer(1, layer1)
        .expect("Insert the second layer in the middle");
    let row2 = Row::wrap(vec![
        new_block("100"),
        new_block("101"),
        new_block("102"),
        new_block("103"),
    ]);
    stack.add_row_to_layer(1, row2)
        .expect("Add the first row to the second layer");
    let row3 = Row::wrap(vec![
        new_block("111"),
    ]);
    stack.insert_row(1, 1, row3)
        .expect("Insert the second row for the second layer");
    stack.insert_block(1, 1, 0, new_block("110"));
    stack.insert_blocks(1, 1, 2, vec![
        new_block("112"),
        new_block("113"),
    ])
        .expect("Insert blocks at the end of the second row, second layer");
    // TODO Assert

    let mut layer2 = Layer::<TestBlock1>::new();
    layer2.populate_with_clones(layout![4; 2], &TestBlock1::create(&"top".to_string()));
    stack.add_layer(layer2);
    // TODO Assert
 */

}

#[test] fn populate_stack_test() {
    let mut stack1 = Stack::<TestBlock1>::default();
    let mut stack2 = Stack::<TestBlock1>::default();

    let layouts = vec![layout!(2,2), layout!(2,2)];
    stack1.populate(layouts.clone(), &"test1".to_string());
    // TODO Assert

    let proto = TestBlock1::create(&"test2".to_string());
    stack2.populate_with_clones(layouts.clone(), &proto);
    // TODO Assert
}

#[test] fn clone_stack_test() {
    let mut stack = Stack::<TestBlock1>::default();
    let layouts = vec![layout!(2,2), layout!(2,2)];
    stack.populate(layouts.clone(), &"test1".to_string());

    let layers = stack.clone_into_layers();
    stack.set_from_layers(layers);
    // Assert

    let rows = stack.clone_into_rows();
    stack.set_from_rows(rows);
    // Assert

    let blocks = stack.clone_into_blocks();
    stack.set_from_blocks(blocks);
    // Assert
}

#[test] fn stack_voids_test() {
    let mut stack = Stack::<TestBlock1>::default();
    let layouts = vec![layout!(2,2), layout!(2,2)];
    stack.populate(layouts.clone(), &"test1".to_string());

    // offset, etc.
    // realize fill fillclones compress
}

#[test] fn partial_stack_reference_test() {
    let mut stack = Stack::<TestBlock1>::default();
    let layouts = vec![layout!(2,2), layout!(2,2)];
    stack.populate(layouts.clone(), &"test1".to_string());

    // block row range layer
}


