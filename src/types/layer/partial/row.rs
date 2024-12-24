
use crate::{ Block, Layer };

/// Methods for partial data access:
impl<B: Block> Layer<B> {

    /// Get a range representing the layout row from start to end.
    /// Returns an error if the row doesn't exist within the layer.
    /// Returns None if the row is empty (contains no blocks).
    pub fn find_row_bounds(&self, r: usize) -> anyhow::Result<Option<(usize, usize)>> {
        self.layout.row_range(r)
    }

    /// Find the block index for the start of a row.
    /// Returns an error if the row doesn't exist within the layer.
    /// Returns None if the row is empty (contains no blocks).
    pub fn find_row_start(&self, r: usize) -> anyhow::Result<Option<usize>> {
        self.layout.row_start(r)
    }

    /// Find the block index for the end of a row.
    /// Returns an error if the row doesn't exist within the layer.
    /// Returns None if the row is empty (contains no blocks).
    pub fn find_row_end(&self, r: usize) -> anyhow::Result<Option<usize>> {
        self.layout.row_end(r)
    }

    /// Get a vector of references to the blocks that represent a layer row.
    /// Returns None if the row could not be found, and an empty vector if the row is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_row_ref(&self, r: usize) -> Option<Vec<&B>> {
        if let Ok(row_bounds) = self.find_row_bounds(r) {
            // If the row is empty, it will not have a start or end (None).
            if let Some((start, end)) = row_bounds {
                let row = self.get_range_ref(start, end)?;
                Some(row)
            } else {
                // If the row is empty, return an empty Vec.
                Some(Vec::new())
            }
        } else {
            // If the row doesn't exist, return None.
            None
        }
    }

    /// Get a vector of mutable references to the blocks that represent a layer row.
    /// Returns None if the row could not be found, and an empty vector if the row is empty.
    /// Use this for operations on a collection of blocks, not for building layer structure.
    /// (Adding to this vector will not add blocks to the layer.)
    pub fn get_row_mut(&mut self, r: usize) -> Option<Vec<&mut B>> {
        if let Ok(row_bounds) = self.find_row_bounds(r) {
            if let Some((start, end)) = row_bounds {
                // The only difference is that we get mutable references.
                let row = self.get_range_mut(start, end)?;
                Some(row)
            } else {
                Some(Vec::new())
            }
        } else {
            None
        }
    }

}
