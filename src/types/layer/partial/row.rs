
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



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use super::*;
    use crate::{ Block, Layer, Layout };
    use crate::types::layer::test::test_layer;

    ///
    #[test] fn find_row_bounds_test() {

        // Test layer layout is [1, 2]
        let mut layer = test_layer();
        layer.new_row();
        // row bounds, start, end

        assert_eq!(layer.find_row_start(0).unwrap(), Some(0));
        assert_eq!(layer.find_row_end(0).unwrap(), Some(0));
        assert_eq!(layer.find_row_bounds(0).unwrap(), Some((0,0)));

        assert_eq!(layer.find_row_start(1).unwrap(), Some(1));
        assert_eq!(layer.find_row_end(1).unwrap(), Some(2));
        assert_eq!(layer.find_row_bounds(1).unwrap(), Some((1,2)));

        assert_eq!(layer.find_row_start(2).unwrap(), None);
        assert_eq!(layer.find_row_end(2).unwrap(), None);
        assert_eq!(layer.find_row_bounds(2).unwrap(), None);

        assert!(layer.find_row_start(3).is_err());
        assert!(layer.find_row_end(3).is_err());
        assert!(layer.find_row_bounds(3).is_err());
    }

    ///
    #[test] fn get_row_test() {

        // Test layer layout is [1, 2]
        let mut layer = test_layer();

        assert!(layer.get_row_ref(0).is_some());
        assert!(layer.get_row_ref(1).is_some());
        assert!(layer.get_row_ref(2).is_none());
        assert_eq!(layer.get_row_mut(1).expect("Row exists").len(), 2);
    }
}


