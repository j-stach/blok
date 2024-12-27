
use blok::{ Block, Node };

#[derive(
    Default, Clone,         // Necessary for Block impl
    Debug, Eq, PartialEq    // Nice to have for testing
)]
pub struct TestNode1 {
    pub id: String,
    pub connections: Vec<String>,
}

impl Block for TestNode1 {

    type CreationInstructions = String;

    fn create(id: &String) -> Self {
        TestNode1 {
            id: id.to_owned(),
            connections: Vec::new(),
        }
    }

    fn void() -> Self {
        Self::default()
    }

    fn is_void(&self) -> bool {
        self.id.is_empty()
    }

}


impl Node for TestNode1 {

    type ConnectionInstructions = u32;

    fn connect(
        &mut self, 
        other: &mut Self, 
        times: &u32
    ) {
        for _ in 0..*times {
            self.connections.push(other.id.clone())
        }
    }

}

/// Test using block connections.
#[test] fn connect_block_test() {
    let mut a = TestNode1::create(&"a".to_string());
    let mut b = TestNode1::create(&"b".to_string());

    a.connect(&mut b, &1);
    assert_eq!(&a.connections[0], &b.id);
    a.connect(&mut b, &2);
    assert_eq!(a.connections.len(), 3);
}

