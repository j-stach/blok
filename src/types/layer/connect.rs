
use super::*;

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

    // TODO Connect without clone
    layer1.set_from_blocks(rows1);
    layer2.set_from_blocks(rows2);
}

