
use blok::{ Block, Row };
use crate::block::TestBlock1;

/// Test row creation.
#[test] fn new_row_test() {
    let block = TestBlock1::create(&"test".to_string());
    let blocks = vec![block.clone(); 3];
    let row = Row::wrap(blocks);
    assert_eq!(row.len(), 3);

    let num_blocks = row.blocks().len();
    assert_eq!(num_blocks, 3);

    let first_block = &row.blocks()[0];
    assert_eq!(&first_block.id, "test");

    let block_id = &row[2].id;
    assert_eq!(block_id, &block.id);
}

/// Test functional building with rows.
#[test] fn build_row_test() {

    let test_block = |id: &str| TestBlock1::create(&id.to_string());
    let mut row = Row::default();
    assert_eq!(row.len(), 0);

    row.add_block(test_block("1"));
    assert_eq!(row.len(), 1);
    assert_eq!(&row[0].id, "1");

    row.insert_block(0, test_block("0"))
        .expect("Insert block at index 0");
    assert_eq!(row.len(), 2);
    assert_eq!(&row[0].id, "0");
    
    row.add_blocks(vec![test_block("2"), test_block("3")]);
    assert_eq!(row.len(), 4);
    assert_eq!(&row[2].id, "2");
    assert_eq!(&row[3].id, "3");

    row.insert_blocks(0, vec![test_block("-2"), test_block("-1")])
        .expect("Insert two blocks at index 0");
    assert_eq!(row.len(), 6);

    let ids: Vec<&str> = row.iter()
        .map(|block| block.id.as_str())
        .collect();
    assert_eq!(ids, vec!["-2", "-1", "0", "1", "2", "3"]);

    //

    let mut other_row = Row::default();
    other_row.add_block(test_block("1"))
        .insert_block(0, test_block("0"))
            .expect("Insert block at index 0")
        .add_blocks(vec![test_block("2"), test_block("3")])
        .insert_blocks(0, vec![test_block("-2"), test_block("-1")])
            .expect("Insert two blocks at index 0");

    let other_ids: Vec<&str> = row.iter()
        .map(|block| block.id.as_str())
        .collect();
    assert_eq!(other_ids, vec!["-2", "-1", "0", "1", "2", "3"]);

    //

    let mut empty_row = Row::default();
    assert!(empty_row.insert_block(1, test_block("error")).is_err());
    assert!(empty_row.insert_blocks(1, vec![
        test_block("miss"),
        test_block("miss"),
        test_block("miss"),
    ]).is_err());
    assert_eq!(empty_row.len(), 0);
}

/// Test row auto-population.
#[test] fn populate_row_test() {
    let mut row = Row::default();
    row.populate(4, &"test".to_string());

    let test_block = TestBlock1::create(&"test".to_string());
    let mut other_row = Row::default();
    other_row.populate_with_clones(4, &test_block);

    let ids: Vec<&str> = row.iter()
        .map(|block: &TestBlock1| block.id.as_str())
        .collect();
    let other_ids: Vec<&str> = row.iter()
        .map(|block| block.id.as_str())
        .collect();

    assert_eq!(row.len(), other_row.len());
    assert_eq!(ids, other_ids);
}

#[test] fn clone_row_test() {
    let mut row = Row::default();
    row.populate(4, &"test".to_string());
    let ids: Vec<&str> = row.iter()
        .map(|block: &TestBlock1| block.id.as_str())
        .collect();
    
    let blocks = row.clone_into_blocks();
    assert_eq!(blocks.len(), row.len());
    blocks.iter().for_each(|block| assert_eq!(&block.id, "test"));

    row.set_from_blocks(blocks);
    let other_ids: Vec<&str> = row.iter()
        .map(|block: &TestBlock1| block.id.as_str())
        .collect();

    assert_eq!(ids, other_ids);
}
