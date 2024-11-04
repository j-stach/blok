
use super::*;

/// Layer transformation functions:
impl<B: Block> Layer<B> {

    /// Flip the layer across the Y axis, reversing the sequence of rows.
    pub fn flip_x(&mut self) {
        let rows = self.clone_into_blocks()
            .into_iter()
            .rev()
            .collect();

        self.set_from_blocks(rows);
    }

    /// Flip the layer across the X axis, reversing the order of blocks within the rows.
    pub fn flip_y(&mut self) {
        let rows = self.clone_into_blocks()
            .into_iter()
            .map(|row| 
                row.into_iter()
                    .rev()
                    .collect()
            )
            .collect();

        self.set_from_blocks(rows);
    }

    // TODO pub fn rotate_90(&mut self) -> &mut Self 
    // TODO pub fn rotate_180(&mut self) -> &mut Self
    // TODO pub fn rotate_270(&mut self) -> &mut Self

    /// Add the other layer's rows to this layer.
    pub fn stitch_x(&mut self, other: &mut Self) {
        self.layout_mut().append(other.layout_mut());
        self.blocks_mut().append(other.blocks_mut());
    }

    /// Append each row with the corresponding row from the other layer.
    // TODO Describe mismatched size behavior
    pub fn stitch_y(&mut self, other: &mut Self) {
        let mut s1 = self.clone_into_blocks();
        let mut s2 = other.clone_into_blocks();

        if s1.len() >= s2.len() {
            for (r, row1) in s1.iter_mut().enumerate() {
                if let Some(row2) = s2.get_mut(r) {
                    row1.append(row2);
                }
            }
        } else {
            for (r, row2) in s2.iter_mut().enumerate() {
                if let Some(row1) = s1.get_mut(r) {
                    row1.append(row2);
                } else {
                    s1.push(row2.clone())
                }
            }
        }

        self.set_from_blocks(s1);
    }

    /// Split a layer into two at the given row number. Leaves the original in place.
    pub fn split_x(&mut self, split: usize) -> Self {
        let mut original = self.clone_into_blocks();
        let remainder = original.split_off(split);

        self.set_from_blocks(original);

        let mut new = Self::default();
        new.set_from_blocks(remainder);
        new
    }

    /// Split a layer into two by splitting each row at index given.
    pub fn split_y(&mut self, split: usize) -> Self {
        let mut original = self.clone_into_blocks();
        let mut remainder = Vec::new();

        for row in original.iter_mut() {
            if row.len() > split {
                remainder.push(row.split_off(split));
            }
        }

        self.set_from_blocks(original);

        let mut new = Self::default();
        new.set_from_blocks(remainder);
        new
    }

    /// Stitch an x-flipped clone (after this layer's existing rows).
    pub fn mirror_x(&mut self) {
        let mut reflection = self.clone();
        reflection.flip_x();
        self.stitch_x(&mut reflection);
    }

    /// Stitch a y-flipped clone (to the ends of this layer's rows).
    pub fn mirror_y(&mut self) {
        let mut reflection = self.clone();
        reflection.flip_y();
        self.stitch_y(&mut reflection);
    }

    /// Merge the other layer into this one, by alternating rows.
    /// New layer will begin with a row originally from "self".
    pub fn riffle_x(&mut self, other: &mut Self) {
        let rows = self.clone_into_blocks();
        let other = other.clone_into_blocks();

        let riffled: Vec<Vec<B>> = rows.into_iter()
            .zip(other.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();

        self.set_from_blocks(riffled);
    }

    /// Merge with the other layer, by alternating indices for corresponding rows.
    /// New layer's rows will begin with blocks originally from "self".
    pub fn riffle_y(&mut self, other: &mut Self) {
        let rows = self.clone_into_blocks();
        let other = other.clone_into_blocks();

        let riffled: Vec<Vec<B>> = rows.into_iter()
            .zip(other.into_iter())
            .map(|(r, o)| {
                let r: Vec<B> = r.into_iter()
                    .zip(o.into_iter())
                    .flat_map(|(rr, oo)| vec![rr, oo])
                    .collect();
                r
            })
            .collect();

        self.set_from_blocks(riffled);
    }

}
