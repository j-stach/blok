
pub mod align;
pub use align::{ Alignment, Aligner };

use crate::types::{ Block, Layer, Stack };

// TODO: DRY alignment-driven execution of functions


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

/// Connect two layers using row_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn layer_connection<B: Block>(
    layer1: &mut Layer<B>, 
    layer2: &mut Layer<B>, 
    row_align: Aligner<Vec<B>>,
    block_align: Aligner<B>, 
    mut instructions: Vec<Vec<B::ConnectionInstructions>>
) {

    // TODO Connect without clone
    let mut rows1 = layer1.clone_into_blocks();
    let mut rows2 = layer2.clone_into_blocks();

    let alignment = row_align(&rows1, &rows2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        vec![B::ConnectionInstructions::default()]
    };

    while alignment.len() < instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if rows1.len() < pair.0 && rows2.len() < pair.1 {
            let row1 = &mut rows1[pair.0];
            let row2 = &mut rows2[pair.1];
            row_connection(row1, row2, block_align, instructions[step].clone());
        } 
        step += 1;
    }

    layer1.set_from_blocks(rows1);
    layer2.set_from_blocks(rows2);
}

/// Connect two stacks using layer_connection.
pub fn stack_connection<B: Block, S: Stack<B>>(
    stack1: &mut S, 
    stack2: &mut S, 
    layer_align: Aligner<Layer<B>>,
    row_align: Aligner<Vec<B>>,
    block_align: Aligner<B>, 
    mut instructions: Vec<Vec<Vec<B::ConnectionInstructions>>>
) {

    // TODO Connect without clone
    let mut layers1 = stack1.clone_into_layers();
    let mut layers2 = stack2.clone_into_layers();

    let alignment = layer_align(&layers1, &layers2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        vec![vec![B::ConnectionInstructions::default()]]
    };

    while alignment.len() < instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if layers1.len() < pair.0 && layers2.len() < pair.1 {
            let layer1 = &mut layers1[pair.0];
            let layer2 = &mut layers2[pair.1];
            layer_connection(layer1, layer2, row_align, block_align, instructions[step].clone());
        } 
        step += 1;
    }

    stack1.set_from_layers(layers1);
    stack2.set_from_layers(layers2);
}

/*
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

*/
