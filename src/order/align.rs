
use derive_more::{ Deref, DerefMut };


/// Associates two rows of blocks, layers of rows, or stacks of layer, by index.
/// Used for scheduling procedural connection generation.
#[derive(Debug, Default, Clone, Deref, DerefMut)]
pub struct Alignment(Vec<(usize, usize)>);

impl Alignment {
    /// Create an alignment from an existing vector of index pairs.
    pub fn wrap(vec: Vec<(usize, usize)>) -> Self { 
        Self(vec) 
    }
}


/// Function type for generating an Alignment.
pub type Aligner<T> = fn(&Vec<T>, &Vec<T>) -> Alignment;

/// Here are some basic Aligners. 
/// You can also write your own, as needed.
impl Alignment {

    /// Align corresponding indices.
    pub fn corresponding<B>(
        row1: &Vec<B>, 
        row2: &Vec<B>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec = Vec::new();

        for i in 0..max { 
            vec.push((i, i)) 
        }

        Self::wrap(vec)
    }

    /// Align the elements end-to-end.
    pub fn reversed<B>(
        row1: &Vec<B>, 
        row2: &Vec<B>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec = Vec::new();

        for i in 0..max { 
            vec.push((0+i, r2-i)) 
        }

        Self::wrap(vec)
    }

    /// Align a random element from the first row to a random element in the second.
    pub fn random<B>(
        row1: &Vec<B>, 
        row2: &Vec<B>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };

        let mut vec1 = Vec::new();
        let mut vec2 = Vec::new();
        for _1 in 0..r1 { 
            vec1.push(_1) 
        }
        for _2 in 0..r2 { 
            vec1.push(_2) 
        }

        use rand::seq::SliceRandom;
        vec1.shuffle(&mut rand::thread_rng());
        vec2.shuffle(&mut rand::thread_rng());

        let mut vec = Vec::new();
        for i in 0..max { 
            vec.push((vec1[i], vec2[i])) 
        }

        Self::wrap(vec)
    }

    /// Align each element in one row to each element in the other.
    pub fn dense<B>(
        row1: &Vec<B>, 
        row2: &Vec<B>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();
        let mut vec = Vec::new();

        for _1 in 0..r1 { 
            for _2 in 0..r2 { 
                vec.push((_1, _2)) 
            }
        }

        Self::wrap(vec)
    }

    /// Centers the elignment as much as possible.
    /// Won't be perfect when lengths are opposite parity.
    /// Follows rules for usize division (round down).
    pub fn centered<B>(
        row1: &Vec<B>, 
        row2: &Vec<B>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();

        let (max, diff) = if r1 > r2 { 
            (r2, r1 - r2)
        } else { 
            (r1, r2 - r1)
        };

        let offset = diff / 2;
        let mut vec = Vec::new();

        if r1 > r2 {
            for i in 0..max { 
                vec.push((i+offset, i)) 
            }
        } else {
            for i in 0..max { 
                vec.push((i, i+offset)) 
            }
        }

        Self::wrap(vec)
    }
}

