
/// Contains methods for adding blocks to a stack.
pub mod block;

/// Contains methods for adding rows to a stack.
pub mod row;

/// Contains methods for adding layers to a stack.
pub mod layer;


use crate::{ Block, Stack };

/// Retrieve the block index for the end of the previous layer.
/// If there is no previous block, use 0 to place at layer start.
/// (This finds the preceding block, remember to add 1 in order to place the block after it.)
/// (This expects the current layer and all previous layers exist, make sure to check first.)
fn previous_available_layer_recursion_helper<B: Block>(
    stack: &Stack<B>,
    l: usize 
) -> usize {
    // Can't look before the first layer, so just return the start of the collection.
    if l == 0 { return 0 }

    let search = stack.find_layer_end(l - 1)
        .expect("Previous layer exists");

    match search {
        Some(index) => index,
        // Repeat for the previous layer if this layer is also empty.
        None => previous_available_layer_recursion_helper(stack, l - 1),
    }
}

/// Retrieve the block index for the end of the previous row.
/// If there is no previous block in the layer, uses the layer_recursion_helper.
/// (This finds the preceding block, remember to add 1 in order to place the block after it.)
/// (This expects the current layer and all previous layers exist, make sure to check first.)
fn previous_available_row_recursion_helper<B: Block>(
    stack: &Stack<B>,
    l: usize,
    r: usize
) -> usize {
    // Can't look before the first row, 
    if r == 0 {
        // so use the end of a previous layer instead.
        return previous_available_layer_recursion_helper(stack, l)
    }

    let search = stack.find_row_end(l, r - 1)
        .expect("Previous layer exists");

    match search {
        Some(index) => index,
        // Repeat for the previous row if this row is also empty.
        None => previous_available_row_recursion_helper(stack, l, r - 1),
    }
}


