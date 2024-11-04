
use super::*;
use crate::Aligner;

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

