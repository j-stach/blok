
use super::*;

/// Connect two stacks using layer_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
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

    // TODO Connect without clone
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
