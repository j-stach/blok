
use crate::{ Block, Layout, layout };

// TODO Furthermore, should there be a "Row" type?
// Also "LayerRef" and "RowRef" types?

// TODO rotate buffer_xy <-(offset at end)


/// Holds a grid layout of blocks in a single vector,
/// with row indexing stored separately.
#[derive(Clone)]
pub struct Layer<B: Block> {
    /// Represents the organization of blocks in the array.
    layout: Layout,
    /// Blok assumes you want to store the array contiguously in memory.
    blocks: Vec<B>
}

impl<B: Block> Layer<B> {
    /// Create an empty layer.
    pub fn new() -> Self { Layer { layout: layout![], blocks: vec![] }}

    /// Get a reference to the blocks of the layer.
    pub fn blocks(&self) -> &Vec<B> { &self.blocks }
    /// Get a mutable reference to the blocks of the layer.
    pub(crate) fn blocks_mut(&mut self) -> &mut Vec<B> { &mut self.blocks }

    /// Get a reference to the layout of the layer.
    pub fn layout(&self) -> &Layout { &self.layout }
    /// Get a mutable reference to the layout of the layer.
    pub(crate) fn layout_mut(&mut self) -> &mut Layout { &mut self.layout }

    // TODO Make Result type?
    /// Set the layer to the provided layout and blocks.
    /// WARNING: Panics if the layout total and number of blocks do not match.
    pub fn set_from_layout(&mut self, layout: Layout, blocks: Vec<B>) {
        assert_eq!(layout.total(), blocks.len());
        *self.layout_mut() = layout;
        *self.blocks_mut() = blocks;
    }

    /// Get the span of blocks representing the given row number.
    pub fn get_row(&self, row: usize) -> Option<&[B]> {
        let layout = self.layout();
        if *layout.get(row)? == 0 { return None };
        Some(&self.blocks()[layout.row_start(row)?..=layout.row_end(row)?])
    }

    /// Get a singular block given the row and index in the layer.
    pub fn get_block(&self, row: usize, index: usize) -> Option<&B> {
        if *self.layout().get(row)? <= index { return None };
        let mut cursor = 0usize;
        for l in &self.layout()[0..row] { cursor += l }
        cursor += index;
        self.blocks().get(cursor)
    }

