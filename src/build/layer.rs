
use crate::{ Block, Props, Layout };

pub trait Layer<'b, P: Props, B: Block<'b, P>>: Clone {
    fn new() -> Self;
    fn build(&mut self) -> Result<Self, impl std::error::Error>;

    fn blocks(&self) -> &Vec<B>;
    fn blocks_mut(&mut self) -> &mut Vec<B>;

    fn layout(&self) -> &Layout;
    fn layout_mut(&mut self) -> &mut Layout;

    fn new_row(&mut self) { self.layout_mut().push(0) }

    fn get_row(&self, row: usize) -> Option<&[B]> {
        let layout = self.layout();
        if *layout.get(row)? == 0 { return None };
        Some(&self.blocks()[layout.row_start(row)?..=layout.row_end(row)?])
    }

    fn get_block(&self, row: usize, index: usize) -> Option<&B> {
        if *self.layout().get(row)? <= index { return None };
        let mut cursor = 0usize;
        for l in &self.layout()[0..row] { cursor += l }
        cursor += index;
        self.blocks().get(cursor)
    }

    fn clone_into_rows(&self) -> Vec<Vec<B>> {
        assert_eq!(self.layout().total(), self.blocks().len());
        let mut clone = self.blocks().clone();
        let mut blocks = Vec::new();
        for r in self.layout().iter() {
            let (row, rest) = clone.split_at(*r);
            blocks.push(row.to_vec());
            clone = rest.to_vec();
        }
        blocks
    }

    fn set_from_rows(&mut self, blocks: Vec<Vec<B>>) {
        *self.layout_mut() = blocks.iter().map(|v| v.len()).collect();
        *self.blocks_mut() = blocks.into_iter().flatten().collect();
    }

    fn insert_row(&mut self, index: usize, collection: Vec<B>) {
        if self.layout().len() >= index {
            self.layout_mut().insert(index, collection.len());
            let mut rows = self.clone_into_rows();
            rows.insert(index, collection);
            self.set_from_rows(rows);
        }
    }

    fn add_block(& mut self, block: B) {
        self.blocks_mut().push(block);
        if self.layout().len() == 0 { self.new_row() }
        *self.layout_mut().last_mut().unwrap() += 1;
    }

    fn add_to_row(& mut self, row: usize, block: B) -> Result<& mut Self, anyhow::Error> {
        if self.layout().len() < row {
            let index = {
                let mut index = 0usize;
                for r_len in 0..row {
                    index += r_len
                }
                index
            };
            self.blocks_mut().insert(index, block);
            self.layout_mut()[row] += 1;
            Ok(self)
        } else { Err(anyhow::anyhow!("Row {} could not be indexed", row))}
    }

    fn insert_block(& mut self, row: usize, index: usize, block: B) -> Result<& mut Self, anyhow::Error> {
        let row_max = self.layout().len();
        if row_max > row && row_max > 0  {
            let index = {
                let mut i = 0usize;
                for r_len in 0..row - 1 {
                    i += r_len
                }
                let row_target = self.layout()[row];
                if row_target > index {
                    i += index;
                    i
                } else {
                    return Err(anyhow::anyhow!("Row {} could not be indexed at {}", row, index))
                }
            };
            self.blocks_mut().insert(index, block);
            self.layout_mut()[row] += 1;
            Ok(self)
        } else { Err(anyhow::anyhow!("Row {} could not be indexed", row))}

    }

