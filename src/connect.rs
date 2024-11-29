
use crate::{ Block, Row, Layer, Stack, Aligner };

// TODO Need a convenient way to map alignments into connection instructions 


/// Connect two row refs of blocks according to the parameters given.
/// If the number of instructions is fewer than connections to perform,
/// it will repeat the last instruction given for the remaining connections.
pub fn row_connection<'c, B: Block>(
    row1: &mut Vec<&'c mut B>, 
    row2: &mut Vec<&'c mut B>, 
    block_align: Aligner<&'c mut B>, 
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

    /// Method version of row_connection
    pub fn connect<'c>(
        &'c mut self,
        mut other: &'c mut Row<B>,
        block_align: Aligner<&'c mut B>, 
        mut instructions: Vec<B::ConnectionInstructions>
    ) -> anyhow::Result<()> {

        let this = self.get_all_mut();
        let other = other.get_all_mut();
        if this.is_none() || other.is_none() {
            return Err(anyhow::anyhow!("Could not reference rows"))
        }

        let (mut this, mut other) = (this.unwrap(), other.unwrap());
        row_connection(&mut this, &mut other, block_align, instructions);

        Ok(())
    }

}


/// Connect two layer refs using row_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn layer_connection<'c, B: Block>(
    layer1: &mut Vec<Vec<&'c mut B>>, 
    layer2: &mut Vec<Vec<&'c mut B>>, 
    row_align: Aligner<Vec<&'c mut B>>,
    block_align: Aligner<&'c mut B>, 
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
    // TODO connect method
    // and get_all_mut
}


/// Connect two stack refs using layer_connection.
/// If the number of instruction lists is fewer than the number of rows to be connected,
/// it will repeat the last instruction given for the remaining connections.
pub fn stack_connection<'c, B: Block>(
    stack1: &mut Vec<Vec<Vec<&'c mut B>>>, 
    stack2: &mut Vec<Vec<Vec<&'c mut B>>>, 
    layer_align: Aligner<Vec<Vec<&'c mut B>>>,
    row_align: Aligner<Vec<&'c mut B>>,
    block_align: Aligner<&'c mut B>, 
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

    // TODO connect method
    // and get_all_mut
}



