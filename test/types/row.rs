
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

}

#[test] fn row_partial_reference_test() {
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

