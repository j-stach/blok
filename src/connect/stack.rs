
use crate::build::{ block::Block, layer::Layer, stack::Stack };
use crate::connect::layer::*;

pub fn interconnect_stacks<B: Block>(stack1: &mut Stack, stack2: &mut Stack, l1: usize, l2: usize, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {

}

pub fn interconnect_corresponding_layers<B: Block>(stack1: &mut Stack, stack2: &mut Stack, r1: usize, r2: usize, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {

}

pub fn interconnect_corresponding_rows<B: Block>(stack1: &mut Stack, stack2: &mut Stack, align: Aligner<B>, instructions: Vec<B::ConnectionInstructions>) {
    // TODO
}

pub fn interconnect_corresponding_blocks<B: Block>(stack1: &mut Stack, stack2: &mut Stack, instructions: Vec<B::ConnectionInstructions>) {
    interconnect_corresponding_rows(stack1, stack2, Alignment::corresponding, instructions)
}
