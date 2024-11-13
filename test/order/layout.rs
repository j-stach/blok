
use blok::{ Layout, layout };

/// Test the creation of new layouts via functions and the utility macro.
#[test] fn new_layout_test() {
    let layout1 = Layout::default();
    let layout2 = Layout::new();
    let layout3 = layout!();
    assert_eq!(layout1, layout2);
    assert_eq!(layout3, layout2);

    let layout1 = Layout::wrap(vec![1]);
    let layout2 = layout![1];
    assert_eq!(layout1, layout2);

    let layout1 = Layout::wrap(vec![1, 2, 3]);
    let layout2 = layout![1, 2, 3];
    assert_eq!(layout1, layout2);

    let layout1 = Layout::wrap(vec![0, 0, 0]);
    let layout2 = layout![0; 3];
    assert_eq!(layout1, layout2);
}

/// Test counting the number of blocks in a layout.
#[test] fn layout_total_test() {
    let layout = layout!();
    assert_eq!(layout.total(), 0);

    let layout = layout![1];
    assert_eq!(layout.total(), 1);

    let layout = layout![1, 2, 3];
    assert_eq!(layout.total(), 6);
}

/// Test finding the start and end of layout rows.
#[test] fn layout_bounds_test() {
    let layout = layout![1, 2, 3, 0, 5];
    let start = layout.row_start(0);
    let end = layout.row_end(0);
    assert_eq!((start, end), (Some(0), Some(0)));

    let start = layout.row_start(1);
    let end = layout.row_end(1);
    assert_eq!((start, end), (Some(1), Some(2)));

    let start = layout.row_start(2);
    let end = layout.row_end(2);
    assert_eq!((start, end), (Some(3), Some(5)));

    let start = layout.row_start(3);
    let end = layout.row_end(3);
    assert_eq!((start, end), (None, None));

    let start = layout.row_start(4);
    let end = layout.row_end(4);
    assert_eq!((start, end), (Some(6), Some(10)));
}