    /// Clone the layer into a matrix of blocks.
    pub fn clone_into_blocks(&self) -> Vec<Vec<B>> {
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

    /// Overwrite a layer's values from a matrix of blocks.
    pub fn set_from_blocks(&mut self, blocks: Vec<Vec<B>>) {
        *self.layout_mut() = blocks.iter().map(|v| v.len()).collect();
        *self.blocks_mut() = blocks.into_iter().flatten().collect();
    }

    /// Add an empty row to the layer.
    pub fn new_row(&mut self) { self.layout_mut().push(0) }

    /// Add a collection of blocks as a new row.
    pub fn add_row(&mut self, mut collection: Vec<B>) {
        self.layout_mut().push(collection.len());
        self.blocks_mut().append(&mut collection)
    }

    /// Insert a collection of blocks as a new row at the given index.
    pub fn insert_row(&mut self, index: usize, collection: Vec<B>) {
        if self.layout().len() >= index {
            self.layout_mut().insert(index, collection.len());
            let mut rows = self.clone_into_blocks();
            rows.insert(index, collection);
            self.set_from_blocks(rows);
        }
    }

    /// Add a new block to the last row in the layer.
    pub fn add_block(&mut self, block: B) {
        self.blocks_mut().push(block);
        if self.layout().len() == 0 { self.new_row() }
        *self.layout_mut().last_mut().unwrap() += 1;
    }

    /// Add a block to the end of the given row.
    pub fn add_to_row(&mut self, row: usize, block: B) -> Result<&mut Self, anyhow::Error> { // TODO Error
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

    /// Insert a block into the given row, at the given index.
    pub fn insert_block(
        &mut self,
        row: usize,
        index: usize,
        block: B
    ) -> Result<&mut Self, anyhow::Error> { // TODO Error
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
                    return Err(anyhow::anyhow!("Row {} could not be indexed at {}", row, index)) // TODO Error
                }
            };
            self.blocks_mut().insert(index, block);
            self.layout_mut()[row] += 1;
            Ok(self)
        } else { Err(anyhow::anyhow!("Row {} could not be indexed", row))}
    }

    /// Create blocks using the given constructor,
    /// adding them in rows according to the given layout.
    pub fn populate(&mut self, mut layout: Layout, c: B::Constructor) -> &mut Self {
        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(c())
            }
        }
        self.layout_mut().append(&mut layout);
        self
    }

    /// Create blocks by cloning a prototype,
    /// adding them in rows according to the given layout.
    pub fn populate_with_clones(&mut self, mut layout: Layout, block: &B) -> &mut Self {
        for row in layout.iter() {
            for _ in 0..*row {
                self.blocks_mut().push(block.clone());
            }
        }
        self.layout_mut().append(&mut layout);
        self
    }

    /// Offset with an empty row
    // TBD Void block or simple imaginary row?
    pub fn offset_x(&mut self, offset: usize) {
        let mut layout = vec![0; offset];
        layout.append(self.layout_mut());
        **self.layout_mut() = layout;
    }

    /// Offset a row with void blocks.
    pub fn offset_row(&mut self, row: usize, offset: usize) -> Result<(), anyhow::Error> {
        if row > self.layout().len() {
            return Err(anyhow::anyhow!("Row {} could not be found", row))
        }
        for _ in 0..offset {
            self.insert_block(row, 0, B::void())?;
        }
        Ok(())
    }

    /// Offset all rows with void blocks.
    pub fn offset_y(&mut self, offset: usize) {
        for r in 0..self.layout().len() {
            self.offset_row(r, offset)
                .expect("Error: Layout does not reflect actual blocks")
        }
    }

    /// Square off the matrix to the highest row length,
    /// by inserting void blocks into the empty indices.
    pub fn realize_voids(&mut self) -> &mut Self {
        let mut rows = self.clone_into_blocks();
        let max = rows.iter()
            .map(|r| r.len())
            .max()
            .expect("There should be rows present in layer before considering voids");
        for r in rows.iter_mut() {
            while r.len() < max {
                r.push(B::void());
            }
        }
        self.set_from_blocks(rows);
        self
    }

    /// Square off the matrix up to the given x and y,
    /// by inserting void blocks into the empty indices.
    pub fn realize_volume(&mut self, x: usize, y: usize) -> &mut Self {
        let mut rows = self.clone_into_blocks();
        for r in rows.iter_mut() {
            while r.len() < y {
                r.push(B::void());
            }
        }
        while rows.len() < x {
            rows.push(vec![B::void(); y])
        }
        self.set_from_blocks(rows);
        self
    }

    /// Replace all void blocks with ones generated by the given constructor.
    pub fn fill_voids(&mut self, constructor: &B::Constructor) {
        for block in self.blocks_mut().iter_mut() {
            if block.is_void() {
                *block = constructor()
            }
        }
    }

    /// Replace all void blocks with ones cloned from a prototype.
    pub fn fill_with_clones(&mut self, block: &B) {
        for b in self.blocks_mut().iter_mut() {
            if b.is_void() {
                *b = block.clone()
            }
        }
    }

    /// Remove all void blocks from the matrix.
    pub fn remove_voids(&mut self) {
        let mut rows = self.clone_into_blocks();
        rows = rows.into_iter().map(|r| r.into_iter().filter(|b| !b.is_void()).collect()).collect();
        self.set_from_blocks(rows)
    }

    /// Flip the layer across the Y axis, reversing the sequence of rows.
    pub fn flip_x(&mut self) {
        let rows = self.clone_into_blocks().into_iter().rev().collect();
        self.set_from_blocks(rows);
    }

    /// Flip the layer across the X axis, reversing the order of blocks within the rows.
    pub fn flip_y(&mut self) {
        let rows = self.clone_into_blocks()
            .into_iter()
            .map(|v| v.into_iter().rev().collect())
            .collect();
        self.set_from_blocks(rows);
    }

    // TODO pub fn rotate_90(&mut self) -> &mut Self { todo!{}}
    // TODO pub fn rotate_180(&mut self) -> &mut Self { todo!{}}
    // TODO pub fn rotate_270(&mut self) -> &mut Self { todo!{}}

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
        let mut new = Self::new();
        let remainder = original.split_off(split);
        self.set_from_blocks(original);
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
        let mut new = Self::new();
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
    /// New layer will begin with a row originally from self.
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
    /// New layer's rows will begin with blocks originally from self.
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
