
use derive_more::{ Deref, DerefMut };

use crate::Block;


/// Function for generating an Alignment from two rows.
pub type Aligner<B> = fn(&Vec<B>, &Vec<B>) -> Alignment;

/// Associates the blocks of two rows, by index.
#[derive(Deref, DerefMut)]
pub struct Alignment(Vec<(usize, usize)>);
impl Alignment {
    /// Create an alignment from an existing vector of index pairs.
    pub fn wrap(vec: Vec<(usize, usize)>) -> Self { Self(vec) }
}

/// These are default implementations for Aligners. You can write your own, too.
impl Alignment {

    /// Align corresponding indices.
    pub fn corresponding<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec = Vec::new();

        for i in 0..max { vec.push((i, i)) }
        Self::wrap(vec)
    }

    /// Align the rows end-to-end.
    pub fn reversed<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec = Vec::new();

        for i in 0..max { vec.push((0+i, r2-i)) }
        Self::wrap(vec)
    }

    /// Align a random block from the first row to a random block in the second.
    pub fn random<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        for _1 in 0..r1 { vec1.push(_1) }
        for _2 in 0..r2 { vec1.push(_2) }

        use rand::seq::SliceRandom;
        vec1.shuffle(&mut rand::thread_rng());
        vec2.shuffle(&mut rand::thread_rng());

        let mut vec = Vec::new();
        for i in 0..max { vec.push((vec1[i], vec2[i])) }
        Self::wrap(vec)
    }

    /// Align each block in one row to each block in the other.
    pub fn dense<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        let r1 = row1.len();
        let r2 = row2.len();
        let mut vec = Vec::new();

        for _1 in 0..r1 { for _2 in 0..r2 { vec.push((_1, _2)) }}
        Self::wrap(vec)
    }

    /// Centers the elignment as much as possible.
    /// Won't be perfect when row lengths are opposite parity.
    /// Follows rules for usize division (shift down).
    pub fn centered<B: Block>(row1: &Vec<B>, row2: &Vec<B>) -> Self {
        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let diff = if r1 > r2 { r1-r2 } else { r2-r1 };
        let offset = diff / 2;
        let mut vec = Vec::new();

        if r1 > r2 {
            for i in 0..max { vec.push((i+offset, i)) }
        } else {
            for i in 0..max { vec.push((i, i+offset)) }
        }

        Self::wrap(vec)
    }
}


