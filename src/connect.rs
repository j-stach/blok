
use crate::{ Block, Row, Layer, Stack, Aligner };

// TODO Need a convenient way to map alignments into connection instructions 


/// Connect two row refs of blocks according to the parameters given.
/// If the number of instructions is fewer than connections to perform,
/// it will repeat the last instruction given for the remaining connections.
pub fn row_connection<'b, B: Block>(
    row1: &mut Vec<&'b mut B>, 
    row2: &mut Vec<&'b mut B>, 
    block_align: Aligner<&'b mut B>, // Used to be Alignment (for performance), 
                                     // but it complicated types and errors too much
    mut instructions: Vec<B::ConnectionInstructions>
) {
    let alignment = block_align(row1, row2);

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

impl<B: Block> Row<B> {
    //
}


/// Connect two layer refs using row_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn layer_connection<'b, B: Block>(
    layer1: &mut Vec<Vec<&'b mut B>>, 
    layer2: &mut Vec<Vec<&'b mut B>>, 
    row_align: Aligner<Vec<&'b mut B>>,
    block_align: Aligner<&'b mut B>, 
    mut instructions: Vec<Vec<B::ConnectionInstructions>>
) {

    let alignment = row_align(&layer1, &layer2);

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
        if layer1.len() < pair.0 && layer2.len() < pair.1 {
            let row1 = &mut layer1[pair.0];
            let row2 = &mut layer2[pair.1];
            row_connection(row1, row2, block_align, instructions[step].clone());
        } 
        step += 1;
    }
}

impl<B: Block> Layer<B> {
    //
}


/// Connect two stack refs using layer_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn stack_connection<'b, B: Block>(
    stack1: &mut Vec<Vec<Vec<&'b mut B>>>, 
    stack2: &mut Vec<Vec<Vec<&'b mut B>>>, 
    layer_align: Aligner<Vec<Vec<&'b mut B>>>,
    row_align: Aligner<Vec<&'b mut B>>,
    block_align: Aligner<&'b mut B>, 
    mut instructions: Vec<Vec<Vec<B::ConnectionInstructions>>>
) {

    let alignment = layer_align(&stack1, &stack2);

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
        if stack1.len() < pair.0 && stack2.len() < pair.1 {
            let layer1 = &mut stack1[pair.0];
            let layer2 = &mut stack2[pair.1];
            layer_connection(layer1, layer2, row_align, block_align, instructions[step].clone());
        } 
        step += 1;
    }
}

impl<B: Block> Stack<B> {
    //
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
