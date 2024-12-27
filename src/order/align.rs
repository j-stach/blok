
use derive_more::{ Deref, DerefMut };


/// Associates two rows of blocks, layers of rows, or stacks of layer, by index.
/// Used for scheduling procedural connection generation.
#[derive(Debug, Default, Clone, Deref, DerefMut, Eq, PartialEq)]
pub struct Alignment(Vec<(usize, usize)>);

impl Alignment {
    /// Create an alignment from an existing vector of index pairs.
    /// ```
    /// use blok::Alignment;
    /// Alignment::wrap(vec![(0,0), (1,1), (2,2)]);
    /// ```
    pub fn wrap(vec: Vec<(usize, usize)>) -> Self { 
        Self(vec) 
    }
}


/// This is a function that generates an Alignment.
pub type Aligner<T> = fn(&Vec<T>, &Vec<T>) -> Alignment;

/// Here are some basic Aligners. 
/// You can also write your own, as needed.
impl Alignment {

    /// Align corresponding indices.
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2]);
    /// let align = Alignment::corresponding(&row1, &row1);
    /// assert_eq!(align, Alignment::wrap(vec![(0,0), (1,1), (2,2)]));
    /// ```
    pub fn corresponding<T>(
        row1: &Vec<T>, 
        row2: &Vec<T>
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
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2]);
    /// let align = Alignment::reversed(&row1, &row1);
    /// assert_eq!(align, Alignment::wrap(vec![(0,2), (1,1), (2,0)]));
    /// ```
    pub fn reversed<T>(
        row1: &Vec<T>, 
        row2: &Vec<T>
    ) -> Self {

        let r1 = row1.len();
        let r2 = row2.len();
        let max = if r1 > r2 { r2 } else { r1 };
        let mut vec = Vec::new();

        for i in 0..max { 
            vec.push((0+i, r2-1-i)) 
        }

        Self::wrap(vec)
    }

    /// Align each random element from the first row to a random element in the second.
    /// Each index is only used once.
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2]);
    /// let align = Alignment::random(&row1, &row1);
    // TODO: Test by asserting each index is present only once.
    /// ```
    pub fn random<T>(
        row1: &Vec<T>, 
        row2: &Vec<T>
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
            vec2.push(_2) 
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
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2]);
    /// let align = Alignment::dense(&row1, &row1);
    /// assert_eq!(align, Alignment::wrap(vec![(0,0), (0,1), (0,2), (1,0), (1,1), (1,2), (2,0), (2,1), (2,2)]));
    /// ```
    pub fn dense<T>(
        row1: &Vec<T>, 
        row2: &Vec<T>
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
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2, 3, 4]);
    /// let align = Alignment::centered(&row1, &row2);
    /// assert_eq!(align, Alignment::wrap(vec![(0,1), (1,2), (2,3)]));
    ///
    /// let (row3, row4) = (vec![0, 1, 2], vec![0, 1, 2, 3]);
    /// let align = Alignment::centered(&row3, &row4);
    /// assert_eq!(align, Alignment::wrap(vec![(0,0), (1,1), (2,2)]));
    ///
    /// let (row5, row6) = (vec![0, 1], vec![0, 1, 2, 3]);
    /// let align = Alignment::centered(&row5, &row6);
    /// assert_eq!(align, Alignment::wrap(vec![(0,1), (1,2)]));
    /// ```
    pub fn centered<T>(
        row1: &Vec<T>, 
        row2: &Vec<T>
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

    /// Returns an empty alignment.
    /// For when you do not want to make connections.
    /// ```
    /// use blok::Alignment;
    ///
    /// let (row1, row2) = (vec![0, 1, 2], vec![0, 1, 2]);
    /// let align = Alignment::none(&row1, &row1);
    /// assert_eq!(align, Alignment::wrap(vec![]));
    /// ```
    pub fn none<T>(
        _row1: &Vec<T>, 
        _row2: &Vec<T>
    ) -> Self {
        Self::default()
    }

}

