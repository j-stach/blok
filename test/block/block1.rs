
use blok::Block;

#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct TestBlock1 {
    id: String,
    connections: Vec<String>,
}

impl Block for TestBlock1 {
    type CreationInstructions = String;
    type ConnectionInstructions = u32;

    fn create(id: &String) -> Self {
        TestBlock1 {
            id: id.to_owned(),
            connections: Vec::new(),
        }
    }

    fn connect(
        &mut self, 
        other: &mut Self, 
        times: &u32
    ) {
        for _ in 0..*times {
            self.connections.push(other.id.clone())
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
}

/// Test using block connections.
#[test] fn connect_block_test() {
    let mut a = TestBlock1::create(&"a".to_string());
    let mut b = TestBlock1::create(&"b".to_string());

    a.connect(&mut b, &1);
    assert_eq!(&a.connections[0], &b.id);
    a.connect(&mut b, &2);
    assert_eq!(a.connections.len(), 3);
}

/// Test using void blocks.
#[test] fn void_block_test() {
    let mut a = TestBlock1::create(&"NotVoid".to_string());
    let mut b = TestBlock1::void();
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


