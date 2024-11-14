
use blok::{ Block, Row };
use crate::block::TestBlock1;


#[test] fn partial_reference_test() {
    let test_block = |id: &str| TestBlock1::create(&id.to_string());
    let mut row = Row::wrap(vec![
        test_block("0"),
        test_block("1"),
        test_block("2"),
        test_block("3"),
        test_block("4"),
    ]);

    // block/range ref/mut 
    let block_ref = row.get_block_ref(0)
        .expect("Get block ref");
    assert_eq!(&block_ref.id, "0");

    let block_mut = row.get_block_mut(1)
        .expect("Get block mut");
    assert_eq!(&block_mut.id, "1");

    block_mut.id = "0".to_string();
    assert_eq!(&block_mut.id, "0");

    let range_mut = row.get_range_mut(2, 4)
        .expect("Get range mut");
    range_mut.into_iter()
        .for_each(|block_mut| block_mut.id = "0".to_string());

    let range_ref = row.get_range_ref(0, 4)
        .expect("Get range ref");
    range_ref.into_iter()
        .for_each(|block_ref| assert_eq!(&block_ref.id, "0"));

    assert!(row.get_range_ref(0, 0).is_some());
    assert!(row.get_range_mut(0, 0).is_some());
    assert_eq!(row.get_range_ref(0, 0).unwrap().len(), 1);

    assert!(row.get_block_ref(5).is_none());
    assert!(row.get_block_mut(5).is_none());

    assert!(row.get_range_ref(4, 5).is_none());
    assert!(row.get_range_mut(4, 5).is_none());
    assert!(row.get_range_ref(4, 3).is_none());
    assert!(row.get_range_mut(4, 3).is_none());
    assert!(row.get_range_ref(3, 5).is_none());
    assert!(row.get_range_mut(3, 5).is_none());
    assert!(row.get_range_ref(5, 6).is_none());
    assert!(row.get_range_mut(5, 6).is_none());
}

