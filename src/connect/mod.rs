
pub mod align;
pub use align::{ Alignment, Aligner };

use crate::types::{ Block, Layer, Stack };


/// Connect two rows of blocks according to the parameters given.
/// If the number of instructions is fewer than connections to perform,
/// it will repeat the last instruction given for the remaining connections.
pub fn row_connection<B: Block>(
    row1: &mut Vec<B>, 
    row2: &mut Vec<B>, 
    align: Aligner<B>, // Used to be Alignment (for performance), but it complicated Errors
    mut instructions: Vec<B::ConnectionInstructions>
) {
    let alignment = align(row1, row2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        B::ConnectionInstructions::default()
    };

    while alignment.len() < instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if row1.len() < pair.0 && row2.len() < pair.1 {
            let block1 = &mut row1[pair.0];
            let block2 = &mut row2[pair.1];
            block1.connect(block2, &instructions[step])
        } 
        step += 1;
    }
}

// TODO: Instead of r1, r2, figure out how to use aligners for Rows as well.
/// Connect two layers using row_connection.
pub fn interconnect_layers<B: Block>(
    layer1: &mut Layer<B>, 
    layer2: &mut Layer<B>, 
    r1: usize,
    r2: usize, 
    align: Aligner<B>, 
    instructions: Vec<B::ConnectionInstructions>
) {
    if layer1.layout().len() <= r1 || layer2.layout().len() <= r2 { 
        panic!("Could not index row... Errors not yet implemented") 
    }

    let mut rows1 = layer1.clone_into_blocks();
    let mut rows2 = layer2.clone_into_blocks();
    let row1 = &mut rows1[r1];
    let row2 = &mut rows2[r2];

    row_connection(row1, row2, align, instructions);

    layer1.set_from_blocks(rows1);
    layer2.set_from_blocks(rows2);
}

/// Connect two layers for each pair of corresponding rows.
pub fn interconnect_corresponding_rows<B: Block>(
    layer1: &mut Layer<B>, 
    layer2: &mut Layer<B>, 
    align: Aligner<B>, 
    instructions: Vec<B::ConnectionInstructions>
) {
    let (l1, l2) = (layer1.layout().len(), layer2.layout().len());
    let max = if l1 > l2 {l1} else {l2};
    for i in 0..max {
        interconnect_layers(layer1, layer2, i, i, align, instructions.clone());
    }
}

/// Connect two layers for each pair of corresponding blocks in each pair of corresponding rows.
pub fn interconnect_corresponding_blocks<B: Block>(
    layer1: &mut Layer<B>, 
    layer2: &mut Layer<B>, 
    instructions: Vec<B::ConnectionInstructions>
) {
    interconnect_corresponding_rows(layer1, layer2, Alignment::corresponding, instructions)
}

/// Connect two stacks using layer_connection.
pub fn interconnect_stacks<B: Block, S: Stack<B>>(
    stack1: &mut S, 
    stack2: &mut S, 
    l1: usize, 
    l2: usize, 
    r1: usize, 
    r2: usize, 
    align: Aligner<B>, 
    instructions: Vec<B::ConnectionInstructions>
) {
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

// TODO Refine this to be more flexible.
pub fn autoconnect_stack_uniformly<B: Block, S: Stack<B>>(
    stack: &mut S, 
    align: Aligner<B>, 
    instructions: Vec<B::ConnectionInstructions>
) {
    let layers = stack.clone_into_layers();
    let mut connected = Vec::new();

    let mut current = layers[0].clone();
    for l in 1..layers.len() - 1 {
        let mut next = layers[l].clone();
        interconnect_corresponding_rows(&mut current, &mut next, align, instructions.clone());
        connected.push(current);
        current = next
    }

    stack.set_from_layers(connected)
}


