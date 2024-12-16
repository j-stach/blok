
/// Methods for additively building row.
pub mod add;

/// Methods for rapidly adding populations of similar blocks.
pub mod populate;

/// Methods for using blocks that represent empty space.
pub mod void;

// Methods for changing a row's organization by manipulating clones.
// pub mod transform;

// Methods for combining rows.
// pub mod merge;


    // TODO! Needs `disconnect` method for Block trait

    ///// Remove the block at the given row, index.
    //pub fn remove_block(
    //    &mut self,
    //    r: usize,
    //    i: usize
    //) -> anyhow::Result<()> {
    //    // TODO
    //    todo!{}
    //}

    ///// Remove the row from the given row.
    //pub fn remove_row(&mut self, r: usize) -> anyhow::Result<()> {
    //    // TODO
    //    todo!{}
    //}
