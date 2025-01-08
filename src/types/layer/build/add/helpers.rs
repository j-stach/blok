
use crate::{ Block, Layer };

/// Retrieve the block index for the end of the previous row.
/// (This finds the preceding block, remember to add 1 in order to place the block after it.)
/// (This expects the current row and all previous rows exist, make sure to check first.)
pub(super) fn previous_available_row_recursion_helper<B: Block>(
    layer: &Layer<B>,
    r: usize
) -> usize {
    // Can't look before the first row, so just return the start of the collection.
    if r == 0 { return 0 } // DEBUG/NOTE: This is bad if the layer is empty...

    let search = layer.find_row_end(r - 1)
        .expect("Previous row exists");

    match search {
        Some(index) => index,
        // Repeat for the previous row if this row is also empty.
        None => previous_available_row_recursion_helper(layer, r - 1),
    }
}



/*  UNIT TESTS  */
#[cfg(test)] mod test {
    use crate::block::{ Block, test::TestBlock };
    use crate::types::layer::test::test_layer;

    /// 
    #[test] fn previous_available_row_recursion_test() {

        let mut layer = test_layer();
        layer.new_row();
        layer.add_blocks(vec![TestBlock::create(&"test".to_string()); 3]);

        // TBD: How best to test?
    }
}
