
/// Contains methods for adding blocks to a layer.
pub mod block;

/// Contains methods for adding rows to a layer.
pub mod row;

use crate::{ Block, Layer };

/// Retrieve the block index for the end of the previous row.
/// (This finds the preceding block, remember to add 1 in order to place the block after it.)
/// (This expects the current row and all previous rows exist, make sure to check first.)
fn previous_available_row_recursion_helper<B: Block>(
    layer: &Layer<B>,
    r: usize
) -> usize {
    // Can't look before the first row, so just return the start of the collection.
    if r == 0 { return 0 }

    let search = layer.find_row_end(r - 1)
        .expect("Previous row exists");

    match search {
        Some(index) => index,
        // Repeat for the previous row if this row is also empty.
        None => previous_available_row_recursion_helper(layer, r - 1),
    }
}

