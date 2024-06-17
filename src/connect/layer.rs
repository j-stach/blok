
use derive_more::{ Deref, DerefMut };

use crate::build::{ block::Block, layer::Layer };
use crate::layout::Layout;


pub type Aligner<B> = fn(&Vec<B>, &Vec<B>) -> Alignment;

#[derive(Deref, DerefMut)]
pub struct Alignment(Vec<(usize, usize)>);
impl Alignment {
    fn corresponding<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        todo!{}
    }

    fn reversed() -> Self {
        todo!{}
    }

    fn random() -> Self {
        todo!{}
    }

    fn dense() -> Self {
        todo!{}
    }

    fn centered() -> Self {
        todo!{}
    }
}

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

pub fn interconnect_layers<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    let mut rows1 = layer1.clone_into_blocks();
    let mut rows2 = layer2.clone_into_blocks();
    let row1 = &mut rows1[r1];
    let row2 = &mut rows2[r2];
    let alignment = align(&row1, &row2);
    row_connection(row1, row2, alignment, instructions);
    layer1.set_from_blocks(rows1);
    layer2.set_from_blocks(rows2);
}

pub fn interconnect_corresponding_rows<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    let (l1, l2) = (layer1.layout().len(), layer2.layout().len());
    let max = if l1 > l2 {l1} else {l2};
    for i in 0..max {
        interconnect_layer(layer1, layer2, i, i, align, instructions.clone());
    }
}

pub fn interconnect_corresponding_blocks<B: Block>(layer1: &mut Layer<B>, layer2: &mut Layer<B>, instructions: Vec<B::ConnectionInstructions>) {
    interconnect_corresponding_rows(layer1, layer2, Alignment::corresponding, instructions)
}

