
use blok::{ Block, Layout, layout, Stack };
use crate::block::TestBlock1;


#[test] fn new_stack_test() {
    let stack = Stack::<TestBlock1>::new();
    let layouts = stack.layouts();
    let blocks = stack.blocks();
    // TODO Assert
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


