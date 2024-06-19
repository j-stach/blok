
use derive_more::{ Deref, DerefMut };

use crate::Block;
use crate::layout::Layout;


pub type Aligner<B> = fn(&Vec<B>, &Vec<B>) -> Alignment;

#[derive(Deref, DerefMut)]
pub struct Alignment(Vec<(usize, usize)>);
impl Alignment {
    pub fn corresponding<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
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


