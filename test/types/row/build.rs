
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
    assert_eq!(ids, vec!["test"; 4]);
    
    let blocks = row.clone_into_blocks();
    assert_eq!(blocks.len(), row.len());
    blocks.iter().for_each(|block| assert_eq!(&block.id, "test"));

    row.set_from_blocks(blocks);
    let other_ids: Vec<&str> = row.iter()
        .map(|block: &TestBlock1| block.id.as_str())
        .collect();
    assert_eq!(other_ids, vec!["test"; 4]);

}

#[test] fn row_voids_test() {
    let mut row = Row::default();
    row.add_block(TestBlock1::create(&"real".to_string()));

    row.offset(1);
    assert_eq!(row.len(), 2);
    assert!(
        row[0].is_void() && !row[1].is_void(), 
        "Offset should add a void at the beginning."
    );

    row.pad(1);
    assert_eq!(row.len(), 3);
    assert!(
        row[2].is_void() && !row[1].is_void(), 
        "Pad should add a void at the end."
    );

    row.offset(0);
    assert_eq!(row.len(), 3, "Offset 0 should add 0");
    row.offset(2);
    assert_eq!(row.len(), 5, "Offset 2 should add 2");

    row.pad(0);
    assert_eq!(row.len(), 5, "Pad 0 should add 0");
    row.pad(2);
    assert_eq!(row.len(), 7, "Pad 2 should add 2");

    let num_voids = row.iter()
        .filter(|block| block.is_void())
        .count();
    assert_eq!(num_voids, 6, "There should be 6 voids in total.");

    row.compress();
    assert_eq!(row.len(), 1, "There should be 6 blocks removed.");
    assert_eq!(
        &row[0].id, 
        "real", 
        "Compress must preserve real."
    );

    let num_voids = row.iter()
        .filter(|block| block.is_void())
        .count();
    assert_eq!(num_voids, 0, "Compress should remove voids.");

    row.offset(1).pad(1);
    row.fill_voids(&"filler".to_string());

    let num_voids = row.iter()
        .filter(|block| block.is_void())
        .count();
    assert_eq!(num_voids, 0, "There should be no voids unfilled.");
    assert_eq!(
        &row[0].id, 
        "filler", 
        "Offset should be replaced with filler."
    );
    assert_eq!(
        &row[2].id, 
        "filler", 
        "Pad should be replaced with filler."
    );

    row.offset(1).pad(1);
    row.fill_with_clones(&TestBlock1::create(&"more_filler".to_string()));

    let ids: Vec<&str> = row.iter()
        .map(|block| block.id.as_str())
        .collect();
    assert_eq!(ids, vec![
        "more_filler",
        "filler",
        "real",
        "filler",
        "more_filler",
    ]);


    // TODO Row voids?
}