    fn populate(& mut self, mut layout: Layout, constructor: B::Constructor) -> & mut Self {
        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(constructor())
            }
        }
        self.layout_mut().append(&mut layout);
        self
    }

    fn populate_with_clones(& mut self, mut layout: Layout, block: &B) -> & mut Self {
        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(block.clone());
            }
        }
        self.layout_mut().append(&mut layout);
        self
    }

    // TODO OFFSET

    // TODO Supertrait?
    fn realize_voids(& mut self) -> & mut Self{
        let mut rows = self.clone_into_rows();
        let max = rows.iter()
            .map(|r| r.len())
            .max()
            .expect("There should be rows present in layer before considering voids");
        for r in rows.iter_mut() {
            while r.len() < max {
                r.push(B::void());
            }
        }
        self.set_from_rows(rows);
        self
    }

    fn fill_voids(&mut self, constructor: &B::Constructor) {
        for block in self.blocks_mut().iter_mut() {
            if block.is_void() {
                *block = constructor()
            }
        }
    }

    fn fill_with_clones(&mut self, block: &B) {
        for b in self.blocks_mut().iter_mut() {
            if b.is_void() {
                *b = block.clone()
            }
        }
    }

    fn remove_voids(&mut self) {
        let mut rows = self.clone_into_rows();
        rows = rows.into_iter().map(|r| r.into_iter().filter(|b| !b.is_void()).collect()).collect();
        self.set_from_rows(rows)
    }

    /// Flip the layer across the Y axis, reversing the sequence of rows.
    fn flip_x(&mut self) {
        let rows = self.clone_into_rows().into_iter().rev().collect();
        self.set_from_rows(rows);
    }

    /// Flip the layer across the X axis, reversing the order of assemblies within the rows.
    fn flip_y(&mut self) {
        let rows = self.clone_into_rows()
            .into_iter()
            .map(|v| v.into_iter().rev().collect())
            .collect();
        self.set_from_rows(rows);
    }

    // TODO pub fn rotate_90(&mut self) -> &mut Self { todo!{}}
    // TODO pub fn rotate_180(&mut self) -> &mut Self { todo!{}}
    // TODO pub fn rotate_270(&mut self) -> &mut Self { todo!{}}

    /// Adds the other layer's rows to this layer.
    fn stitch_x(& mut self, mut other: Self) {
        self.layout_mut().append(other.layout_mut());
        self.blocks_mut().append(other.blocks_mut());
    }

    /// Appends each row with the corresponding row from the other layer.
    // TODO Describe mismatched size behavior
    fn stitch_y(& mut self, other: Self) {
        let mut s1 = self.clone_into_rows();
        let mut s2 = other.clone_into_rows();

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

        self.set_from_rows(s1);
    }


    /// Splits a layer into two at the given row number. Leaves the original in place.
    fn split_x(&mut self, split: usize) -> Self {
        let mut original = self.clone_into_rows();
        let mut new = Self::new();
        let remainder = original.split_off(split);
        self.set_from_rows(original);
        new.set_from_rows(remainder);
        new
    }

    /// Splits a structure into two by splitting each row at index given.
    fn split_y(&mut self, split: usize) -> Self {
        let mut original = self.clone_into_rows();
        let mut remainder = Vec::new();
        for row in original.iter_mut() {
            if row.len() > split {
                remainder.push(row.split_off(split));
            }
        }
        self.set_from_rows(original);
        let mut new = Self::new();
        new.set_from_rows(remainder);
        new
    }


    /// Stitches an x-flipped clone (after this layer's existing rows).
    fn mirror_x(& mut self) {
        let mut reflection = self.clone();
        reflection.flip_x();
        self.stitch_x(reflection);
    }

    /// Stitches a y-flipped clone (to the ends of this layer's rows).
    fn mirror_y(& mut self) {
        let mut reflection = self.clone();
        reflection.flip_y();
        self.stitch_y(reflection);
    }


    /// Merges the other layer into this one, by alternating rows.
    /// New layer will begin with a row originally from self.
    fn riffle_x(&mut self, other: & mut Self) {
        let rows = self.clone_into_rows();
        let other = other.clone_into_rows();
        let riffled: Vec<Vec<B>> = rows.into_iter()
            .zip(other.into_iter())
            .flat_map(|(r, o)| vec![r, o])
            .collect();
        self.set_from_rows(riffled);
    }

    /// Merges with the other layer, by alternating indices for corresponding rows.
    /// New layer's rows will begin with blocks originally from self.
    fn riffle_y(& mut self, other: & mut Self) {
        let rows = self.clone_into_rows();
        let other = other.clone_into_rows();
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
        self.set_from_rows(riffled);
    }

    // merge, fuse, transmute, ditto



}
