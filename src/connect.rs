
use crate::types::{ Block, Layer, Stack };
use crate::align::{ Alignment, Aligner };

// TODO autoconnect, or a "Connectable" trait for row/layer/stack?


/// Connect two rows of blocks according to the parameters given.
pub fn row_connection<B: Block>(row1: &mut Vec<B>, row2: &mut Vec<B>, alignment: Alignment, instructions: Vec<B::ConnectionInstructions>) {
    // TODO if instructions are incomplete for alignment, fill it in
    assert_eq!(alignment.len(), instructions.len());
    let mut step = 0usize;
    for pair in alignment.iter() {
        // TODO assert alignment
        if row1.len() < pair.0 && row2.len() < pair.1 {
            row1[pair.0].connect(&mut row2[pair.1], &instructions[step])
        }
        step += 1;
        // TODO Logging hookup for errors, just skip otherwise
    }
}

/// Connect two layers using row_connection.
pub fn interconnect_layers<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    if layer1.layout().len() <= r1 || layer2.layout().len() <= r2 { panic!("Could not index row... Errors not yet implemented") }
    let mut rows1 = layer1.clone_into_blocks();
    let mut rows2 = layer2.clone_into_blocks();
    let row1 = &mut rows1[r1];
    let row2 = &mut rows2[r2];
    let alignment = align(&row1, &row2);
    row_connection(row1, row2, alignment, instructions);
    layer1.set_from_blocks(rows1);
    layer2.set_from_blocks(rows2);
}

/// Connect two layers for each pair of corresponding rows.
pub fn interconnect_corresponding_rows<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    let (l1, l2) = (layer1.layout().len(), layer2.layout().len());
    let max = if l1 > l2 {l1} else {l2};
    for i in 0..max {
        interconnect_layers(layer1, layer2, i, i, align, instructions.clone());
    }
}

/// Connect two layers for each pair of corresponding blocks in each pair of corresponding rows.
pub fn interconnect_corresponding_blocks<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, instructions: Vec<B::ConnectionInstructions>) {
    interconnect_corresponding_rows(layer1, layer2, Alignment::corresponding, instructions)
}

/// Connect two stacks using layer_connection.
pub fn interconnect_stacks<B: Block, S: Stack<B>>(stack1: &mut S, stack2: &mut S, l1: usize, l2: usize, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    if stack1.layouts().len() <= l1 || stack2.layouts().len() <= l2 { panic!("Could not index layer... Errors not yet implemented") }
    let mut layers1 = stack1.clone_into_layers();
    let mut layers2 = stack2.clone_into_layers();
    let layer1 = &mut layers1[l1];
    let layer2 = &mut layers2[l2];
    interconnect_layers(layer1, layer2, r1, r2, align, instructions);
    stack1.set_from_layers(layers1);
    stack2.set_from_layers(layers2);
}

//pub fn interconnect_corresponding_layers<B: Block>(stack1: &mut Stack, stack2: &mut Stack, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
//    // stack length
//    // interconnect layers upt to min stack length
//
//}

// TODO fn autoconnect_layers_stepwise / uniformly
