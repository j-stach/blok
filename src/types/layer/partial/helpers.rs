
use crate::Layout;


/// Check whether the range falls within the total number of blocks.
pub(crate) fn range_boundary_check_helper(
    total: usize, 
    start: usize, 
    end: usize
) -> bool {

    if start > end || end >= total { false } 
    else { true }
}

/// Helps to organize blocks using the layout, 
/// so that reference reflects layer structure.
/// Returns an empty Vec when the layer is empty.
pub(crate) fn collection_organization_helper<T>(
    layout: &Layout,
    mut blocks_ref: Vec<T>,
) -> Vec<Vec<T>> {

    let mut layer_ref = Vec::new();

    for r in layout.iter() {
        let tail = blocks_ref.split_off(*r); // DEBUG?
        layer_ref.push(blocks_ref);
        blocks_ref = tail;
    }
    
    layer_ref
}



/*  UNIT TESTS  */
#[cfg(test)] mod test {

    use super::*;
    use crate::{ Block, Layer, Layout };
    use crate::types::layer::test::test_layer;

    /// Test for verifying a range of blocks in a layer.
    #[test] fn range_boundary_check_test() {

        let layer = test_layer();

        // Within range should return true.
        let range_good = range_boundary_check_helper(
            layer.layout.total(),
            0, 2
        );

        // Out of range should return false.
        let range_bad = range_boundary_check_helper(
            layer.layout.total(),
            0, 3
        );

        assert_eq!(range_good, true);
        assert_eq!(range_bad, false);

    }

    /// Test for organizing collections using layout for structure.
    #[test] fn collection_organization_test() {

        let layer = test_layer();

        let organized_collection = collection_organization_helper(
            &layer.layout(),
            layer.blocks().iter().collect()
        );

        // Should mirror the structure of the test layer (1, 2):
        assert_eq!(organized_collection.len(), 2);
        assert_eq!(organized_collection[0].len(), 1);
        assert_eq!(organized_collection[1].len(), 2);
    }

}

