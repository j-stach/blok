
use crate::{ Node, Row, Layer, Stack, Aligner };

// TODO Split into modules?

/// Connect two row refs of blocks according to the parameters given.
/// If the number of instructions is fewer than connections to perform,
/// it will repeat the last instruction given for the remaining connections.
pub fn row_connection<'c, N: Node>(
    row1: &mut Vec<&'c mut N>, 
    row2: &mut Vec<&'c mut N>, 
    block_align: Aligner<&'c mut N>, 
    mut instructions: Vec<N::ConnectionInstructions>
) {

    let alignment = block_align(row1, row2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        N::ConnectionInstructions::default()
    };

    while alignment.len() > instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if row1.len() > pair.0 && row2.len() > pair.1 {
            let block1 = &mut row1[pair.0];
            let block2 = &mut row2[pair.1];
            block1.connect(block2, &instructions[step])
        } 
        step += 1;
    }
}

impl<N: Node> Row<N> {

    /// Method version of row_connection.
    pub fn connect<'c>(
        &'c mut self,
        other: &'c mut Self,
        block_align: Aligner<&'c mut N>, 
        instructions: Vec<N::ConnectionInstructions>
    ) {

        let mut this = self.get_all_mut();
        let mut other = other.get_all_mut();
        row_connection(&mut this, &mut other, block_align, instructions);
    }

}


/// Connect two layer refs using row_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn layer_connection<'c, N: Node>(
    layer1: &mut Vec<Vec<&'c mut N>>, 
    layer2: &mut Vec<Vec<&'c mut N>>, 
    row_align: Aligner<Vec<&'c mut N>>,
    block_align: Aligner<&'c mut N>, 
    mut instructions: Vec<Vec<N::ConnectionInstructions>>
) {

    let alignment = row_align(&layer1, &layer2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        vec![N::ConnectionInstructions::default()]
    };

    while alignment.len() > instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if layer1.len() > pair.0 && layer2.len() > pair.1 {
            let row1 = &mut layer1[pair.0];
            let row2 = &mut layer2[pair.1];
            row_connection(row1, row2, block_align, instructions[step].clone());
        } 
        step += 1;
    }
}

impl<N: Node> Layer<N> {

    /// Method version of layer_connection.
    pub fn connect<'c>(
        &'c mut self, 
        other: &'c mut Self, 
        row_align: Aligner<Vec<&'c mut N>>,
        block_align: Aligner<&'c mut N>, 
        instructions: Vec<Vec<N::ConnectionInstructions>>
    ) {

        let mut this = self.get_all_mut();
        let mut other = other.get_all_mut();

        layer_connection(
            &mut this, 
            &mut other, 
            row_align,
            block_align, 
            instructions
        );
    }

}


/// Connect two stack refs using layer_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn stack_connection<'c, N: Node>(
    stack1: &mut Vec<Vec<Vec<&'c mut N>>>, 
    stack2: &mut Vec<Vec<Vec<&'c mut N>>>, 
    layer_align: Aligner<Vec<Vec<&'c mut N>>>,
    row_align: Aligner<Vec<&'c mut N>>,
    block_align: Aligner<&'c mut N>, 
    mut instructions: Vec<Vec<Vec<N::ConnectionInstructions>>>
) {

    let alignment = layer_align(&stack1, &stack2);

    let last_instr = if let Some(instr) = instructions.last() {
        instr.clone()
    } else {
        vec![vec![N::ConnectionInstructions::default()]]
    };

    while alignment.len() > instructions.len() {
        instructions.push(last_instr.clone())
    }

    let mut step = 0usize;
    for pair in alignment.iter() {
        if stack1.len() > pair.0 && stack2.len() > pair.1 {
            let layer1 = &mut stack1[pair.0];
            let layer2 = &mut stack2[pair.1];
            layer_connection(layer1, layer2, row_align, block_align, instructions[step].clone());
        } 
        step += 1;
    }
}

impl<N: Node> Stack<N> {

    /// Method version of stack_connection.
    pub fn connect<'c>(
        &'c mut self, 
        other: &'c mut Self, 
        layer_align: Aligner<Vec<Vec<&'c mut N>>>,
        row_align: Aligner<Vec<&'c mut N>>,
        block_align: Aligner<&'c mut N>, 
        instructions: Vec<Vec<Vec<N::ConnectionInstructions>>>
    ) {

        let mut this = self.get_all_mut();
        let mut other = other.get_all_mut();

        stack_connection(
            &mut this, 
            &mut other, 
            layer_align,
            row_align,
            block_align, 
            instructions
        );
    }

}



