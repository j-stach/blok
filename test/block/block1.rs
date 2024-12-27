
use blok::Block;

#[derive(
    Default, Clone,         // Necessary for Block impl
    Debug, Eq, PartialEq    // Nice to have for testing
)]
pub struct TestBlock1 {
    pub id: String,
}

impl Block for TestBlock1 {
    type CreationInstructions = String;

    fn create(id: &String) -> Self {
        TestBlock1 {
            id: id.to_owned(),
        }
    }

    fn void() -> Self {
        Self::default()
    }

    fn is_void(&self) -> bool {
        self.id.is_empty()
    }
}


/// Test for creating new blocks.
#[test] fn new_block_test() {
    let block1 = TestBlock1::create(&"block1".to_string());
    assert_eq!(block1.id, "block1");
}

/// Test using void blocks.
#[test] fn void_block_test() {
    let mut a = TestBlock1::create(&"NotVoid".to_string());
    let b = TestBlock1::void();
    assert!(b.is_void() && !a.is_void());

    a.to_void();
    assert_eq!(a, b);
}

/// Test modifying blocks in place.
#[test] fn modify_block_test() {
    let mut a = TestBlock1::create(&"NotVoid".to_string());
    let mut b = TestBlock1::void();

    a.shift(&mut b);
    assert_eq!("NotVoid", &b.id);

    a.overwrite(TestBlock1::create(&"NotVoidEither".to_string()));
    assert_eq!(&a.id, "NotVoidEither");

    a.shift(&mut b);
    assert_eq!(&a.id, "NotVoidEither");
    a.swap(&mut b);
    assert_eq!(&a.id, "NotVoid");
}


